# Command & Conquer Remastered Collection

This repository includes source code for TiberianDawn.dll, RedAlert.dll and the Map Editor for the Command & Conquer Remastered Collection. This release provides support to the [Steam Workshop](https://steamcommunity.com/workshop/browse/?appid=1213210) for the C&C Remaster Collection.

## Dependencies

The following dependencies must be installed to successfully build the solution;

- Windows 8.1 SDK
- MFC for Visual Studio C++

## Compiling (Win32 Only)

To use the compiled binaries, you must own the game. The C&C Remastered Collection is available for purchase on [EA App](https://www.ea.com/games/command-and-conquer/command-and-conquer-remastered/buy/pc) or [Steam](https://store.steampowered.com/app/1213210/Command__Conquer_Remastered_Collection/).

The quickest way to build all configurations in the project is open [CnCRemastered.sln](CnCRemastered.sln) in Microsoft Visual Studio (we recommend using 2017 as later versions report an error due to a packing mismatch with the Windows SDK headers) and select "Build" from the toolbar, then select "Batch Build". Click the "Select All" button, then click the "Rebuild" button.

When the solution has finished building, the compiled binaries can be found in the newly created `bin` folder in the root of the repository.

## Contributing

This repository will not be accepting any contributions (pull requests, issues, etc). If you wish to create changes to the source code and encourage collaboration, please create a fork of the repository under your GitHub user/organization space.

## Support

This repository is for preservation purposes only and is archived without support.

## License

This repository and its contents are licensed under the GPL v3 license, with additional terms applied. Please see [LICENSE.md](LICENSE.md) for details.

## ENFORCED POLICIES (Updated 2026-10-19)

- **Git / CI** — All builds and commits must go through the local **Forgejo** server (`https://local.lan`) using credentials from `~/APIKEYS.md` (`FORGEJO_API_TOKEN`, `FORGEJO_USER=hyperion`). Push is enforced via `.github/workflows/rust-ci.yml` (and equivalent Forgejo runners) before any merge.
- **Semantic Versioning** — Every crate in `RUST/` must declare a strict SemVer version (e.g. `1.2.3`) in its `Cargo.toml` (via `version.workspace` or explicit `version`). No non-SemVer versions allowed.
- **Documentation** — Every crate must maintain:
  - `README.md` (overview, build instructions, documentation links)
  - `CHANGELOG.md` (Keep a Changelog format, linked to SemVer releases)
  - `codemap.md` (architecture reference, already present in most crates)
- See `README.md` and `RUST/Cargo.toml` for workspace-level version rules.
