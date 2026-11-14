#!/usr/bin/env python3
# generate_function_catalog.py
"""Generate a pinned per-file callable-symbol inventory with Universal Ctags.

The output is a symbol index, not an automatic semantic port specification.
Every symbol remains SIGNATURE_CATALOGUED until a human records its behavior
using PORTING_CONTRACT_TEMPLATE.md.
"""
from __future__ import annotations

import argparse
import asyncio
import csv
import json
import sys
from collections import Counter, defaultdict
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

PINNED_SOURCE_COMMIT = "f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae"
CATALOG_ROOT = Path(__file__).resolve().parents[1]
REPOSITORY_ROOT = Path(__file__).resolve().parents[4]
DEFAULT_SOURCE_ROOT = REPOSITORY_ROOT / "original"
DEFAULT_MANIFEST = CATALOG_ROOT / "SOURCE_FILE_MANIFEST.csv"
DEFAULT_OUTPUT = CATALOG_ROOT

GROUPS = ("REDALERT", "TIBERIANDAWN", "MAP_EDITOR", "SCRIPTS_AND_TOOLS")
FUNCTION_KINDS = {
    "C++": {"function", "prototype", "f", "p"},
    "C#": {"method", "property", "event", "delegate", "m", "p", "e"},
    "Python": {"function", "method", "f", "m"},
    "Asm": {"function", "procedure", "proc", "entry", "f"},
}
MACRO_KINDS = {"macro", "d"}
ASSEMBLY_LABEL_KINDS = {"label", "l"}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Build a per-file function/method inventory from the pinned CnC source tree."
    )
    parser.add_argument(
        "--source-root",
        type=Path,
        default=DEFAULT_SOURCE_ROOT,
        help="Directory containing REDALERT/, TIBERIANDAWN/, and CnCTDRAMapEditor/.",
    )
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--ctags", default="ctags", help="Universal Ctags executable.")
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Validate manifest/source paths without running Ctags.",
    )
    parser.add_argument(
        "--allow-missing",
        action="store_true",
        help="Continue when pinned source files are absent; missing files are recorded.",
    )
    return parser.parse_args()


def load_manifest(path: Path) -> list[dict[str, str]]:
    if not path.is_file():
        raise FileNotFoundError(f"Manifest not found: {path}")
    with path.open("r", encoding="utf-8-sig", newline="") as handle:
        rows = list(csv.DictReader(handle))
    if not rows:
        raise ValueError(f"Manifest contains no source rows: {path}")
    commits = {row.get("pinned_source_commit", "") for row in rows}
    if commits != {PINNED_SOURCE_COMMIT}:
        raise ValueError(
            f"Manifest revision mismatch: expected {PINNED_SOURCE_COMMIT}, got {sorted(commits)}"
        )
    required = {
        "source_path",
        "subsystem",
        "extension",
        "language",
        "inventory_role",
    }
    if not required.issubset(rows[0]):
        missing = sorted(required - set(rows[0]))
        raise ValueError(f"Manifest is missing required columns: {missing}")
    return rows


def normalize_path(value: str) -> str:
    result = Path(value.replace("\\", "/")).as_posix()
    while result.startswith("./"):
        result = result[2:]
    return result


def group_for(row: dict[str, str]) -> str:
    group = row["subsystem"]
    return group if group in GROUPS else "SCRIPTS_AND_TOOLS"


def markdown_cell(value: Any) -> str:
    text = "" if value is None else str(value)
    return text.replace("|", r"\|").replace("\r", " ").replace("\n", " ")


def is_callable_tag(tag: dict[str, Any]) -> tuple[bool, str]:
    language = str(tag.get("language", ""))
    kind = str(tag.get("kind", "")).strip().lower()
    name = str(tag.get("name", ""))

    if kind in FUNCTION_KINDS.get(language, set()):
        return True, "callable"
    if kind in MACRO_KINDS and tag.get("signature"):
        return True, "function-like macro"
    if language == "Asm" and kind in ASSEMBLY_LABEL_KINDS:
        # A label is a routine candidate until PROC/global-entry semantics are
        # confirmed directly from the assembly source.
        return True, "assembly label candidate"
    if language == "Asm" and name.upper().endswith("_PROC"):
        return True, "assembly routine candidate"
    return False, ""


async def run_process(*command: str, cwd: Path) -> tuple[int, bytes, bytes]:
    process = await asyncio.create_subprocess_exec(
        *command,
        cwd=str(cwd),
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE,
    )
    stdout, stderr = await process.communicate()
    return process.returncode, stdout, stderr


async def extract_tags(
    ctags: str, source_root: Path, source_paths: list[str]
) -> tuple[list[dict[str, Any]], str]:
    version_code, version_out, version_err = await run_process(
        ctags, "--version", cwd=source_root
    )
    version = (version_out or version_err).decode("utf-8", errors="replace").strip()
    if version_code != 0:
        raise RuntimeError(f"Unable to run Ctags: {version}")
    if "Universal Ctags" not in version:
        detected = version.splitlines()[0] if version else "unknown"
        raise RuntimeError(
            "Universal Ctags is required (JSON output is not supported by every ctags fork). "
            f"Detected: {detected}"
        )

    command = [
        ctags,
        "--options=NONE",
        "--output-format=json",
        "--fields=+nKSa",
        "--fields=+s",
        "--fields=+l",
        "--extras=+q",
        "--sort=no",
        "--languages=C++,C#,Asm,Python",
        "-f",
        "-",
        *source_paths,
    ]
    code, stdout, stderr = await run_process(*command, cwd=source_root)
    if code not in (0, 1):
        detail = stderr.decode("utf-8", errors="replace")
        raise RuntimeError(f"Ctags failed with exit code {code}:\n{detail[-8000:]}")

    tags: list[dict[str, Any]] = []
    malformed = 0
    for raw_line in stdout.decode("utf-8", errors="replace").splitlines():
        try:
            item = json.loads(raw_line)
        except json.JSONDecodeError:
            malformed += 1
            continue
        if item.get("_type") == "tag":
            tags.append(item)
    if malformed:
        print(
            f"warning: skipped {malformed} non-JSON Ctags output lines",
            file=sys.stderr,
        )
    detail = stderr.decode("utf-8", errors="replace").strip()
    return tags, version + (f"\n{detail}" if detail else "")


def write_text(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + ".tmp")
    temporary.write_text(content, encoding="utf-8", newline="\n")
    temporary.replace(path)


def build_outputs(
    rows: list[dict[str, str]],
    source_root: Path,
    output_dir: Path,
    tags: list[dict[str, Any]],
    ctags_version: str,
    missing: list[str],
) -> dict[str, int]:
    manifest_by_path = {normalize_path(row["source_path"]): row for row in rows}
    callable_by_path: dict[str, list[dict[str, Any]]] = defaultdict(list)
    macro_by_path: Counter[str] = Counter()
    asm_by_path: Counter[str] = Counter()
    symbols: list[dict[str, Any]] = []

    for tag in tags:
        raw_path = normalize_path(str(tag.get("path", "")))
        if raw_path not in manifest_by_path:
            continue
        row = manifest_by_path[raw_path]
        if row["inventory_role"] != "callable-source-or-assembly":
            continue
        accepted, category = is_callable_tag(tag)
        if not accepted:
            continue

        record = {
            "source_path": raw_path,
            "subsystem": row["subsystem"],
            "language": str(tag.get("language", row["language"])),
            "name": str(tag.get("name", "")),
            "kind": str(tag.get("kind", "unknown")),
            "category": category,
            "scope": tag.get("scope", ""),
            "signature": tag.get("signature", ""),
            "line": int(tag.get("line", 0) or 0),
            "access": tag.get("access", ""),
            "typeref": tag.get("typeref", ""),
            "review_state": "SIGNATURE_CATALOGUED",
        }
        callable_by_path[raw_path].append(record)
        if category == "function-like macro":
            macro_by_path[raw_path] += 1
        if category == "assembly label candidate":
            asm_by_path[raw_path] += 1
        symbols.append(record)

    for values in callable_by_path.values():
        values.sort(
            key=lambda item: (
                item["line"],
                item["name"],
                item["kind"],
                str(item["signature"]),
            )
        )
    symbols.sort(
        key=lambda item: (
            item["subsystem"],
            item["source_path"],
            item["line"],
            item["name"],
        )
    )

    coverage_rows: list[dict[str, Any]] = []
    missing_set = set(missing)

    for row in sorted(rows, key=lambda item: item["source_path"]):
        path = normalize_path(row["source_path"])
        count = len(callable_by_path.get(path, []))
        exists = (source_root / Path(path)).is_file()
        if path in missing_set or not exists:
            status = "MISSING_PINNED_SOURCE"
        elif row["inventory_role"] != "callable-source-or-assembly":
            status = "NON_CALLABLE_RESOURCE_OR_BUILD"
        elif count:
            status = "SIGNATURES_EXTRACTED_SEMANTICS_UNREVIEWED"
        else:
            status = "NO_TAGS_MANUAL_REVIEW_REQUIRED"
        coverage_rows.append(
            {
                "source_path": path,
                "subsystem": row["subsystem"],
                "extension": row["extension"],
                "language": row["language"],
                "inventory_role": row["inventory_role"],
                "callable_symbols": count,
                "function_like_macros": macro_by_path[path],
                "assembly_label_candidates": asm_by_path[path],
                "catalog_status": status,
            }
        )

    # Generate a subsystem index, preserving each original source file and
    # exact source line. Names alone never become behavioral claims.
    for group in GROUPS:
        entries = [
            symbol
            for symbol in symbols
            if group_for(manifest_by_path[symbol["source_path"]]) == group
        ]
        lines = [
            f"# {group.replace('_', ' ').title()} — Function Index",
            "",
            f"**Pinned source:** {PINNED_SOURCE_COMMIT}  ",
            "**Interpretation:** extracted signatures only; behavior contracts remain to be reviewed.",
            "",
            "| Source file | Line | Kind | Symbol | Scope | Signature | Review state |",
            "|---|---:|---|---|---|---|---|",
        ]
        for item in entries:
            lines.append(
                f"| {markdown_cell(item['source_path'])} | {item['line']} | "
                f"{markdown_cell(item['kind'])} ({markdown_cell(item['category'])}) | "
                f"{markdown_cell(item['name'])} | {markdown_cell(item['scope'])} | "
                f"{markdown_cell(item['signature'])} | SIGNATURE_CATALOGUED |"
            )
        if not entries:
            lines.append(
                "| — | — | — | No callable symbol tags extracted | — | — | Manual inspection required |"
            )
        lines.extend(
            [
                "",
                f"**Extracted callable/candidate symbols:** {len(entries)}",
                "",
                "A source-level signature is not a behavior description. Use "
                "PORTING_CONTRACT_TEMPLATE.md to document each contract, or group "
                "tightly coupled helpers when their behavior is inseparable.",
                "",
            ]
        )
        write_text(output_dir / group / "FUNCTION_INDEX.md", "\n".join(lines))

    jsonl = "".join(
        json.dumps(item, ensure_ascii=False, sort_keys=True) + "\n"
        for item in symbols
    )
    write_text(output_dir / "SYMBOLS.jsonl", jsonl)

    coverage_header = [
        "source_path",
        "subsystem",
        "extension",
        "language",
        "inventory_role",
        "callable_symbols",
        "function_like_macros",
        "assembly_label_candidates",
        "catalog_status",
    ]
    import io

    buffer = io.StringIO(newline="")
    writer = csv.DictWriter(
        buffer, fieldnames=coverage_header, lineterminator="\n"
    )
    writer.writeheader()
    writer.writerows(coverage_rows)
    write_text(output_dir / "FILE_COVERAGE.csv", buffer.getvalue())

    status_counts = Counter(item["catalog_status"] for item in coverage_rows)
    generated_on = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
    summary = [
        "# Function Catalog Generation Summary",
        "",
        f"- Generated: {generated_on}",
        f"- Pinned source revision: {PINNED_SOURCE_COMMIT}",
        f"- Source root: {source_root}",
        f"- Ctags: {ctags_version.splitlines()[0] if ctags_version else 'unknown'}",
        f"- Manifest files: {len(rows)}",
        f"- Callable/candidate symbols: {len(symbols)}",
        f"- Source files missing from checkout: {len(missing)}",
        "",
        "## Coverage by source group",
        "",
        "| Group | Manifest files | Files with extracted symbols | Symbols |",
        "|---|---:|---:|---:|",
    ]
    for group in GROUPS:
        group_rows = [
            row for row in coverage_rows
            if group_for(manifest_by_path[row["source_path"]]) == group
        ]
        group_symbols = [
            symbol for symbol in symbols
            if group_for(manifest_by_path[symbol["source_path"]]) == group
        ]
        files_with = sum(1 for row in group_rows if row["callable_symbols"] > 0)
        summary.append(
            f"| {group} | {len(group_rows)} | {files_with} | {len(group_symbols)} |"
        )
    summary.extend(
        [
            "",
            "## File status",
            "",
            "| Status | Files |",
            "|---|---:|",
        ]
    )
    for status, count in sorted(status_counts.items()):
        summary.append(f"| {status} | {count} |")
    summary.extend(
        [
            "",
            "## Important limitation",
            "",
            "This run enumerates symbols; it does not semantically document their "
            "behavior. Every extracted item is SIGNATURE_CATALOGUED, not "
            "CONTRACT_DOCUMENTED or PARITY_VERIFIED. Files with no tags need "
            "manual inspection; Ctags can miss macro-generated, unusual, "
            "inline-assembly, or compiler-specific constructs.",
            "",
            "## Missing pinned source files",
            "",
        ]
    )
    if missing:
        summary.extend(f"- {path}" for path in missing)
    else:
        summary.append("None.")
    summary.append("")
    write_text(output_dir / "SUMMARY.md", "\n".join(summary))

    return {
        "manifest_files": len(rows),
        "symbols": len(symbols),
        "missing": len(missing),
        "files_with_symbols": sum(1 for values in callable_by_path.values() if values),
        "files_without_detected_symbols": sum(
            1
            for row in rows
            if row["inventory_role"] == "callable-source-or-assembly"
            and normalize_path(row["source_path"]) not in missing_set
            and not callable_by_path.get(normalize_path(row["source_path"]))
        ),
    }


async def async_main() -> int:
    args = parse_args()
    rows = load_manifest(args.manifest)
    source_root = args.source_root.expanduser().resolve()
    output_dir = args.output_dir.expanduser().resolve()
    if not source_root.is_dir():
        raise NotADirectoryError(f"Source root not found: {source_root}")

    candidates = [
        normalize_path(row["source_path"])
        for row in rows
        if row["inventory_role"] == "callable-source-or-assembly"
    ]
    missing = [
        path for path in candidates
        if not (source_root / Path(path)).is_file()
    ]
    print(f"Manifest files: {len(rows)}")
    print(f"Callable-source/assembly candidates: {len(candidates)}")
    print(f"Missing callable-source files: {len(missing)}")
    for path in missing[:40]:
        print(f"  MISSING: {path}")

    if missing and not args.allow_missing:
        print(
            "Refusing to generate an incomplete symbol catalog; use "
            "--allow-missing only for a deliberate partial run.",
            file=sys.stderr,
        )
        return 2
    if args.dry_run:
        print("Dry run complete; no files written.")
        return 0

    tags, version = await extract_tags(args.ctags, source_root, candidates)
    result = build_outputs(rows, source_root, output_dir, tags, version, missing)
    print(json.dumps(result, indent=2))
    print(f"Generated indexes under: {output_dir}")
    return 0


def main() -> int:
    try:
        return asyncio.run(async_main())
    except KeyboardInterrupt:
        print("Interrupted; no background process remains.", file=sys.stderr)
        return 130
    except Exception as error:
        print(f"error: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
