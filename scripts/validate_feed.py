#!/usr/bin/env python3
"""Validate an RSS/Atom feed against the W3C Feed Validation Service.

The feed content is POSTed directly to the service (rawdata mode), so no
publicly reachable URL is needed — localhost/dev output works — and the
result is always fresh. (The url= interface, by contrast, requires a public
URL and caches aggressively: an identical URL can serve a stale verdict
long after the feed changed.)

Usage:
    validate_feed.py SOURCE [--strict] [--allow TYPE,...] [--keep-css n/a]

SOURCE is a file path or an http(s) URL.

Exit codes: 0 = valid, 1 = invalid (errors), 2 = transport/parse failure.
--strict also fails on warnings (after the allowlist).
"""

from __future__ import annotations

import argparse
import sys
import urllib.parse
import urllib.request
import xml.etree.ElementTree as ET

SERVICE = "https://validator.w3.org/feed/check.cgi"
TIMEOUT_S = 180
UA = "michalvankodev-site validate (just validate-feed)"

# rawdata submissions carry no retrieval URI, so the atom:link self vs
# document-location comparison cannot be meaningful — ignore it unless
# explicitly kept via --allow "".
DEFAULT_ALLOW = ("SelfDoesntMatchLocation",)


def local_name(tag: str) -> str:
    return tag.rsplit("}", 1)[-1]


def load_source(source: str) -> bytes:
    if source.startswith(("http://", "https://")):
        req = urllib.request.Request(source, headers={"User-Agent": UA})
        with urllib.request.urlopen(req, timeout=TIMEOUT_S) as resp:
            return resp.read()
    with open(source, "rb") as f:
        return f.read()


def validate(content: bytes) -> dict:
    """POST the feed to the service, return {'validity', 'errors', 'warnings'}.

    Each error/warning is a dict of the SOAP entry fields (type, line, text,
    msgcount, element, parent, ...). The service already aggregates findings
    per type — msgcount is the number of occurrences.
    """
    data = urllib.parse.urlencode(
        {"rawdata": content.decode("utf-8", "replace"), "output": "soap12"}
    ).encode()
    req = urllib.request.Request(SERVICE, data=data, headers={"User-Agent": UA})
    with urllib.request.urlopen(req, timeout=TIMEOUT_S) as resp:
        body = resp.read()

    try:
        root = ET.fromstring(body)
    except ET.ParseError as exc:
        sys.stderr.write(f"!! service returned non-SOAP output: {exc}\n{body[:400]!r}\n")
        sys.exit(2)

    findings: dict = {"errors": [], "warnings": []}
    validity = None
    for element in root.iter():
        name = local_name(element.tag)
        if name == "validity":
            validity = (element.text or "").strip()
        elif name in ("error", "warning"):
            fields = {local_name(child.tag): (child.text or "") for child in element}
            findings[f"{name}s"].append(fields)
    findings["validity"] = validity == "true" and not findings["errors"]
    return findings


def report(source: str, findings: dict, allow: set[str], strict: bool) -> int:
    errors = findings["errors"]
    all_warnings = findings["warnings"]
    warnings = [w for w in all_warnings if w.get("type") not in allow]
    suppressed = len(all_warnings) - len(warnings)

    def fmt(entry: dict) -> str:
        count = entry.get("msgcount", "1")
        prefix = f"x{count:<3}" if count and count != "1" else "     "
        line = f"L{entry['line']}: " if entry.get("line") else ""
        return f"  {prefix}{entry.get('type', '?')}: {line}{entry.get('text', '')}"

    print(f"W3C Feed Validation Service (rawdata POST) — {source}")
    if errors:
        print(f"\nERRORS ({len(errors)} type(s)):")
        for entry in errors:
            print(fmt(entry))
    if warnings:
        print(f"\nwarnings ({len(warnings)} type(s)) — non-blocking, STRICT=1 to fail:")
        for entry in warnings:
            print(fmt(entry))
    if suppressed:
        print(f"\n({suppressed} allowlisted warning type(s) hidden: {', '.join(sorted(allow))})")

    if findings["errors"]:
        print("\nverdict: INVALID")
        return 1
    if strict and warnings:
        print("\nverdict: VALID but strict mode fails on warnings")
        return 1
    print("\nverdict: VALID")
    return 0


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("source", help="feed file path or http(s) URL")
    parser.add_argument("--strict", action="store_true", help="fail on warnings too")
    parser.add_argument(
        "--allow",
        default=",".join(DEFAULT_ALLOW),
        help="warning types to ignore, comma-separated (default: %(default)s; "
        "pass an empty string to allow nothing)",
    )
    args = parser.parse_args()
    allow = {t.strip() for t in args.allow.split(",") if t.strip()}

    try:
        content = load_source(args.source)
    except (OSError, urllib.error.URLError) as exc:
        sys.stderr.write(f"!! cannot load {args.source}: {exc}\n")
        sys.exit(2)

    findings = validate(content)
    sys.exit(report(args.source, findings, allow, args.strict))


if __name__ == "__main__":
    main()
