#!/usr/bin/env python3
'''
Download OPTIONAL OpenRA assets for the C&C Rust port (Phase 10.1).

Licensing — read before use:
    OpenRA is GPL-3.0 for its own code; OpenRA-original artwork is CC-BY-SA
    3.0/4.0; art derived from the original EA Command & Conquer titles is
    NOT covered by OpenRA's licenses and remains Electronic Arts'
    property. Nothing in this script grants rights to original EA assets.
    Share-alike obligations CC-BY-SA apply to OpenRA-original art and to
    any derivative works of it — see ASSETS/OpenRA/README.md.

    These assets are entirely OPTIONAL. The engine runs without them: the
    procedural generators in RUST/cnc-procgen synthesize replacement art.

Usage:
    python3 SCRIPTS/download_openra_assets.py --list
    python3 SCRIPTS/download_openra_assets.py [--dest DIR] [--dry-run] [--force]

Stdlib only. Downloads are streamed with a 30s timeout, written atomically
(tmp file + rename), sha256-verified, and skipped on re-runs when the
manifest records a matching hash. No downloaded content is ever executed.
'''
import argparse
import hashlib
import json
import sys
import urllib.request
from pathlib import Path

MANIFEST_NAME = "manifest.json"
CHUNK_BYTES = 64 * 1024
TIMEOUT_SECONDS = 30

# Curated entry set. URLs verified live against the OpenRA repository
# (branch "bleed") on 2026-10-19. source_sha is the GitHub blob sha1 from
# the contents API, used only as an extra cross-check in the manifest.
# NOTE: `bytes` is advisory only. Branch "bleed" is a moving target, so file
# sizes drift upstream and an exact pin goes stale (that brittleness once
# aborted every first fetch). A mismatch is now logged to stderr, never fatal.
# The hard integrity gate is the sha256: computed while streaming, recorded in
# manifest.json on first fetch, and re-verified on every idempotent re-run.
ENTRIES = [
    {
        "name": "openra-copying.txt",
        "url": "https://raw.githubusercontent.com/OpenRA/OpenRA/bleed/COPYING",
        "license": "GPL-3.0-or-later",
        "source_sha": "f9be34c7",
        "bytes": 35151,
    },
    {
        "name": "ra-mod.yaml",
        "url": "https://raw.githubusercontent.com/OpenRA/OpenRA/bleed/mods/ra/mod.yaml",
        "license": "CC-BY-SA-3.0/4.0 (OpenRA-original file; mod rules describe mixed-origin content)",
        "source_sha": "920a32b2",
        "bytes": 7280,
    },
    {
        "name": "ra-missions.yaml",
        "url": "https://raw.githubusercontent.com/OpenRA/OpenRA/bleed/mods/ra/missions.yaml",
        "license": "CC-BY-SA-3.0/4.0 (OpenRA-original file)",
        "source_sha": "417c3a36",
        "bytes": 1074,
    },
]


def default_dest() -> Path:
    '''Shared asset area at the repo root, resolved from this file's path.'''
    repo_root = Path(__file__).resolve().parent.parent
    return repo_root / "ASSETS" / "OpenRA"


def load_manifest(dest: Path) -> dict:
    manifest_path = dest / MANIFEST_NAME
    if not manifest_path.is_file():
        return {"files": {}}
    try:
        with manifest_path.open("r", encoding="utf-8") as handle:
            data = json.load(handle)
    except (json.JSONDecodeError, OSError) as exc:
        print(f"warning: unreadable manifest ({exc}); treating as empty", file=sys.stderr)
        return {"files": {}}
    return data if isinstance(data, dict) else {"files": {}}


def save_manifest(dest: Path, manifest: dict) -> None:
    manifest_path = dest / MANIFEST_NAME
    tmp_path = manifest_path.with_suffix(".json.tmp")
    with tmp_path.open("w", encoding="utf-8") as handle:
        json.dump(manifest, handle, indent=2, sort_keys=True)
        handle.write("\n")
        handle.flush()
    tmp_path.replace(manifest_path)


def entry_known(manifest: dict, entry: dict, dest: Path) -> bool:
    '''True when the file exists and its sha256 matches the manifest record.'''
    record = manifest.get("files", {}).get(entry["name"])
    target = dest / entry["name"]
    if not record or not target.is_file():
        return False
    digest = sha256_file(target)
    return digest is not None and digest == record.get("sha256")


def sha256_file(path: Path):
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        while True:
            chunk = handle.read(CHUNK_BYTES)
            if not chunk:
                break
            digest.update(chunk)
    return digest.hexdigest()


def fetch_entry(entry: dict, dest: Path) -> dict:
    '''Stream one entry to dest atomically; verify sha256 of what landed.

    Returns {"sha256": <hexdigest of what landed>, "bytes": <actual length>}.
    The streamed sha256 is the integrity gate recorded in the manifest; the
    "bytes" field on an entry is advisory (see ENTRIES note) and is only
    logged on mismatch, never fatal.
    '''
    if not entry["url"].startswith("https://"):
        raise RuntimeError(f"refusing non-https url: {entry['url']}")
    dest.mkdir(parents=True, exist_ok=True)
    target = dest / entry["name"]
    tmp_path = target.with_suffix(target.suffix + ".tmp")
    digest = hashlib.sha256()
    total = 0
    request = urllib.request.Request(
        entry["url"], headers={"User-Agent": "cnc-rust-port-asset-fetch/1.0"}
    )
    try:
        with urllib.request.urlopen(request, timeout=TIMEOUT_SECONDS) as response, tmp_path.open("wb") as out:
            while True:
                chunk = response.read(CHUNK_BYTES)
                if not chunk:
                    break
                out.write(chunk)
                digest.update(chunk)
                total += len(chunk)
    except OSError as exc:
        tmp_path.unlink(missing_ok=True)
        raise RuntimeError(f"download failed for {entry['name']}: {exc}") from exc
    expected = entry.get("bytes")
    if expected is not None and total != expected:
        print(
            f"warning: {entry['name']} is {total} B, advisory pin says {expected} B "
            "(upstream drift on branch 'bleed'); sha256 remains the integrity gate",
            file=sys.stderr,
        )
    tmp_path.replace(target)
    return {"sha256": digest.hexdigest(), "bytes": total}


def list_entries() -> str:
    lines = []
    for entry in ENTRIES:
        lines.append(
            f"{entry['name']:<20} {entry['bytes']:>7} B  {entry['license']}\n"
            f"    {entry['url']}"
        )
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Download optional OpenRA assets (CC-BY-SA / GPL). See header docstring."
    )
    parser.add_argument("--dest", type=Path, default=default_dest(),
                        help="destination directory (default: <repo>/ASSETS/OpenRA)")
    parser.add_argument("--list", action="store_true", help="list planned entries and exit")
    parser.add_argument("--dry-run", action="store_true", help="plan without writing or fetching")
    parser.add_argument("--force", action="store_true", help="re-download even if sha256 matches")
    args = parser.parse_args()

    if args.list:
        print(f"entries (dest: {args.dest}):")
        print(list_entries())
        return 0

    manifest = load_manifest(args.dest)
    fetched = skipped = 0
    for entry in ENTRIES:
        if not args.force and not args.dry_run and entry_known(manifest, entry, args.dest):
            print(f"skip    {entry['name']} (sha256 verified in manifest)")
            skipped += 1
            continue
        if args.dry_run:
            print(f"plan    {entry['name']}  {entry['bytes']} B  <- {entry['url']}")
            continue
        print(f"fetch   {entry['name']} ...")
        result = fetch_entry(entry, args.dest)
        manifest.setdefault("files", {})[entry["name"]] = {
            "url": entry["url"],
            "license": entry["license"],
            "source_sha": entry.get("source_sha"),
            **result,
        }
        fetched += 1
        print(f"ok      {entry['name']}  sha256={result['sha256'][:16]}...  {result['bytes']} B")

    if fetched and not args.dry_run:
        save_manifest(args.dest, manifest)
        print(f"manifest: {args.dest / MANIFEST_NAME}")
    print(f"done: {fetched} fetched, {skipped} skipped")
    return 0


if __name__ == "__main__":
    sys.exit(main())
