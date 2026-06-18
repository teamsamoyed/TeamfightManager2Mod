# Troubleshooting

Most mod loading problems are shown in the diagnostics popup on the title screen. Check that first; it usually names the file or DLL that failed.

## The Mod Does Not Appear

Check:

- The mod folder is under `mods/`.
- The folder contains `mod.mod_info`.
- `mod.mod_info` is valid JSON.
- `version` looks like `0.1.0`.
- You restarted the game after adding the folder.

## The Mod Appears But Cannot Be Enabled

Check dependencies:

- The dependency mod is installed.
- The dependency version matches your requirement.
- Your `base` version requirement matches the installed game version. `base` refers to the Teamfight Manager 2 game version shown in the lower-right corner of the title screen, not to base-game asset files.

Installed dependencies are included automatically when the dependent mod is enabled. If the dependency is installed but the mod still cannot be enabled, check the diagnostics popup for a version mismatch or a load failure in the dependency mod.

Mod dependency version requirements use semantic versioning. For the special `base` dependency, the major version is `0` during Early Access and `1` for the full release line. Minor versions are for feature, content, data-format, or compatibility changes, while patch versions are for bug-fix-only updates.

## The Game Disables the Mod

The game disables a mod when it cannot safely load it. Common causes:

- `mod.mod_info` has invalid JSON.
- `mod.override_info` has invalid JSON.
- A native DLL failed to load.
- A native DLL registered the wrong mod id.

Fix the reported issue and restart the game.

## Text Shows as Missing or Empty

Check:

- Your i18n file is valid JSON.
- Your `mod.override_info` uses `"type": "merge"`.
- The target asset path is correct, for example `asset/base/text/champion`.
- The text lookup matches the JSON path:

```text
#asset/base/text/champion?description.my_mod_fire_mage.skill
```

## Images Do Not Load

Check:

- Asset paths do not include file extensions.
- The file is inside the mod folder.
- Spelling and capitalization match.
- Skill icons point to a valid PNG asset or sprite sheet tag.
- If you use a sheet tag, both `source#sheet` and `source#data` exist.
- If you wrote `.sprite_sheet` JSON by hand, the rectangles are normalized values from `0.0` to `1.0`.

## Aseprite Sprites Do Not Animate

Check:

- The `.aseprite` file has sprite user data with `"sheet_type": "Animation"` or `"sheet_type": "LayeredAnimation"`.
- The Aseprite timeline tags match the champion action names, such as `idle`, `attack`, `skill`, `skill2`, and `ult`.
- The layer names in the Aseprite user data match the actual layer names.
- A data champion using an animated source has `anim_prefix` set. Use `""` when the tags should be copied as-is.
- If you use a prefixed source, the tags really have that prefix, for example `eagle_idle` with `"anim_prefix": "eagle_"`.

## A Sprite Sheet Tag Shows the Wrong Image

Check:

- The tag name in JSON matches the tag in the sheet data.
- Manual `.sprite_sheet` rectangles use normalized values, not pixels.
- Manual `.fanim` rectangles use pixel values, not normalized values.
- Aseprite `Sheet` and `PackedSheet` tags are named `<layer_name>_<frame_number>`, such as `fire_skill_0`.

## A Data Champion Does Not Work

Check:

- The file extension is `.data_champion`.
- The champion has `attack`, `skill`, and `skill2`.
- Enum values are spelled exactly, such as `Targeting`, `Enemy`, and `BaseAttack`.
- For a new champion, the champion `id` is unique. For an existing champion rework, the `id` exactly matches the base champion id.
- Description keys point to text that has been merged into i18n.
- If `sprite` points to an animated source, `anim_prefix` is present.
- If `sprite` points to a PNG, `anim_prefix` is not needed.

## An Existing Champion Rework Does Not Apply

Check:

- The `.data_champion` or `ModChampionInfo::id()` uses the exact base champion id.
- For JSON reworks, the file extension is `.data_champion` and the mod loads without diagnostics errors.
- For native Rust reworks, the DLL uses the current SDK and registers the champion with `replace_champion`.
- You restarted the game after changing data files or rebuilding the DLL.
- In multiplayer, every player has the same mod enabled and native DLLs were built with a compatible SDK.
- If both JSON and native Rust register the same id, the native Rust runtime takes priority.
- Name and description changes are provided through an i18n merge into `asset/base/text/champion`; otherwise the old text may still appear even though gameplay data changed.

## A Native DLL Does Not Load

Check:

- The DLL is in the mod folder.
- The DLL exports `tfm2_mod_entry` by using `declare_mod!`.
- `ModRegistration::new("...")` uses the same id as the mod folder.
- The DLL was built with the matching Mod SDK.
- The DLL targets Windows x86_64 MSVC.
- The DLL was built for the game version you are running.
- The DLL was built from the same profile you are testing. Use a release DLL with the release game package.
- If diagnostics mention an API version symbol or version mismatch, rebuild with the current Mod SDK.

## A Native AI Hook Does Not Run

Check:

- The native DLL loads without diagnostics errors.
- The hook is registered in `ModRegistration` with `add_draft_score_hook` or `add_player_input_ai`.
- The mod is enabled and the game was restarted after replacing the DLL.
- `ModPlayerInputAi::matches()` returns `true` for the player you expect.
- The hook's `priority()` is not being overridden by a later higher-priority hook.
- A returned player `Input` is valid for the current frame. Invalid replacement inputs are ignored.
- The behavior is being observed in a simulation that uses `GameRunner`, such as a match or title-screen simulated match.

For player input AI, start by returning `PlayerInputDecision::Pass` until your trigger condition is true, then use helpers such as `ctx.get_run_away_input()`, `ctx.get_recall_input()`, and `ctx.is_safe_to_recall()` before constructing manual inputs.

## Reading Game Data Fails to Compile

Check:

- You are using a current Mod SDK. Older SDKs may not expose the newest `ClientData` lookup helpers.
- Your code runs from a `ModExtension` and first matches `Scene::InGame { data }`.
- The ID exists in the current save. Lookup helpers return `None` when the data is not present on the client.
- You let returned `Ref<'_, T>` values go out of scope before calling APIs that need a mutable client data borrow, such as mod save-data writes.
- If you need a type name in annotations, import it from `mod_api::*`; common internal types such as `Athlete`, `Team`, `MatchInfo`, `League`, `Tournament`, and `ChampionInfo` are re-exported.

## Mod Save Data Does Not Persist

Check:

- The native DLL loads without diagnostics errors.
- You access save data from an in-game scene, usually `Scene::InGame { data }`.
- Your `mod_id` matches the mod folder name and `ModRegistration::new(...)`.
- Write helpers such as `mod_save_set_string` return `true`.
- The key and mod id are non-empty, under 128 bytes, and contain no NUL bytes.
- The value is not larger than 1 MiB.
- In multiplayer league saves, the host is the one writing the value.
- You are not clearing the namespace or overwriting the value every frame.

Remember that mod save data is stored in the game save. After writing a value, use the normal save/autosave flow and reload the same save slot to test persistence.

## A Native Service Dependency Does Not Work

If a native mod depends on another native mod's runtime service, check:

- The provider mod is listed in the consumer's `mod.mod_info` dependencies.
- The provider mod is installed.
- The provider DLL loads without diagnostics errors.
- The provider calls `GameCtx::register_service` during its entry function.
- The consumer uses the correct provider mod id, service id, and version requirement.
- The provider and consumer use the same `#[repr(C)]` service vtable layout.

If the consumer has a fallback path, it may still load but behave differently. For example, an item price that should include a provider bonus may stay at its fallback value when service lookup fails.

## Workshop Upload Fails

Check:

- Steam is running.
- `TFM2ModUploader.exe` is running from the game folder, next to the Steam DLLs.
- You have access to the game app.
- You accepted the Steam Workshop legal agreement if Steam asks for it.
- `mod.mod_info` parses correctly.
- `preview.png`, `preview.jpg`, `thumbnail.png`, or `thumbnail.jpg`, if present, is a valid image.
- If you are updating an existing item, `mod.workshop_id` still points to the correct Workshop item.
- Native Rust mods have the matching Mod SDK installed before using the build option.

If the mod already has a compiled DLL and you only want to upload it, uncheck **Build native Rust code before uploading**.

## Native Build Fails in the Uploader

Check:

- The matching Mod SDK is next to `TFM2ModUploader.exe`.
- `mod-sdk/deps` contains the prebuilt `mod-api` files.
- Rust is installed and both `cargo` and `rustc` can run.
- The mod has either `Cargo.toml` or `src/lib.rs`.
- If using external crates, Cargo can download them or they already exist in your Cargo cache.
- If the mod is inside another Cargo workspace, add an empty `[workspace]` table to the mod's `Cargo.toml` or move it outside that workspace.
- Do not add `mod-api` to `[dependencies]` for the public SDK build path; the uploader injects the matching prebuilt `mod_api` crate.
- The SDK version matches the game version you are targeting.
