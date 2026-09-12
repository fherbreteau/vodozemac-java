"""Compose GitHub release notes from CHANGELOG.md and the generated notes.

Combines the released version's CHANGELOG.md section with the notes from
GitHub's releases/generate-notes API so the final release body contains:

  1. a "Changes" section with the curated changelog content,
  2. a "What's Changed (not in the changelog)" section listing the merged
     pull requests that are NOT referenced in the changelog section
     (matched by the #NNN reference),
  3. GitHub's New Contributors section and Full Changelog link, verbatim.

Usage:
  compose_release_notes.py --version 1.0.0 --changelog CHANGELOG.md \
      --github-notes github-notes.md [--output release-notes.md]
"""

import argparse
import re
import sys

WHAT_CHANGED_HEADING = "## What's Changed"
PR_REF_RE = re.compile(r"in (?:#|.*?/pull/)(\d+)$")
SECTION_HEADING_RE = re.compile(r"(?m)^## \[")


def extract_changelog_section(changelog_path, version):
    with open(changelog_path, encoding="utf-8") as f:
        text = f.read()
    header = f"## [{version}]"
    start = text.find(header)
    if start == -1 or (start > 0 and text[start - 1] != "\n"):
        sys.exit(f"No '{header}' section found in {changelog_path}")
    body_start = text.index("\n", start) + 1
    rest = text[body_start:]
    match = SECTION_HEADING_RE.search(rest)
    body = rest[: match.start()] if match else rest
    return body.strip("\n")


def split_github_notes(notes_path):
    with open(notes_path, encoding="utf-8") as f:
        lines = f.read().splitlines()
    bullets = []
    tail = []
    in_changes = False
    for line in lines:
        if line.strip() == WHAT_CHANGED_HEADING:
            in_changes = True
            continue
        if in_changes and (
            line.startswith("## ") or line.startswith("**Full Changelog**")
        ):
            in_changes = False
        if in_changes:
            if line.startswith("* "):
                bullets.append(line)
        elif line.strip() or tail:
            tail.append(line)
    while tail and not tail[-1].strip():
        tail.pop()
    return bullets, tail


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--version", required=True, help="release version, without the v prefix")
    parser.add_argument("--changelog", default="CHANGELOG.md")
    parser.add_argument("--github-notes", required=True)
    parser.add_argument("--output", default="-")
    args = parser.parse_args()

    section = extract_changelog_section(args.changelog, args.version)
    changelog_refs = set(re.findall(r"#\d+", section))

    bullets, tail = split_github_notes(args.github_notes)
    remaining = [
        bullet
        for bullet in bullets
        if (match := PR_REF_RE.search(bullet)) is None
        or f"#{match.group(1)}" not in changelog_refs
    ]

    parts = [f"## Changes\n\n{section}\n"]
    if remaining:
        parts.append(
            "## What's Changed (not in the changelog)\n\n" + "\n".join(remaining) + "\n"
        )
    if tail:
        parts.append("\n".join(tail) + "\n")

    output = "\n".join(parts)
    if args.output == "-":
        sys.stdout.write(output)
    else:
        with open(args.output, "w", encoding="utf-8") as f:
            f.write(output)


if __name__ == "__main__":
    main()
