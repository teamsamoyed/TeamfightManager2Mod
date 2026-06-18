# Teamfight Manager 2 Modding Guide

Welcome to the Teamfight Manager 2 modding guide.

Mods can add new champions, change text and images, replace or merge game assets, add UI pieces, and, for more advanced projects, run custom Rust code through a native DLL.

If this is your first mod, start with a data-only champion. You can make one with JSON files and image assets, without installing the Mod SDK or compiling code.

## What You Can Make

### Data and asset mods

These are the easiest mods to build and share.

- Add champions with `.data_champion` files.
- Rework existing champions by reusing their base champion id in a `.data_champion` file.
- Add custom icons, sprites, UI layouts, and text.
- Merge new translations into the game's text files.
- Override existing assets such as sprites or JSON data.

You do not need the Mod SDK for this kind of mod.

### Database packs

Database packs are Workshop sharing packages for custom database files. They use `database_pack.info`, can contain one or more database files directly in the package folder, and are downloaded/shared through Steam Workshop without being automatically enabled as in-game mods.

### Native Rust mods

Native mods are for things that need custom code.

- Add champions with custom simulation logic.
- Rework existing champions with custom runtime logic through `replace_champion`.
- Add items with runtime callbacks.
- Add UI or scene behavior through lifecycle hooks.
- Add server-side management hooks for save/game-state logic.
- Send client-to-server mod commands and receive server-to-client mod events.
- Read current save data such as teams, athletes, matches, leagues, and champion info.
- Adjust ban/pick scoring and replace final player AI inputs.
- Store custom per-save data for your mod.
- Expose reusable runtime services that other native mods can depend on.
- Build deeper experiments that cannot be described with JSON alone.

Native mods need the Mod SDK and should be rebuilt for the game version they target.

## Start Here

- [Getting Started](docs/getting-started.md)
- [Mod Package Structure](docs/mod-package.md)
- [Data-Only Champions](docs/data-champion.md)
- [Data Champion Schema](docs/data-champion-schema/index.md)
- [Assets and Sprite Sheets](docs/assets-and-sprite-sheets.md)
- [Asset Overrides and i18n](docs/asset-overrides-and-i18n.md)
- [Native Rust Mods](docs/native-rust-mods.md)
- [Native Mod API Reference](docs/native-mod-api-reference.md)
- [Native AI Hooks](docs/native-ai-hooks.md)
- [Mod Save Data](docs/mod-save-data.md)
- [Workshop Upload](docs/workshop-upload.md)
- [Troubleshooting](docs/troubleshooting.md)

## A Small Mod Folder

A simple champion mod might look like this:

```text
my_mod/
  mod.mod_info
  thumbnail.png
  champion/
    my_champion.data_champion
  text/
    champion.i18n
  icons/
    my_champion_skill.png
    my_champion_skill2.png
    my_champion_ult.png
  champions/
    my_champion.aseprite
```

The folder name becomes the mod id. Files inside the folder are referenced as `asset/<mod_id>/...`.

For example:

```text
mods/my_mod/icons/my_champion_skill.png
```

is used as:

```text
asset/my_mod/icons/my_champion_skill
```

File extensions are left out when writing asset paths.

## Recommended First Steps

1. Create a folder under `mods/`.
2. Add `mod.mod_info`.
3. Add one `.data_champion` file.
4. Reuse a base-game sprite while testing.
5. Add your own icons and text after the champion appears in-game.
6. Use `TFM2ModUploader.exe` to publish the finished mod to Steam Workshop.
7. Move to a native Rust mod only when JSON effects are not enough.

The game shows mod loading issues on the title screen, so if something does not appear, check the diagnostics popup first.

## Sharing on Steam Workshop

The normal upload tool is `TFM2ModUploader.exe`, included with the game package. It lets you choose a mod or database pack folder, check its metadata, choose Workshop visibility, write a change note, and publish or update the Workshop item.

Most mods and database packs only need this uploader. If your mod contains native Rust source code in `src/lib.rs`, install the matching Mod SDK next to the uploader before using the build option.
