# CnCTDRAMapEditor/Steamworks.NET/ — Vendored Steamworks.NET Bindings

## Responsibility
**Complete Steamworks SDK P/Invoke bindings** for C#. Enables Steam Workshop integration (UGC publish/query/download), user stats, leaderboards, networking, and authentication. Vendored directly in repo — not a NuGet package.

## Design Patterns

### Auto-Generated Bindings (`autogen/`)
**Machine-generated** from Steamworks SDK headers (`steam_api.h`, `steam_api_flat.h`). Do not edit manually.
- `isteam*.cs` — Interface definitions (`ISteamUGC`, `ISteamUserStats`, `ISteamRemoteStorage`, etc.)
- `SteamCallbacks.cs` — Callback structs (`UGCQueryCompleted_t`, `SubmitItemUpdateResult_t`, etc.)
- `SteamConstants.cs` — `k_uAPICallInvalid`, `k_unEnumeratePublishedFilesMaxResults`, etc.
- `SteamEnums.cs` — `EItemState`, `EItemPreviewType`, `EUserUGCList`, etc.
- `SteamStructs.cs` — `PublishedFileId_t`, `UGCHandle_t`, `SteamAPICall_t`, etc.
- `NativeMethods.cs` — `DllImport("steam_api.dll")` for all flat API functions

### Type Definitions (`types/`)
Organized by Steam interface:
| Namespace | Contents |
|-----------|----------|
| `Steamworks.NET.types.SteamTypes` | `AppId_t`, `DepotId_t`, `AccountID_t`, `SteamAPICall_t`, `RTime32` |
| `Steamworks.NET.types.SteamClientPublic` | `CSteamID`, `HAuthTicket`, `CGameID` |
| `Steamworks.NET.types.SteamUGC` | `UGCQueryHandle_t`, `UGCUpdateHandle_t`, `PublishedFileId_t`, `PublishedFileUpdateHandle_t` |
| `Steamworks.NET.types.SteamUserStats` | `SteamLeaderboard_t`, `SteamLeaderboardEntries_t` |
| `Steamworks.NET.types.SteamRemoteStorage` | `UGCFileWriteStreamHandle_t`, `UGCHandle_t`, `PublishedFileId_t` |
| `Steamworks.NET.types.SteamMatchmaking` | `HServerListRequest`, `HServerQuery` |
| `Steamworks.NET.types.SteamNetworking` | `SNetListenSocket_t`, `SNetSocket_t` |
| ... | (18 total subdirectories) |

### Callback Dispatcher
**`CallbackDispatcher.cs`** — Routes native Steam callbacks to managed delegates:
```csharp
CallbackDispatcher.Register<UGCQueryCompleted_t>(callback => {
    // Marshaled on threadpool
    OnUGCQueryCompleted(callback);
});
SteamAPI.RunCallbacks(); // Called from MainForm message pump
```

### High-Level Wrapper
**`Steam.cs`** — Static facade:
- `Steam.Init()` / `Steam.Shutdown()` — Loads `steam_api.dll`, initializes API
- `Steam.UGC` → `ISteamUGC` instance
- `Steam.UserStats` → `ISteamUserStats` instance
- `Steam.Friends` → `ISteamFriends` instance
- `Steam.Matchmaking` → `ISteamMatchmaking` instance
- `Steam.RunCallbacks()` — Pumps callback queue

### Interop Helpers
- `InteropHelp.cs` — `PtrToStringUTF8`, `StringToPtrUTF8`, `StructureToPtr`, `TryMarshal`
- `Packsize.cs` — Structure packing verification (matches SDK `PACKSIZE`)

## Data & Control Flow

### Initialization
```
Program.cs → Steam.Init()
  → NativeMethods.SteamAPI_Init() → loads steam_api.dll
  → CallbackDispatcher.Init() → registers all callback types
  → Returns true if Steam client running
```

### Workshop Publish Flow (via `Utility/SteamworksUGC.cs`)
```
SteamworksUGC.CreateItem(AppId, EWorkshopFileType.Community)
  → Steam.UGC.CreateItem() → returns UGCUpdateHandle_t
  → Steam.UGC.StartItemUpdate(handle) 
  → Steam.UGC.SetItemTitle/Description/Tags/Content/Preview
  → Steam.UGC.SubmitItemUpdate(handle, changeNote)
    → Callback: SubmitItemUpdateResult_t → PublishedFileId_t
  → Steam.UGC.SetItemVisibility(PublishedFileId_t, ERemoteStoragePublishedFileVisibility.Public)
```

## Integration Points

### Dependencies
- `steam_api.dll` (native) — Copied to output via `.csproj` `<Content Include="Steamworks.NET\redist\steam_api.dll" />`
- .NET Framework 4.6.2
- x64 only (`STEAMWORKS_X64` define)

### Consumers
| Consumer | Uses |
|----------|------|
| `Utility/SteamworksUGC.cs` | High-level workshop operations |
| `Dialogs/SteamDialog.cs` | Publish UI → calls `SteamworksUGC` |
| `MainForm.cs` | `Steam.Init()` at startup, `Steam.RunCallbacks()` in message loop |

## Key Files
| File | Role |
|------|------|
| `Steam.cs` | Static facade — `Init`, `Shutdown`, `RunCallbacks`, interface accessors |
| `CallbackDispatcher.cs` | Callback routing — native → managed |
| `InteropHelp.cs` | Marshaling utilities |
| `NativeMethods.cs` | All `DllImport` signatures |
| `autogen/isteam*.cs` | **Auto-generated** interface definitions |
| `autogen/SteamCallbacks.cs` | **Auto-generated** callback structs |
| `autogen/SteamStructs.cs` | **Auto-generated** handle/ID types |
| `types/*/*.cs` | **Auto-generated** typed handles per interface |