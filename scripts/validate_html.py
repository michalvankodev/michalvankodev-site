#!/usr/bin/env python3
"""Validate HTML documents against the W3C Nu Html Checker.

Each document is POSTed directly to https://validator.w3.org/nu/?out=json,
so localhost/dev output works without a publicly reachable URL.

Nu's embedded CSS checker lags modern CSS (Tailwind v4's
view-transition-name and friends are unknown to it), so error messages
whose text starts with "CSS:" are dropped by default. Pass --keep-css to
see everything.

Usage:
    validate_html.py SOURCE [SOURCE ...] [--keep-css]

SOURCEs are file paths or http(s) URLs.

Exit codes: 0 = all documents valid, 1 = any errors, 2 = transport/parse
failure.
"""

from __future__ import annotations

import argparse
import json
import sys
import urllib.request

SERVICE = "https://validator.w3.org/nu/?out=json"
TIMEOUT_S = 180
UA = "michalvankodev-site validate (just validate-html)"


def load_source(source: str) -> bytes:
    if source.startswith(("http://", "https://")):
        req = urllib.request.Request(source, headers={"User-Agent": UA})
        with urllib.request.urlopen(req, timeout=TIMEOUT_S) as resp:
            return resp.read()
    with open(source, "rb") as f:
        return f.read()


def trim_after_html(content: bytes) -> tuple[bytes, int]:
    """Drop anything after the first </html>.

    The dev server's livereload middleware appends its script after </html>,
    which is invalid HTML by construction (prod/SSG output never has it).
    Nothing valid ever follows </html>, so trimming normalizes dev output to
    what production serves. Returns (content, bytes_trimmed).
    """
    marker = b"</html>"
    idx = content.find(marker)
    if idx == -1:
        return content, 0
    end = idx + len(marker)
    rest = content[end:]
    if not rest.strip():
        return content, 0
    return content[:end], len(rest)


def validate(content: bytes) -> list[dict]:
    req = urllib.request.Request(
        SERVICE,
        data=content,
        headers={"User-Agent": UA, "Content-Type": "text/html; charset=utf-8"},
    )
    with urllib.request.urlopen(req, timeout=TIMEOUT_S) as resp:
        body = resp.read()
    try:
        parsed = json.loads(body)
    except json.JSONDecodeError as exc:
        sys.stderr.write(f"!! service returned non-JSON output: {exc}\n{body[:400]!r}\n")
        sys.exit(2)
    return parsed.get("messages", [])


def report(source: str, messages: list[dict], keep_css: bool) -> tuple[int, bool]:
    """Print one document's report; return (error_count, fatal)."""
    fatal = [m for m in messages if m.get("type") == "non-document-error"]
    if fatal:
        for m in fatal:
            print(f"  !! service could not process the document: {m.get('message', '')}")
        return 0, True

    def is_css(m: dict) -> bool:
        return m.get("message", "").startswith("CSS:")

    errors = [m for m in messages if m.get("type") == "error" and (keep_css or not is_css(m))]
    css_hidden = len([m for m in messages if m.get("type") == "error" and is_css(m) and not keep_css])
    infos = [m for m in messages if m.get("type") not in ("error", "non-document-error")]

    status = "INVALID" if errors else "valid"
    print(f"== {source} — {len(errors)} error(s), {len(infos)} info {status}")
    for m in errors:
        line = f"L{m['lastLine']}: " if m.get("lastLine") else ""
        print(f"   {line}{m.get('message', '')}")
    if css_hidden:
        print(f"   ({css_hidden} CSS-checker error(s) hidden — Nu's CSS knowledge lags "
              f"Tailwind v4; --keep-css to see them)")
    return len(errors), False


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("sources", nargs="+", help="HTML file paths or http(s) URLs")
    parser.add_argument("--keep-css", action="store_true", help="include CSS-checker messages")
    parser.add_argument("--no-trim", action="store_true",
                        help="do not trim content after </html> (livereload injection)")
    args = parser.parse_args()

    total_errors = 0
    fatal = False
    for source in args.sources:
        try:
            content = load_source(source)
        except (OSError, urllib.error.URLError) as exc:
            sys.stderr.write(f"!! cannot load {source}: {exc}\n")
            sys.exit(2)
        if not args.no_trim:
            content, trimmed = trim_after_html(content)
            if trimmed:
                print(f"   (trimmed {trimmed} bytes after </html> — dev livereload "
                      f"injection; pass --no-trim to disable)")
        try:
            messages = validate(content)
        except urllib.error.URLError as exc:
            sys.stderr.write(f"!! validator request failed for {source}: {exc}\n")
            sys.exit(2)
        errors, doc_fatal = report(source, messages, args.keep_css)
        total_errors += errors
        fatal = fatal or doc_fatal

    if fatal:
        sys.exit(2)
    print(f"\n{len(args.sources)} document(s) checked — {total_errors} error(s) total")
    sys.exit(1 if total_errors else 0)


if __name__ == "__main__":
    main()
