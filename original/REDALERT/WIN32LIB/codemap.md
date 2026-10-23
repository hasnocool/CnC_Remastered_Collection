# REDALERT/WIN32LIB/ — Win32 Platform Abstraction Layer (Red Alert)

## Responsibility
**Platform-specific implementation** for Red Alert engine: video, input, audio, file I/O, registry, timing, and DirectDraw wrappers. Isolates engine core from Windows API details. Shared conceptually with `TIBERIANDAWN/WIN32LIB/` but may have RA-specific customizations.

## Design Patterns

### Subsystem Modules
| Module | Files | Responsibility |
|--------|-------|----------------|
| **Video** | `DDRAW.CPP/H`, `DIB.CPP/H`, `PALETTE.CPP/H` | DirectDraw surface management, palette animation, DIB conversion |
| **Input** | `MOUSE.CPP/H`, `KEYBOARD.CPP/H`, `JOYSTICK.CPP/H` | Device polling, cursor management, hotkey mapping |
| **Audio** | `AUDIO.CPP/H`, `MIXER.CPP/H`, `CDAUDIO.CPP/H` | DirectSound wrapper, streaming, CD music |
| **File I/O** | `FILE.CPP/H`, `CCFILE.CPP/H` | `.mix` archive access, `CCINIClass` INI parsing |
| **Registry** | `REGISTRY.CPP/H` | Settings persistence (video mode, volume, keys) |
| **Timing** | `TIMER.CPP/H` | High-resolution frame timer (`FrameTimerClass`) |
| **Memory** | `ALLOC.CPP/H` | Debug allocator, leak detection (RA variant) |
| **Network** | `CONNECT.CPP/H`, `CONNMGR.H` | DirectPlay/IPX/TCP abstraction (used by `COMQUEUE`) |

### Engine Interface
Each subsystem exposes a C++ class with virtual methods — engine core holds pointers to concrete Win32 implementations:
```cpp
class VideoClass { virtual bool SetMode(int w, int h, int bpp); virtual Surface* CreateSurface(...); };
class AudioClass { virtual void PlaySound(SoundID); virtual void SetVolume(float); };
```
Allows (theoretical) swap for other platforms — though only Win32 implemented.

## Data & Control Flow

### Initialization Sequence
```
WinMain
  → Registry::ReadSettings()
  → Video::SetMode() → DirectDraw cooperative level, primary surface
  → Audio::Init() → DirectSound primary buffer
  → Input::Init() → Mouse/keyboard acquisition
  → Timer::Init() → QueryPerformanceFrequency
  → File::Init() → Scan .mix archives
  → Network::Init() → DirectPlay enumerate
```

### Frame Loop Integration
```
Engine.Run_Frame()
  → Timer::StartFrame()
  → Input::Poll() → queues commands
  → Network::Receive() → CommQueueClass::Queue_Receive()
  → Engine logic...
  → Video::Present() → DirectDraw Flip/Blt
  → Timer::EndFrame() → sleep for target FPS
```

## Integration Points

### Consumers (Engine Core)
| Engine Module | Uses |
|---------------|------|
| `AUDIO.CPP` | `AudioClass`, `MixerClass` |
| `DISPLAY.CPP` | `VideoClass`, `DDRAW`, `PaletteClass` |
| `MOUSE.CPP`/`KEYBOARD.CPP` | Input classes |
| `CCFILE.CPP` | `FileClass`, `MIX` archive |
| `COMQUEUE.CPP` | `NetworkClass` (DirectPlay) |
| `TIMER.CPP` | `TimerClass` (frame pacing) |

### Shared with Tiberian Dawn
`TIBERIANDAWN/WIN32LIB/` has **nearly identical structure** — divergence only in:
- Video mode defaults (RA supports more resolutions)
- Audio track mapping (RA has more CD tracks)
- Registry keys (different app name)

## Key Files
| File | Role |
|------|------|
| `DDRAW.CPP/H` | DirectDraw 7 wrapper — surfaces, clipping, palette |
| `DIB.CPP/H` | Device-independent bitmap conversion |
| `MOUSE.CPP/H` | Cursor capture, position, button state |
| `KEYBOARD.CPP/H` | Key state array, hotkey translation |
| `AUDIO.CPP/H` | DirectSound — primary/secondary buffers, 3D |
| `MIXER.CPP/H` | Software mixing fallback |
| `FILE.CPP/H` | File open/read/seek, `.mix` archive mounting |
| `CCFILE.CPP/H` | `CCFileClass` — wrapper with buffering |
| `REGISTRY.CPP/H` | `RegKeyClass` — read/write settings |
| `TIMER.CPP/H` | `FrameTimerClass` — QPC-based fixed timestep |
| `CONNECT.CPP/H` | DirectPlay session management |
| `ALLOC.CPP/H` | Debug `new`/`delete` with guard pages |