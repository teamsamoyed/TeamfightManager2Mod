# Stable Native Mods (Build Once, Keep Working)

The stable mod API lets you build a native DLL **once** and keep using it across game updates. This is the recommended way to write new native Rust mods.

Looking for a specific function? The [Stable API Reference](stable-api-reference.md) lists every trait, method, value type, and enum with signatures, units, payload schemas, and examples. This page is the getting-started guide.

It differs from the classic native path (`mod-api`) in four ways:

- **No rebuilds after game updates.** The game keeps loading your module, and it works in both directions: an old module runs on a new game, and a module built against the newest SDK still runs on an older game.
- **Any Rust toolchain.** Stable Rust, any nightly — no pinned compiler, no SDK dependency downloads.
- **Any platform.** Windows, macOS, and Linux can all run native stable mods. See [Platform Support](#platform-support).
- **A smaller, contract-based API.** You call game functions through a versioned interface instead of touching internal game types directly. The classic path still exists for mods that need it, but those DLLs must be rebuilt for each game version.

## Getting the SDK

The stable SDK is a folder named `mod-sdk-stable` containing:

- `mod-api-stable/` — the API crate (plain Rust source, no dependencies).
- `template/` — a ready-to-build mod project.
- `README.txt` — the short version of this page.

It ships with the game on **every platform**, in your game install directory:

| Platform | Where |
|---|---|
| Windows | `mod-sdk-stable/`, next to `TFM2ModUploader.exe` |
| Linux | `mod-sdk-stable/`, next to the game executable |
| macOS | `TeamfightManager2.app/Contents/MacOS/mod-sdk-stable/` |

The SDK is plain Rust source with no dependencies, which is why it can ship everywhere — there is nothing in it compiled for one platform. (The classic SDK could not: it was a bundle of pre-compiled Windows `.rlib` files pinned to one nightly compiler.)

## Your First Stable Mod

1. Copy the `template` folder anywhere and rename it (the folder name becomes your module's file name).
2. Keep `mod-api-stable` next to it — the template references it as `../mod-api-stable`.
3. Edit `src/lib.rs`:

```rust
use mod_api_stable::{declare_stable_mod, LogLevel, StableHost, StableMod};

fn init(host: &StableHost) -> StableMod {
    host.log(LogLevel::Info, "hello from my stable mod");

    let mut decl = StableMod::new("my_mod_id");
    // Register content here (see the sections below).
    decl
}

declare_stable_mod!(init);
```

4. Build and install:

```
cargo build --release
```

Copy the built module out of `target/release/` into `mods/<my_mod_id>/`, renamed to your mod id plus its extension, together with a `mod.mod_info` file (same metadata format as every other mod — see [Mod Package](mod-package.md)).

| Building on | Cargo produces | Copy it in as |
|---|---|---|
| Windows | `my_mod.dll` | `my_mod.dll` |
| Linux | `libmy_mod.so` | `my_mod.so` |
| macOS | `libmy_mod.dylib` | `my_mod.dylib` |

Note the dropped `lib` prefix on Linux and macOS: the file name must match your mod id.

If you publish with `TFM2ModUploader.exe`, you can skip the manual build: the uploader detects a stable mod (a `Cargo.toml` that depends on `mod-api-stable`), builds it with your default toolchain, and even installs `mod-api-stable` next to your mod if it is missing (it looks in `mod-sdk-stable` beside the uploader, or wherever `TFM2_STABLE_SDK_DIR` points).

## What You Can Register

Everything goes through the `StableMod` builder in `init`:

| Call | Implement | What it does |
|---|---|---|
| `add_champion(...)` | `StableChampion` (+ `StableAction`, `StableEffectType`, `StablePassive`) | New champion, or [rework an existing one](override-existing-champions.md) by reusing its id |
| `add_item(...)` | `StableItem` | New item with runtime callbacks |
| `add_native_effect(name, ...)` | `StableEffectType` | Named effect referenced from `.data_champion` files via `DataEffectDef::Native` |
| `set_extension(...)` | `StableExtension` | Client lifecycle hooks: title/game UI, save data, mod commands/events |
| `set_server_extension(...)` | `StableServerExtension` | Authoritative server hooks: per-save data writes, client events |
| `add_draft_score_hook(...)` | `StableDraftHook` | Adjust ban/pick scoring, or decide the exact ban/pick (candidate identities included) |
| `add_item_build_hook(...)` | `StableItemBuildHook` | Adjust or fully decide which final items the AI builds — the way to get your own items actually built |
| `add_player_input_ai(...)` | `StablePlayerAi` | Replace final per-tick player inputs |
| `set_map_customizer(...)` | `StableMapCustomizer` | Edit the freshly built match map (towers, camps, lanes, nexus, fountains, wall/bush grids) before every match |
| `set_match_hook(...)` | `StableMatchHook` | Custom match logic on top of an existing mode: start/tick hooks plus your own end condition and winner |

During `init` you can also publish or consume runtime services shared between native mods with `host.register_service(...)` / `host.query_service(...)`.

Inside gameplay callbacks you receive a `StableSim` context: entities, players, damage/heal/shield/buff/CC, projectiles, kill log, debug drawing, direct mutation (HP/position/stats/gold, `force_end`), unit and projectile spawning, a delayed effect queue, and the live team-strategy document. Client extension hooks receive a `StableClient` context: scene kind (including NewGame options), full UI control, render-overlay drawing, audio, raw hotkeys, i18n, per-mod save data, mod commands/events, and management data reads (15 record kinds, champion info, game clock, head-to-head, current screen). Server hooks receive a `StableServerCtx`: per-mod save namespace, client event emission, settings and record documents (read/write), news injection, management event subscription, and forced transfers. The [Stable API Reference](stable-api-reference.md) covers all of it method by method.

### Full UI control

Every UI widget kind the game itself uses ("runner": label, button, checkbox, text_edit, slider, dropdown, image, tree_view, scroll_view, ...) is available:

- **Create** — `ui_spawn_source(parent, source)` takes the same source text `.ui` asset files use, children and styles included: `ctx.ui_spawn_source("body", "my_hint:label { text: \"hi\"; }")`. `ui_spawn_template` still loads prebuilt template assets.
- **Modify** — `ui_set_properties(path, "size: 24.; visible: true;")` applies any `.ui` property line (layout, colors, fonts, images, runner-specific settings) to an existing node. `ui_set_visible` / `ui_set_text` cover the two most common cases directly.
- **Interact** — runtime widget state is readable and writable: `ui_checkbox_selected` / `ui_set_checkbox_selected`, `ui_text_edit_text` / `ui_set_text_edit_text`, `ui_slider_ratio` / `ui_set_slider_ratio`, `ui_selectable_selected`, `ui_dropdown_selected_item`, or the generic `ui_state_json` / `ui_set_state_json`.
- **Observe** — `ui_register_click` for plain buttons, or `ui_register_path_events` + `ui_current_event` to receive **every** UI event on a path (clicks, checkbox toggles, text edit completion, tree view moves, ...) with a JSON payload.
- **Inspect / remove** — `ui_runner_name(path)` tells you what kind of widget a node is; `ui_remove_node(path)` deletes a subtree.

All of this is regular Rust — traits and safe wrapper types. You never write `unsafe` or FFI code yourself.

## Changing Rules, Maps, and Data (JSON Paths)

Large game structures (settings, the match map, team/athlete records) are exposed as **JSON documents** instead of hundreds of individual functions. You read or write any field by its dot-separated path; an empty path means the whole document, and numbers index arrays.

```rust
// Server hook: double kill gold for every later match in this save.
let gold = ctx.setting_get_i64(SettingTargetV1::GameSetting, "kill_gold").unwrap_or(0);
ctx.setting_set_json(SettingTargetV1::GameSetting, "kill_gold", &(gold * 2).to_string());

// Map customizer: read both nexus positions, then move the first tower's
// blue-side x coordinate (positions are [[blue_x, blue_y], [red_x, red_y]]).
let nexus = doc.get_json("nexus_pos");
doc.set_i64("towers.0.pos.0.0", 120_000);

// Server hook: rename a team (typed helpers exist for strings/ints/bools).
ctx.team_set_string(team_id, "name", "New Name");
```

Writes are validated by round-tripping the whole document through the game's own schema — a write that would produce an invalid document is rejected atomically and returns `false`, so you cannot corrupt a save or a match with a typo.

Combining these is how you build what feels like a **custom game mode** without one: settings JSON (rules and numbers) + a map customizer (geometry and objects) + a match hook (win condition, per-tick logic) + sim mutation (executing your rules). Match hooks and map customizers run inside the deterministic simulation — derive any randomness only from the `rng_seed` you are handed, never from wall clocks or `rand::random`.

## Platform Support

Native stable mods run on Windows, macOS, and Linux. What decides where *your* mod runs is simple: **the binaries you put in the package.** There is no platform field to fill in anywhere.

### Support is read off your files

The game looks at which module files your mod ships and derives the answer:

| Your package contains | Where the mod runs |
|---|---|
| No native module (assets, `.data_champion`, i18n, overrides only) | Everywhere. Data mods have no platform at all. |
| `my_mod.dll` | Windows |
| `my_mod.dll` + `my_mod.so` | Windows and Linux |
| `my_mod.dll` + `my_mod.so` + `my_mod.dylib` | All three |

Only `.dll`, `.so`, and `.dylib` count. Helper scripts you might ship (`.bat`, `.ps1`) are not loadable modules and say nothing about platform support.

This is deliberate: a declared list could drift out of sync with what you actually shipped and promise a platform whose binary is missing. Reading the files instead means the package cannot lie.

### You build each platform yourself

**A module can only be built on the platform it targets.** Plan for this before you promise support.

The Rust *compilation* is portable — nothing in `mod-api-stable` is platform-specific. The obstacle is the final link step, which needs that operating system's system libraries:

- **macOS (`.dylib`) requires a Mac.** Linking needs Apple's SDK, which Apple does not allow to be redistributed, so no tool can bundle it for you and no uploader can do this on your behalf. If you do not have a Mac, you cannot ship a macOS build.
- **Linux (`.so`) requires a Linux machine** — or a cross-linker setup you install yourself (a glibc sysroot via something like `cargo-zigbuild`). Rust's own compiler and linker already handle everything else; only the C runtime libraries are missing on a Windows box. A VM or WSL is usually the least trouble.
- **Windows (`.dll`) requires Windows** in practice, for the same class of reason.

So the realistic workflow is: build on the machines you have, ship those, and say so. Supporting only Windows is completely normal and nothing warns you about it.

### Shipping more than one platform

Put every binary you built into the same mod folder, side by side:

```
mods/my_mod/
  mod.mod_info
  my_mod.dll
  my_mod.so
  my_mod.dylib      (only if you actually built it on a Mac)
  ...your assets...
```

That is one Workshop item. Each player's game loads only the file matching their operating system and ignores the others. You do not need separate uploads per platform, and you never have to touch `mod.mod_info`.

Rebuild and re-upload only the platform you changed; the others keep working, because the stable ABI promise applies identically on all three.

### What players on an unsupported platform see

If someone enables your mod on an OS you did not build for, the game does **not** fail silently. It tells them, in the mod diagnostics popup, that the mod has no version for their operating system, and lists the platforms it does support. That message comes straight from your shipped files, so it is always accurate.

Their save and the rest of their mods are unaffected — your mod simply contributes nothing.

### Using the uploader

`TFM2ModUploader` builds for **the platform you run it on**, and names the output correctly for that platform. Run it on Windows and it produces `my_mod.dll`; the Linux build of your mod has to come from a Linux machine.

When it uploads, it reads your binaries and records the supported platforms in the Workshop item — the description gains a `Runs on:` line — so subscribers can see before installing whether the mod covers their system.

## Compatibility Rules

- Your save data, commands, and events are namespaced to your mod id automatically.
- Unknown enum codes can appear after game updates (new scenes, new tags). The wrappers surface them as `None` — ignore what you do not recognize.
- A module is only rejected when it **declares** that it needs a newer game than the one running (see below); updating the game then fixes it. Nothing else keeps a module from loading, in either direction. This holds identically on Windows, macOS, and Linux.
- ABI compatibility is not behavior compatibility: if the game changes, what your mod *does* can shift even though it still loads. Test after big game updates.

### New API, old games

The API grows by **appending**: new calls and callbacks are added, existing ones never change shape. So building against the newest SDK is always safe — your DLL still loads on older games, and a call into something the running game does not have simply returns `None` / `false` and does nothing. There is no migration step and nothing to guard.

The reference marks each addition with the game version that introduced it, e.g. **`0.5.4+`** — see [Added in 0.5.4](stable-api-reference.md#added-in-054).

If your mod would be *pointless* on an older game — it loads and then does nothing useful — say so, and players get an "update the game" diagnostic instead of an inert mod:

```rust
mod_api_stable::declare_stable_mod!(init, requires = 2);   // needs 0.5.4 or newer
```

Use this sparingly. A mod that merely *prefers* newer features should stay on the plain `declare_stable_mod!(init)` and let the individual calls degrade.

| `requires` | Minimum game version |
|---|---|
| (omitted) | any version with stable mod support |
| `2` | 0.5.4 |

## The Classic Path Is Going Away

The classic [`mod-api` path](native-rust-mods.md) is **deprecated**. Its SDK is supported **through game version 0.5 only — from 0.6 the classic SDK is no longer shipped or updated**. Classic DLLs are tied to one exact game build and must be rebuilt after every update even today.

**Strongly prefer the stable path for everything.** It now covers champions, items, effects, full UI control, render overlays, audio, hotkeys, save data, networking, management records and events, transfers, map/match customization, and both AI hook surfaces. If you hit something the stable contract genuinely cannot do, report it so the stable API can grow that function — do not start a new classic mod for it.
