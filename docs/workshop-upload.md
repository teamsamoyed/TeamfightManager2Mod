# Steam Workshop Upload

Teamfight Manager 2 includes `TFM2ModUploader.exe` for publishing mods and database packs to Steam Workshop.

Put your package folder together, open the uploader, choose the folder, check the details, and publish.

## Before You Upload

Make sure Steam is running and logged in with the account that should own the Workshop item.

A normal game mod folder uses `mod.mod_info`:

```text
my_mod/
  mod.mod_info
  thumbnail.png
```

A database pack folder uses `database_pack.info` directly in the package folder:

```text
my_database_pack/
  database_pack.info
  league_2026.tfm2db
  fantasy_rosters.tfm2db
  thumbnail.png
```

The database pack folder itself is the upload package. Do not add another required inner `packs/` folder unless your own importer expects that layout after download.

`mod.mod_info` and `database_pack.info` provide the title, author, version, and description shown by the uploader. `thumbnail.png` can also be used as the Workshop preview image if you do not provide a separate `preview.png`.

For the best Workshop page, add:

```text
my_mod/
  preview.png
```

`preview.jpg` and `thumbnail.jpg` are also accepted.

## Opening the Uploader

In the game folder, run:

```text
TFM2ModUploader.exe
```

The uploader uses Steam through the same game app, so keep it next to the game executable and Steam DLLs from the game package.

## Viewing Base Game Data

Packaged game builds keep the base game assets in `bundle.game_data` next to the game executable and uploader.

Use the uploader's **Base Bundle** section when you want to inspect those files while making a mod:

1. Run `TFM2ModUploader.exe` from the game folder.
2. Make sure `bundle.game_data` is in the same folder as the uploader.
3. Click **Unpack Base Bundle**.

The tool creates:

```text
mods/base_unpacked/
```

The unpacked folder uses the same layout as the game's base asset files. This is separate from the `base` version dependency in `mod.mod_info`. For example, the bundled asset `asset/base/text/ui` is written as:

```text
mods/base_unpacked/text/ui.i18n
```

Use `base_unpacked` as a read-only reference while building your own mod. Do not edit it as your mod source, because unpacking again will replace that folder.

## Building Without Uploading

To test a mod locally you do not need to publish anything. **Build Only (No Upload)** runs the same steps an upload runs — package checks, the native Rust DLL build when the mod has source, and staging the exact file set an upload would send — and stops before any Steam call. Steam does not even need to be running.

1. Click **Browse** and choose your mod folder.
2. Click **Build Only (No Upload)**.

A built DLL lands inside the mod folder itself, so if the folder already lives under the game's `mods/` folder it is immediately playable — iterate with build-only until the mod is ready, then publish once. The log also prints the staged package folder, which holds exactly the files an upload would ship (useful for checking what gets included).

The same thing is available from the command line:

```text
workshop-uploader build --path mods/my_mod
```

`--skip-build` skips the DLL build step, same as the upload command.

## Publishing a New Package

1. Click **Browse** and choose your mod or database pack folder.
2. Check the package type, name, folder id, author, version, preview image, and Workshop item state.
3. Choose a visibility:
   - **Public**: visible to everyone.
   - **Friends only**: visible to Steam friends.
   - **Private**: visible only to you.
   - **Unlisted**: accessible by link, but not listed publicly.
4. Write a short change note, such as `Initial upload`.
5. Click **Publish to Steam Workshop**.

On the first upload, Steam creates a new Workshop item. The uploader then saves an id file inside your package folder:

- Mods use `mod.workshop_id`.
- Database packs use `database_pack.workshop_id`.

Keep that file. It is how the uploader knows which Workshop item to update next time.

## Updating an Existing Package

After the first upload, the same package folder should contain its Workshop id file:

```text
my_mod/
  mod.workshop_id
```

```text
my_database_pack/
  database_pack.workshop_id
```

Open `TFM2ModUploader.exe`, choose the same folder, write a change note, and click **Update Workshop Item**.

Do not delete the Workshop id file unless you intentionally want to publish the package as a new Workshop item. If you lose it, you may need to recover the item id from the Workshop page URL and recreate the file.

Example:

```json
{
  "published_file_id": 3725617184
}
```

## What Gets Uploaded

The uploader prepares a temporary copy of your package and uploads that copy to Steam.

For mods, it includes normal mod files:

- `mod.mod_info`
- `mod.override_info`
- compiled native modules (`.dll`, `.so`, `.dylib`) — every platform binary you placed in the folder, so one Workshop item can serve Windows, Linux, and macOS players
- JSON and data files
- PNG/JPG images
- Aseprite files
- manual sprite sheet files
- UI layouts
- text/i18n files
- thumbnail and preview images

For database packs, it includes `database_pack.info` and the normal files in that folder, such as one or more custom database pack files and preview images.

It skips files that should not be part of the Workshop download:

- `mod.workshop_id`
- `database_pack.workshop_id`
- `src/`, `target/`, `Cargo.toml`, and `Cargo.lock` for normal mod packages

This means native Rust source code and build output are not uploaded. Only the compiled modules are.

For code mods, the item description also gains a `Runs on:` line listing the platforms your binaries cover. It is derived from the files you shipped, so it always matches reality — see [Platform Support](stable-native-mods.md#platform-support) for how to build for more than one platform.

## Database Packs

Database packs are for Workshop download and sharing. They are not automatically enabled through the in-game Mods menu just because they were downloaded.

Use this structure when you want one Workshop item to contain one or more database files:

```text
my_database_pack/
  database_pack.info
  league_2026.tfm2db
  fantasy_rosters.tfm2db
  preview.png
```

`database_pack.info` uses the same basic fields as `mod.mod_info`:

```json
{
  "name": "2026 League Database Pack",
  "author": "Your Name",
  "version": "1.0.0",
  "description": "Custom databases for the 2026 league setup."
}
```

The uploader detects `database_pack.info`, shows the package type as **Database Pack**, and creates or updates a Workshop item for that folder. After the first upload, keep `database_pack.workshop_id` so later uploads update the same item.

## Native Rust Mods and the SDK

Most mods do not need the Mod SDK. JSON, image, Aseprite, text, UI, and data-only champion mods can be uploaded with `TFM2ModUploader.exe` alone.

Native Rust mods are different. The uploader treats a mod as a code mod when the selected folder contains either:

```text
Cargo.toml
```

or:

```text
src/lib.rs
```

If **Build native Rust code before uploading** is checked, it tries to build the DLL before uploading.

For that build step, you need:

- the matching Teamfight Manager 2 Mod SDK
- a working Rust toolchain with `cargo` and `rustc`
- the SDK folder placed next to `TFM2ModUploader.exe`
- internet access or a populated Cargo cache if your Cargo mod depends on crates.io packages

Recommended layout:

```text
TeamfightManager2.exe
TFM2ModUploader.exe
steam_api64.dll
bundle.game_data
mod-sdk/
  deps/
  native/
  build_mod.bat
  build_mod_cargo.ps1
  rust-toolchain.toml
```

Build behavior:

- `Cargo.toml` present: the uploader runs a Cargo release build, injects the SDK's prebuilt `mod_api` crate, and supports external crates from normal Cargo dependency sources.
- Only `src/lib.rs` present: the uploader uses the older direct `rustc` build with only `mod_api` and `std`.

For Cargo mods, use a normal library crate:

```toml
[package]
name = "my_mod"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
rand = "0.8"
```

Do not add `mod-api` to `[dependencies]` in the public SDK workflow. The uploader injects the matching SDK build automatically.

When the SDK is present, the uploader builds your mod DLL into the selected mod folder, then uploads the finished runtime files. It skips build-only/source paths such as `src/`, `target/`, `Cargo.toml`, and `Cargo.lock`. If you already built the DLL yourself, you can uncheck the build option and upload the existing files.

The SDK should match the game version you are targeting. Rebuild native mods after game updates when a new SDK is released.
## Managing Workshop Items

Use one local package folder as the source of truth for each Workshop item.

Good habits:

- Keep `mod.workshop_id` or `database_pack.workshop_id` in your working copy after the first upload.
- Update the `version` field in `mod.mod_info` or `database_pack.info` when you make a release.
- Use clear change notes so subscribers know what changed.
- Test the mod or database pack locally before publishing an update.
- Keep a backup or source repository for your package folder.

If you are publishing a template for other people to copy, do not include your Workshop id file. Otherwise their uploads may try to update your Workshop item.

## Steam Legal Agreement

Steam may ask you to accept the Workshop legal agreement before the first upload can finish.

Open:

```text
https://steamcommunity.com/sharedfiles/workshoplegalagreement
```

After accepting it, run the upload again.