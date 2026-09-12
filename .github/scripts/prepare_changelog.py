"""Prepare CHANGELOG.md for a release.

Usage: prepare_changelog.py VERSION [--final]

With --final (version X.Y.Z): combines the [Unreleased] section and all
X.Y.Z-rcN sections into a single `## [X.Y.Z] - <today>` section, removes
the consumed rc sections, and inserts a fresh empty [Unreleased] section.

Without --final (version X.Y.Z-rcN): renames [Unreleased] to
`## [X.Y.Z-rcN] - <today>` and inserts a fresh empty [Unreleased] section.

Fails with a non-zero exit code when there is nothing to release (no
[Unreleased] section or no entries to publish), so a tag is never created
from an out-of-date changelog.
"""

import re
import sys
from datetime import datetime, timezone

SECTION_HEADER_RE = re.compile(r"(?m)^(## \[[^\]]+\][^\n]*)\n")
SECTION_NAME_RE = re.compile(r"^## \[([^\]]+)\]")
RC_SECTION_RE = re.compile(r"^\d+\.\d+\.\d+-rc\d+$")


def parse_entries(body):
    """Extract (category, bullet) pairs from a section body, in order."""
    entries = []
    category = None
    for line in body.splitlines():
        stripped = line.strip()
        if stripped.startswith("### "):
            category = stripped[4:].strip()
        elif stripped.startswith("- ") and category is not None:
            entries.append((category, stripped[2:].strip()))
    return entries


def render_section(header, body):
    return header + "\n" + body


def main():
    args = sys.argv[1:]
    final = "--final" in args
    versions = [a for a in args if not a.startswith("--")]
    if len(versions) != 1:
        sys.exit("Usage: prepare_changelog.py VERSION [--final]")
    version = versions[0].removeprefix("v")
    if not re.match(r"^\d+\.\d+\.\d+(-rc\d+)?$", version):
        sys.exit(f"Invalid version '{version}': expected X.Y.Z or X.Y.Z-rcN")

    path = "CHANGELOG.md"
    with open(path, encoding="utf-8") as f:
        text = f.read()

    matches = list(SECTION_HEADER_RE.finditer(text))
    if not matches:
        sys.exit("No version sections found in CHANGELOG.md")

    preamble = text[: matches[0].start()]
    sections = []
    for index, match in enumerate(matches):
        end = matches[index + 1].start() if index + 1 < len(matches) else len(text)
        sections.append((match.group(1), text[match.end(): end]))

    today = datetime.now(timezone.utc).date().isoformat()
    fresh_unreleased = "## [Unreleased]\n\n"

    if not final:
        renamed = False
        for index, (header, body) in enumerate(sections):
            if SECTION_NAME_RE.match(header).group(1) != "Unreleased":
                continue
            if not parse_entries(body):
                sys.exit(
                    "CHANGELOG.md [Unreleased] section has no entries: "
                    "update it with the changes to release before tagging"
                )
            rest = "".join(render_section(h, b) for h, b in sections[index + 1:])
            new_text = (
                preamble
                + fresh_unreleased
                + render_section(f"## [{version}] - {today}", body)
                + rest
            )
            renamed = True
            break
        if not renamed:
            sys.exit("CHANGELOG.md has no [Unreleased] section")
    else:
        combined = {}
        order = []
        consumed_headers = set()
        released_entries = 0
        for header, body in sections:
            name = SECTION_NAME_RE.match(header).group(1)
            is_unreleased = name == "Unreleased"
            is_matching_rc = RC_SECTION_RE.match(name) and name.startswith(version + "-rc")
            if not (is_unreleased or is_matching_rc):
                continue
            consumed_headers.add(header)
            for category, bullet in parse_entries(body):
                if category not in combined:
                    combined[category] = []
                    order.append(category)
                if bullet not in combined[category]:
                    combined[category].append(bullet)
                    released_entries += 1
        if released_entries == 0:
            sys.exit(
                "CHANGELOG.md has no entries to release: [Unreleased] and "
                f"the {version}-rcN sections are empty"
            )
        body = "\n"
        for category in order:
            body += f"### {category}\n\n"
            for bullet in combined[category]:
                body += f"- {bullet}\n"
            body += "\n"
        kept = "".join(
            render_section(h, b)
            for h, b in sections
            if h not in consumed_headers
        )
        new_text = (
            preamble
            + fresh_unreleased
            + render_section(f"## [{version}] - {today}", body)
            + kept
        )

    with open(path, "w", encoding="utf-8") as f:
        f.write(new_text)
    print(f"CHANGELOG.md updated for {version}")


if __name__ == "__main__":
    main()
