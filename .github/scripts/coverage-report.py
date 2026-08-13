#!/usr/bin/env python3
"""Generate a combined Rust + Java coverage report as markdown for PR comments."""

import sys
import xml.etree.ElementTree as ET
from pathlib import Path


def parse_rust_coverage(path):
    """Parse Cobertura XML from cargo-llvm-cov."""
    if not Path(path).exists():
        return None

    tree = ET.parse(path)
    root = tree.getroot()

    line_rate = float(root.get("line-rate", "0"))
    lines_covered = int(root.get("lines-covered", "0"))
    lines_valid = int(root.get("lines-valid", "0"))

    methods_total = 0
    methods_covered = 0

    for method in root.findall(".//method"):
        methods_total += 1
        method_line_rate = float(method.get("line-rate", "0"))
        if method_line_rate > 0:
            methods_covered += 1

    return {
        "line_rate": line_rate,
        "lines_covered": lines_covered,
        "lines_valid": lines_valid,
        "methods_total": methods_total,
        "methods_covered": methods_covered,
    }


def parse_java_coverage(path):
    """Parse JaCoCo XML from jacoco:report."""
    if not Path(path).exists():
        return None

    tree = ET.parse(path)
    root = tree.getroot()

    counters = {}
    for counter in root.findall("counter"):
        ctype = counter.get("type")
        missed = int(counter.get("missed", "0"))
        covered = int(counter.get("covered", "0"))
        counters[ctype] = {
            "missed": missed,
            "covered": covered,
            "total": missed + covered,
        }

    classes = []
    for cls in root.findall(".//class"):
        cls_name = cls.get("name", "").split("/")[-1]
        cls_counters = {}
        for counter in cls.findall("counter"):
            ctype = counter.get("type")
            missed = int(counter.get("missed", "0"))
            covered = int(counter.get("covered", "0"))
            cls_counters[ctype] = {
                "missed": missed,
                "covered": covered,
                "total": missed + covered,
            }
        classes.append({"name": cls_name, "counters": cls_counters})

    return {"counters": counters, "classes": classes}


def pct(covered, total):
    if total == 0:
        return 0.0
    return 100.0 * covered / total


def fmt_pct(covered, total):
    p = pct(covered, total)
    icon = "white_check_mark" if p == 100 else ("x" if p < 50 else "warning")
    return f"{covered}/{total} ({p:.1f}%) :{icon}:"


def generate_markdown(rust_data, java_data):
    lines = []
    lines.append("## :bar_chart: Code Coverage Report")
    lines.append("")

    # Summary table
    lines.append("| Language | Lines | Functions/Methods | Branches |")
    lines.append("|----------|-------|-------------------|----------|")

    if java_data:
        c = java_data["counters"]
        line = c.get("LINE", {"covered": 0, "total": 0})
        method = c.get("METHOD", {"covered": 0, "total": 0})
        branch = c.get("BRANCH", {"covered": 0, "total": 0})
        lines.append(
            f"| **Java** | {fmt_pct(line['covered'], line['total'])} "
            f"| {fmt_pct(method['covered'], method['total'])} "
            f"| {fmt_pct(branch['covered'], branch['total'])} |"
        )
    else:
        lines.append("| **Java** | N/A | N/A | N/A |")

    if rust_data:
        lines.append(
            f"| **Rust** | {fmt_pct(rust_data['lines_covered'], rust_data['lines_valid'])} "
            f"| {fmt_pct(rust_data['methods_covered'], rust_data['methods_total'])} "
            f"| - |"
        )
    else:
        lines.append("| **Rust** | N/A | N/A | - |")

    lines.append("")

    # Java details
    if java_data and java_data["classes"]:
        lines.append("### Java Coverage by Class")
        lines.append("")
        lines.append("| Class | Instructions | Lines | Methods |")
        lines.append("|-------|-------------|-------|---------|")
        for cls in sorted(java_data["classes"], key=lambda c: c["name"]):
            cc = cls["counters"]
            instr = cc.get("INSTRUCTION", {"covered": 0, "total": 0})
            line = cc.get("LINE", {"covered": 0, "total": 0})
            method = cc.get("METHOD", {"covered": 0, "total": 0})
            lines.append(
                f"| {cls['name']} "
                f"| {fmt_pct(instr['covered'], instr['total'])} "
                f"| {fmt_pct(line['covered'], line['total'])} "
                f"| {fmt_pct(method['covered'], method['total'])} |"
            )
        lines.append("")

    # Rust details
    if rust_data:
        lines.append("### Rust Coverage")
        lines.append("")
        lines.append("| Metric | Covered | Total | Coverage |")
        lines.append("|--------|---------|-------|----------|")
        lines.append(
            f"| Lines | {rust_data['lines_covered']} "
            f"| {rust_data['lines_valid']} "
            f"| {pct(rust_data['lines_covered'], rust_data['lines_valid']):.1f}% |"
        )
        lines.append(
            f"| Functions | {rust_data['methods_covered']} "
            f"| {rust_data['methods_total']} "
            f"| {pct(rust_data['methods_covered'], rust_data['methods_total']):.1f}% |"
        )
        lines.append("")
        lines.append(
            "_Rust JNI functions are tested through Java integration tests "
            "but not measured by cargo-llvm-cov._"
        )

    lines.append("")
    return "\n".join(lines)


def main():
    project_root = Path(__file__).resolve().parent.parent.parent
    rust_xml = project_root / "target" / "coverage" / "rust-coverage.xml"
    java_xml = project_root / "target" / "site" / "jacoco" / "jacoco.xml"

    rust_data = parse_rust_coverage(str(rust_xml))
    java_data = parse_java_coverage(str(java_xml))

    if not rust_data and not java_data:
        print("No coverage data found", file=sys.stderr)
        sys.exit(1)

    markdown = generate_markdown(rust_data, java_data)
    output_path = project_root / "coverage-report.md"
    output_path.write_text(markdown)
    print(f"Coverage report written to {output_path}")


if __name__ == "__main__":
    main()
