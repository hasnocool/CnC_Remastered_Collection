# AGENTS.md — REDALERT/

**Generated:** 2026-10-09 · **Commit:** f1f0d42 · **Branch:** ?

## OVERVIEW
Red Alert game engine (C++): 519 files, flat directory with ALL-CAPS filenames. Core game logic for Command & Conquer: Red Alert. Implements unit/building AI, combat simulation, pathfinding, audio, animation, networking, resource management, and Win32 platform abstraction.

## STRUCTURE


## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Game logic | 
| Win32 glue | 
| Deep architecture docs | 

## CONVENTIONS
- Westwood-era C++: custom RTTI enum + member (never C++ RTTI), CCPtr<T> smart pointers, NoInitClass placement-new pattern, manual allocators (ALLOC.CPP), object pools, operator new/delete overrides on hot classes.
- Deterministic lockstep simulation: discrete frames, command queue broadcast, pointer fixup via Code_Pointers/Decode_Pointers on save/load. RA uses fixed-point fixed (16.16); TD uses float bias values — do not unify.
- Flat directory with ALL-CAPS filenames; .H/ .CPP extensions.

## ANTI-PATTERNS (THIS PROJECT)
- Do not break ABI/serialization: saved-game and network-sync formats are load-bearing; struct layout and Code_Pointers fixup must stay consistent.
- Do not "modernize" engine code wholesale (STL migration, RTTI removal): the DLL must interoperate with the GlyphX client; behavior changes desync multiplayer lockstep.

## COMMANDS

## NOTES
- hig file concentration: REDALERT/ (519) — rely on per-dir codemaps rather than browsing.
