use core::fmt;
use std::path::Path;

use image::image_dimensions;
use indoc::formatdoc;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};
use tracing::{debug, error};

use crate::picture_generator::{
    picture_markup_generator::{generate_picture_markup, should_swap_dimensions},
    resolutions::get_max_resolution,
};

pub const MAX_BLOG_IMAGE_RESOLUTION: (u32, u32) = (1280, 860);

/// A heading extracted from a markdown document, for building tables of contents.
#[derive(Clone, Debug)]
pub struct HeadingToc {
    pub level: u8,
    pub id: String,
    pub text: String,
}

/// Slugify heading text exactly the way `parse_markdown` does when it injects
/// heading ids, so anchors and TOC links always agree.
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "-")
}

/// Extract h2/h3 headings (level, anchor id, text) from a markdown document.
/// Mirrors the id-generation logic inside `parse_markdown`.
pub fn extract_headings(markdown: &str) -> Vec<HeadingToc> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let mut headings: Vec<HeadingToc> = Vec::new();
    // (level, provided id, first raw Text event, collected display text)
    let mut current: Option<(u8, Option<String>, Option<String>, String)> = None;

    for event in Parser::new_ext(markdown, options) {
        match event {
            Event::Start(Tag::Heading { level, id, .. }) => {
                current = Some((level as u8, id.map(|i| i.to_string()), None, String::new()));
            }
            Event::Text(text) => {
                if let Some((_, _, first_text, collected)) = current.as_mut() {
                    if first_text.is_none() {
                        *first_text = Some(text.to_string());
                    }
                    collected.push_str(&text);
                }
            }
            Event::Code(text) => {
                if let Some((_, _, _, collected)) = current.as_mut() {
                    collected.push_str(&text);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((level, provided_id, first_text, collected)) = current.take() {
                    if level == 2 || level == 3 {
                        // `parse_markdown` slugs the first raw Text event verbatim
                        let id = provided_id.unwrap_or_else(|| slugify(&first_text.unwrap_or_default()));
                        headings.push(HeadingToc {
                            level,
                            id,
                            text: collected.trim().to_string(),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    headings
}

enum TextKind {
    Text,
    Heading(Option<String>),
    Code(String),
}

/// A raw-HTML chunk that opens one of the block-level elements the image
/// mapping below generates. Matched on the `formatdoc!` prefixes we emit
/// ourselves — pulldown's own output never starts a block with `Event::Html`
/// inside a paragraph.
fn is_figure_start(event: &Event<'_>) -> bool {
    matches!(event, Event::Html(html) if html.trim_start().starts_with("<figure"))
}

/// The exact closing chunk the `Event::End(TagEnd::Image)` arm emits.
fn is_figure_end(event: &Event<'_>) -> bool {
    matches!(event, Event::Html(html) if html.trim() == "</figcaption></figure>")
}

/// Events that carry real inline paragraph content — i.e. should produce
/// (or reopen) a `<p>`. Whitespace-only text, line breaks and empty chunks
/// are excluded: directly after a figure they would materialize an empty
/// `<p>` where browsers today silently drop the auto-closed one. Non-figure
/// raw HTML (`Event::Html` — e.g. the bare external-URL `<img>`) counts as
/// content: inside a paragraph, source-level block HTML never arrives as
/// `Event::Html` (raw blocks are emitted outside paragraphs), so these are
/// always our own inline-capable mappings. Figure start/end chunks are
/// intercepted by their own match arms before this is consulted.
/// `Start(_)` covers the inline tags that can open inside a paragraph
/// (Link, Emphasis, Strong, …).
fn is_inline_content(event: &Event<'_>) -> bool {
    match event {
        Event::Text(text) => !text.chars().all(char::is_whitespace),
        Event::Html(html) => !html.is_empty(),
        Event::Code(_)
        | Event::InlineHtml(_)
        | Event::FootnoteReference(_)
        | Event::TaskListMarker(_)
        | Event::InlineMath(_)
        | Event::Start(_) => true,
        _ => false,
    }
}

/// Streaming `<p>` gate for `parse_markdown` (specs/w3c-validation.md S8).
///
/// pulldown-cmark wraps `![alt](url)` in `Start(Paragraph)` … `End(Paragraph)`
/// while the image mapping renders the image as a block-level `<figure>`;
/// pushing both emits `<p><figure>…</figure></p>`. That is invalid HTML5 —
/// parsers implicitly close the `<p>` when the figure starts, so the
/// trailing `</p>` is stray (the W3C feed validator reports it as `NotHtml`
/// for every affected item; the Nu checker reports "No p element in scope
/// but a p end tag seen").
///
/// The gate defers `<p>` until inline content actually arrives, closes it
/// before a figure starts mid-paragraph, and reopens a fresh `<p>` for
/// trailing inline content — so an image-only paragraph emits a bare
/// `<figure>` at block level, and `text ![img] tail` splits into two
/// paragraphs around it (figures render block-level anyway, so the visual
/// result is unchanged).
///
/// Known limitation, matching today's content (every post image is a
/// standalone paragraph): an image nested inside an *open* inline element
/// (link/emphasis) would close the `<p>` while that element is still open.
struct BlockFigureParagraphGate<'a, I> {
    inner: I,
    queue: std::collections::VecDeque<Event<'a>>,
    /// Between `Start(Paragraph)` and `End(Paragraph)`.
    in_paragraph: bool,
    /// The `<p>` start was actually emitted (and not yet closed).
    paragraph_open: bool,
    /// Between our figure-start and figure-end chunks (figcaption text).
    in_figure: bool,
}

impl<'a, I: Iterator<Item = Event<'a>>> BlockFigureParagraphGate<'a, I> {
    fn new(inner: I) -> Self {
        Self {
            inner,
            queue: std::collections::VecDeque::new(),
            in_paragraph: false,
            paragraph_open: false,
            in_figure: false,
        }
    }
}

impl<'a, I: Iterator<Item = Event<'a>>> Iterator for BlockFigureParagraphGate<'a, I> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Event<'a>> {
        loop {
            if let Some(event) = self.queue.pop_front() {
                return Some(event);
            }
            match self.inner.next()? {
                // Defer the `<p>`: emit it only when inline content shows up.
                // An image-only paragraph therefore never materializes one.
                Event::Start(Tag::Paragraph) => {
                    self.in_paragraph = true;
                    self.paragraph_open = false;
                    // Defensive: a paragraph start invalidates any stale
                    // figure span (ours always close before End(Paragraph)).
                    self.in_figure = false;
                }
                Event::End(TagEnd::Paragraph) => {
                    let close = self.paragraph_open;
                    self.in_paragraph = false;
                    self.paragraph_open = false;
                    if close {
                        return Some(Event::End(TagEnd::Paragraph));
                    }
                }
                event if is_figure_start(&event) => {
                    // Close an open paragraph before the block element; a
                    // fresh `<p>` reopens if trailing inline content follows.
                    //
                    // The figure span is only tracked while INSIDE a
                    // paragraph: an `Event::Html` there can only be one of our
                    // generated figures (source-level raw HTML blocks are
                    // emitted block-level, never wrapped in Paragraph
                    // events, and inline raw HTML uses InlineHtml). A
                    // hand-written `<figure>…</figure>` block in a post must
                    // NOT set the flag — its plain `</figure>` closer never
                    // matches ours, so the flag would stick and swallow the
                    // `<p>` of every following paragraph.
                    if self.in_paragraph {
                        if self.paragraph_open {
                            self.paragraph_open = false;
                            self.queue.push_back(Event::Html("</p>".into()));
                        }
                        // Figcaption content passes through untouched (alt
                        // text must not reopen a paragraph inside the figure).
                        self.in_figure = true;
                    }
                    self.queue.push_back(event);
                }
                event if is_figure_end(&event) => {
                    self.in_figure = false;
                    self.queue.push_back(event);
                }
                event => {
                    if self.in_figure || !self.in_paragraph || self.paragraph_open {
                        return Some(event);
                    }
                    if is_inline_content(&event) {
                        self.paragraph_open = true;
                        self.queue.push_back(Event::Start(Tag::Paragraph));
                        self.queue.push_back(event);
                    }
                    // else: whitespace/line break with no open `<p>` — drop it
                    // (would render an empty paragraph browsers don't show today).
                }
            }
        }
    }
}

/// Escape a value interpolated into a double-quoted HTML attribute in the
/// hand-built `formatdoc!` tags below. pulldown-cmark escapes text *content*,
/// but these attribute slots take raw URLs/titles — an unescaped `&` (e.g.
/// `?video=1&parent=x`) is invalid HTML and surfaces as the feed
/// validator's NotHtml / lxml's htmlParseEntityRef.
fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
}

// pub fn parse_markdown(markdown: &str) -> ::askama::Result<String>
#[askama::filter_fn]
pub fn parse_markdown<T: fmt::Display>(
    markdown: T,
    _: &dyn askama::Values,
) -> ::askama::Result<String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let mut text_kind = TextKind::Text;
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let theme = theme_set.themes.get("base16-ocean.dark").unwrap();
    let mut heading_ended: Option<bool> = None;

    let mds = markdown.to_string();
    let parser = Parser::new_ext(&mds, options).map(|event| match event {
        /*
        Parsing images considers `alt` attribute as inner `Text` event
        Therefore the `[alt]` is rendered in html as subtitle
        and the `[](url "title")` `title` is rendered as `alt` attribute
        */
        Event::Start(Tag::Image {
            link_type: _,
            dest_url,
            title,
            id: _,
        }) => {
            // External URLs and SVG files: no local file to probe for
            // dimensions (image crate can't read SVGs, external URLs aren't
            // on disk) — render the simple figure directly. The alt text
            // then flows into the <figcaption> via the inner Text events,
            // exactly like local raster images: image + visible caption.
            if !dest_url.starts_with("/") || dest_url.to_lowercase().ends_with(".svg") {
                return Event::Html(
                    formatdoc!(
                        r#"<figure>
                            <img src="{src}" alt="{alt}">
                            <figcaption>
                        "#,
                        src = escape_attr(&dest_url),
                        alt = escape_attr(&title),
                    )
                    .into(),
                );
            }

            let dev_only_img_path =
                Path::new("static/").join(dest_url.strip_prefix("/").unwrap_or(&dest_url));

            // We need to take the exif rotation into consideration here
            let img_dimensions = {
                let orig_img_dimensions = image_dimensions(&dev_only_img_path).unwrap();
                if should_swap_dimensions(&dev_only_img_path) {
                    (orig_img_dimensions.1, orig_img_dimensions.0)
                } else {
                    orig_img_dimensions
                }
            };
            let (max_width, max_height) = get_max_resolution(
                img_dimensions,
                MAX_BLOG_IMAGE_RESOLUTION.0,
                MAX_BLOG_IMAGE_RESOLUTION.1,
            );

            // Place image into the content with scaled reso to a boundary
            let picture_markup = generate_picture_markup(
                &dest_url, max_width, max_height, &title, None, false,
            )
            .unwrap_or(formatdoc!(
                r#"
                        <img
                          alt="{alt}"
                          src="{src}"
                        />"#,
                alt = title,
                src = dest_url,
            ));
            Event::Html(
                formatdoc!(
                    r#"<figure>
                        {picture_markup}
                        <figcaption>
                    "#,
                )
                .into(),
            )
        }
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
            text_kind = TextKind::Code(lang.to_string());
            // Own the whole card: skip pulldown's <pre><code> wrapper and emit
            // a .code-card div holding the copy button + the syntect <pre>.
            Event::Html(
                formatdoc!(
                    r#"<div class="code-card"><button type="button" class="code-copy" aria-label="Copy code to clipboard">copy</button>"#
                )
                .into(),
            )
        }
        Event::Text(text) => match &text_kind {
            TextKind::Code(lang) => {
                // TODO Check https://github.com/trishume/syntect/pull/535 for typescript support
                let lang = if ["ts".to_string(), "typescript".to_string()].contains(lang) {
                    "javascript"
                } else {
                    lang
                };
                let syntax_reference = syntax_set
                    .find_syntax_by_token(lang)
                    .unwrap_or(syntax_set.find_syntax_plain_text());
                let highlighted =
                    highlighted_html_for_string(&text, &syntax_set, syntax_reference, theme)
                        .unwrap();
                Event::Html(highlighted.into())
            }
            TextKind::Heading(provided_id) => {
                let heading_id = provided_id.clone().unwrap_or({
                    text.to_lowercase()
                        .replace(|c: char| !c.is_alphanumeric(), "-")
                });
                debug!("heading_id: {}", heading_id.clone());
                match heading_ended {
                    None => {
                        error!("Heading should have set state");
                        panic!("Heading should have set state");
                    }
                    Some(true) => Event::Html(text),
                    Some(false) => {
                        heading_ended = Some(true);
                        Event::Html(
                            formatdoc!(
                                r##"id="{heading_id}">
                            {text}"##
                            )
                            .into(),
                        )
                    }
                }
            }
            _ => Event::Text(text),
        },
        Event::Start(Tag::Heading {
            level,
            id,
            classes: _,
            attrs: _,
        }) => {
            let id_str = id.map(|id| id.to_string());
            debug!("heading_start: {:?}, level: {}", &id_str, level);
            text_kind = TextKind::Heading(id_str);
            heading_ended = Some(false);
            Event::Html(format!("<{level} ").into())
        }
        Event::Start(_) => event,
        // Every image path (external, svg, local raster) opens a figure —
        // so the closer is unconditional again.
        Event::End(TagEnd::Image) => Event::Html("</figcaption></figure>".into()),
        Event::End(TagEnd::CodeBlock) => {
            // Fenced blocks were re-wrapped into .code-card (see Start above);
            // indented blocks still use pulldown's default <pre><code>.
            let fenced = matches!(text_kind, TextKind::Code(_));
            text_kind = TextKind::Text;
            if fenced {
                Event::Html("</div>".into())
            } else {
                Event::End(TagEnd::CodeBlock)
            }
        }
        Event::End(TagEnd::Heading(heading_level)) => {
            text_kind = TextKind::Text;
            heading_ended = None;
            Event::End(TagEnd::Heading(heading_level))
        }
        _ => event,
    });

    // Write to String buffer. The gate keeps generated `<figure>` blocks out
    // of the implicit `<p>` wrapper (invalid HTML5 nesting, S8).
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, BlockFigureParagraphGate::new(parser));
    Ok(html)
}

/// True when a block-level element (`<figure`, `<div`) appears between a
/// `<p>` start and its matching `</p>` — the invalid nesting the W3C feed
/// validator reports as `NotHtml` and Nu as a stray `</p>` (S8). Also used
/// by the feed tests to audit every rendered post body.
#[cfg(test)]
pub(crate) fn paragraph_nests_block(html: &str) -> bool {
    let mut in_paragraph = false;
    let mut rest = html;
    while let Some(rel) = rest.find('<') {
        let at = rel;
        let tail = &rest[at..];
        if tail.starts_with("<p>") || tail.starts_with("<p ") {
            in_paragraph = true;
        } else if tail.starts_with("</p>") {
            in_paragraph = false;
        } else if in_paragraph && (tail.starts_with("<figure") || tail.starts_with("<div")) {
            return true;
        }
        rest = &rest[at + 1..];
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    struct NoValues;
    impl askama::Values for NoValues {
        fn get_value<'a>(&'a self, _key: &str) -> Option<&'a dyn std::any::Any> {
            None
        }
    }

    fn render(md: &str) -> String {
        parse_markdown::default()
            .execute(md, &NoValues)
            .expect("markdown renders")
    }

    #[test]
    fn image_only_paragraph_renders_bare_figure() {
        // SVG path: no dimension probing, figure emitted directly (S8).
        let html = render("![Pi agent logo](/images/uploads/pi-logo.svg 'Pi agent logo')");
        assert!(html.contains("<figure"), "figure rendered: {html}");
        assert!(!html.contains("<p"), "no paragraph around image-only paragraph: {html}");
        assert!(!paragraph_nests_block(&html));
    }

    #[test]
    fn text_around_image_splits_the_paragraph() {
        let html = render("before ![Pi agent logo](/images/uploads/pi-logo.svg) after");
        assert!(!paragraph_nests_block(&html), "figure must not nest in <p>: {html}");
        assert!(html.contains("<p>before"), "leading text keeps its paragraph: {html}");
        assert!(html.contains("<p>"), "trailing text reopens a paragraph: {html}");
        assert_eq!(
            html.matches("<p>").count(),
            html.matches("</p>").count(),
            "paragraph tags stay balanced: {html}"
        );
    }

    #[test]
    fn consecutive_image_paragraphs_stay_bare() {
        let html = render(
            "![one](/images/uploads/pi-logo.svg)\n\n![two](/images/uploads/pi-logo.svg)",
        );
        assert_eq!(html.matches("<figure").count(), 2, "{html}");
        assert!(!html.contains("<p"), "{html}");
    }

    #[test]
    fn fenced_code_is_never_wrapped_in_paragraph() {
        let html = render("intro text\n\n```rust\nfn main() {}\n```\n\noutro text");
        assert!(html.contains("<p>intro text</p>"), "{html}");
        assert!(html.contains("code-card"), "{html}");
        assert!(html.contains("<p>outro text</p>"), "{html}");
        assert!(!paragraph_nests_block(&html), "{html}");
    }

    #[test]
    fn external_image_gets_a_figure_with_visible_caption() {
        // Author intent: external images show the alt text as a visible
        // caption, same as local images (title in the alt attribute, alt
        // text inside <figcaption>). The pre-S8 workaround emitted the
        // caption text after a bare <img> with figure *closers* only —
        // now the opener matches (F13).
        let html = render(
            "![Preview of headphones](https://cdn.example.net/x.jpeg 'Preview of headphones')",
        );
        assert!(html.contains("<figure"), "external images open a figure: {html}");
        assert!(
            html.contains(r#"src="https://cdn.example.net/x.jpeg""#),
            "{html}"
        );
        assert!(
            html.contains(r#"alt="Preview of headphones""#),
            "title renders as the alt attribute: {html}"
        );
        assert!(
            html.contains("<figcaption>"),
            "caption text is visible in a figcaption: {html}"
        );
        assert!(html.contains("</figcaption></figure>"), "{html}");
        assert!(!paragraph_nests_block(&html));
    }

    #[test]
    fn external_image_between_text_splits_the_paragraph() {
        // The external image is a block-level figure now — same gating as
        // local images: leading text keeps its <p>, trailing text reopens one.
        let html = render(
            "see [the shop](https://example.com) and ![pic](https://cdn.example.net/y.png) here",
        );
        assert!(html.contains("<a href=\"https://example.com\">the shop</a>"), "{html}");
        assert!(html.contains("<figure"), "{html}");
        assert!(!paragraph_nests_block(&html), "{html}");
        assert_eq!(html.matches("<p>").count(), html.matches("</p>").count());
        assert!(html.matches("<p>").count() >= 1, "{html}");
    }

    #[test]
    fn external_image_escapes_ampersands_in_attributes() {
        // Real shape from 2020-08-05-webassembly-briefing: query params with
        // raw `&` must be escaped in the attribute (feed validator NotHtml).
        let html = render("![Blazor](https://cdn.example.net/iu/?u=x%2Fy&f=1&nofb=1)");
        assert!(
            html.contains(r#"src="https://cdn.example.net/iu/?u=x%2Fy&amp;f=1&amp;nofb=1""#),
            "{html}"
        );
        assert!(!html.contains("&f=1"), "no raw ampersand may survive: {html}");
    }

    #[test]
    fn raw_source_figure_block_does_not_break_following_paragraphs() {
        // Real corpus shape (2026-04-01-week-with-my-pi-agent): a hand-written
        // <figure> HTML block. It must pass through verbatim WITHOUT being
        // mistaken for a generated figure span — its plain </figure> closer
        // would never match ours, sticking the gate's in_figure flag and
        // stripping <p> from every subsequent paragraph.
        let md = "intro text\n\n<figure>\n  <img src=\"/x.svg\" alt=\"x\" class=\"w-4\">\n</figure>\n\nPi itself is very simple.\n\nMore paragraph text.\n\n![captioned](/images/uploads/pi-logo.svg 'cap')\n\ntail text";
        let html = render(md);
        assert!(html.contains("<figure>\n  <img src=\"/x.svg\""), "raw block verbatim: {html}");
        assert!(html.contains("<p>intro text</p>"), "{html}");
        assert!(
            html.contains("<p>Pi itself is very simple.</p>"),
            "paragraph after raw figure keeps its <p>: {html}"
        );
        assert!(
            html.contains("<p>More paragraph text.</p>"),
            "{html}"
        );
        // …and the gate still works for generated figures after the raw block:
        assert!(!paragraph_nests_block(&html), "{html}");
        assert!(html.contains("<figcaption>"), "{html}");
    }

    #[test]
    fn plain_paragraphs_and_inline_markup_are_untouched() {
        let html = render("just text\n\nmore **bold** and [a link](/x) here");
        assert!(html.contains("<p>just text</p>"), "{html}");
        assert!(html.contains("<strong>bold</strong>"), "{html}");
        assert!(html.contains("<a href=\"/x\">a link</a>"), "{html}");
    }

    #[test]
    fn paragraph_nests_block_detects_the_invalid_shape() {
        assert!(paragraph_nests_block("<p>text <figure><img src=\"x\"></figure></p>"));
        assert!(paragraph_nests_block(
            "<p><div class=\"code-card\">x</div></p>"
        ));
        assert!(!paragraph_nests_block("<p>ok</p><figure><img src=\"x\"></figure>"));
        assert!(!paragraph_nests_block("<p>a</p><p><em>b</em></p>"));
    }
}
