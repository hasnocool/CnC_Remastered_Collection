# Function and Porting Contract Catalog

**Pinned upstream:** electronicarts/CnC_Remastered_Collection at f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae  
**Manifest:** SOURCE_FILE_MANIFEST.csv — all 1,331 tracked upstream file paths  
**Generator:** scripts/generate_function_catalog.py

## Purpose

This catalog is the porting-oriented inventory of the original code surface. It answers which callable symbols exist in each source file, what behavior each provides, what state it reads or mutates, what it calls, and which contracts another language or platform must reproduce.

It is deliberately separate from:
- IMPLEMENTED_FUNCTIONALITY_MAP.md — what the Rust implementation currently does.
- ORIGINAL_FUNCTIONALITY_MAP.md — what the original games and editor do at subsystem level.
- 02-original-file-inventory.md — the pinned source-file inventory and documentation review state.

## Organization

The generator creates:

| Output | Contents |
|---|---|
| SUMMARY.md | Source counts, extraction counts, missing files, review status |
| REDALERT/FUNCTION_INDEX.md | Callable symbols grouped by original RA source file |
| TIBERIANDAWN/FUNCTION_INDEX.md | Callable symbols grouped by original TD source file |
| MAP_EDITOR/FUNCTION_INDEX.md | Methods and callable members grouped by original C# editor file |
| SCRIPTS_AND_TOOLS/FUNCTION_INDEX.md | Callable symbols from source utilities |
| SYMBOLS.jsonl | Machine-readable symbol inventory, one JSON object per line |
| FILE_COVERAGE.csv | Coverage row for every file in the 1,331-file manifest |

Every extracted symbol is marked SIGNATURE_CATALOGUED until its behavior is inspected and documented using PORTING_CONTRACT_TEMPLATE.md.

## Generate from the pinned source checkout

Run from the repository root after checking out the exact source tree under original/:

\`\`\`bash
python3 RUST/original-source-reference/FUNCTION_CATALOG/scripts/generate_function_catalog.py
\`\`\`

The generator uses Universal Ctags to enumerate C++, C#, Python, and assembly symbols. It reads only source paths in the pinned manifest, checks for missing inputs, and writes generated indexes without overwriting this README, the manifest, or manually maintained contract documents.

Install Universal Ctags:

\`\`\`bash
sudo apt-get install universal-ctags
# Arch Linux:
sudo pacman -S universal-ctags
\`\`\`

Useful options:

\`\`\`bash
python3 RUST/original-source-reference/FUNCTION_CATALOG/scripts/generate_function_catalog.py --dry-run
python3 RUST/original-source-reference/FUNCTION_CATALOG/scripts/generate_function_catalog.py --source-root /path/to/pinned/source
python3 RUST/original-source-reference/FUNCTION_CATALOG/scripts/generate_function_catalog.py --ctags /usr/bin/ctags
\`\`\`

The source root must contain the manifest paths directly (for example, REDALERT/, TIBERIANDAWN/, and CnCTDRAMapEditor/). Do not run this against a different revision and label the output as the pinned commit.

## Important interpretation rules

1. A symbol extractor finds declarations and definitions; it does not explain behavior. A detected name is not a reproduction specification.
2. A declaration in a header and a definition in a C++ file are separate source locations. Preserve both until the audit links them.
3. Function-like macros, assembly entry points, callbacks, virtual overrides, message handlers, and exported interfaces can be important even when they do not look like ordinary functions.
4. A port does not need to preserve private function names or file boundaries. It must preserve required behavior, invariants, formats, timing, ordering, deterministic outcomes, side effects, and externally visible contracts.
5. Never mark a function as reproduced merely because a similarly named function exists in another language. Verify behavior and the live call path.
6. Source files with no detected callable tags are not automatically empty; manually inspect macros, inline assembly, generated code, resources, and parser limitations.

## Review states

- UNSEEN — source has not been behavior-reviewed.
- SIGNATURE_CATALOGUED — symbol extracted, semantics not yet documented.
- CONTRACT_DOCUMENTED — inputs, outputs, state effects, callers/callees and reproduction requirements recorded.
- PORT_PARTIAL — some required behavior exists in the target, with named gaps.
- PORT_IMPLEMENTED_UNVERIFIED — implementation exists, parity evidence insufficient.
- PARITY_VERIFIED — source-derived fixtures/tests confirm required behavior.
- REPLACED_WITH_COMPATIBILITY_BOUNDARY — deliberate replacement and limits documented.
- NOT_REQUIRED_WITH_RATIONALE — behavior intentionally omitted with a documented reason.

The file manifest is complete for the pinned upstream tree. The symbol index is generated from the checkout; behavior contracts are completed domain by domain.
