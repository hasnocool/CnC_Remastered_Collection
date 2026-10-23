# TIBERIANDAWN/WIN32LIB/ — Win32 Platform Abstraction Layer (Tiberian Dawn)

## Responsibility
**Platform-specific implementation** for Tiberian Dawn engine: video, input, audio, file I/O, registry, timing, and DirectDraw wrappers. Nearly identical to `REDALERT/WIN32LIB/` — shared Westwood platform layer with TD-specific configuration.

## Design Patterns
Same subsystem architecture as `REDALERT/WIN32LIB/`:
- Video: `DDRAW`, `DIB`, `PALETTE`
- Input: `MOUSE`, `KEYBOARD`, `JOYSTICK`
- Audio: `AUDIO`, `MIXER`, `CDAUDIO`
- File I/O: `FILE`, `CCFILE`
- Registry: `REGISTRY`
- Timing: `TIMER`
- Memory: `ALLOC` (TD has unique `ALLOC.CPP` with debug tracking)
- Network: `CONNECT`, `CONNMGR`

## Key Differences from Red Alert WIN32LIB
| Aspect | Tiberian Dawn | Red Alert |
|--------|---------------|-----------|
| **Memory Allocator** | `ALLOC.CPP` — custom debug alloc with leak tracking, guard pages | Simpler allocator |
| **Video Modes** | Fewer default resolutions (1995 era) | More resolutions, hi-res support |
| **CD Audio** | 20 tracks (GDI/NOD campaigns) | 30+ tracks (Allies/Soviet + expansions) |
| **Registry Keys** | `Westwood\TiberianDawn` | `Westwood\RedAlert` |
| **DirectPlay GUID** | TD-specific application GUID | RA-specific GUID |

## Unique to Tiberian Dawn
- **`ALLOC.CPP`** — Comprehensive debug memory system:
  - Guard pages before/after allocations
  - Allocation stack traces
  - Leak reporting on shutdown
  - Memory usage statistics
  - Not present in Red Alert (simpler `new`/`delete` overrides only)

## Integration Points
Same consumers as Red Alert — engine core modules (`AUDIO`, `DISPLAY`, `MOUSE`, `CCFILE`, `COMQUEUE`, `TIMER`).

## Key Files
| File | Role |
|------|------|
| `ALLOC.CPP/H` | **TD-unique** debug memory allocator |
| `DDRAW.CPP/H` | DirectDraw wrapper |
| `AUDIO.CPP/H` | DirectSound wrapper |
| `FILE.CPP/H` | File I/O + `.mix` |
| `TIMER.CPP/H` | Frame timer |
| `CONNECT.CPP/H` | DirectPlay networking |