# Getting Started

This guide creates a small local mod folder and gets it ready for the in-game Mods menu.

## 1. Create a Mod Folder

Create a folder under the game's `mods` directory:

```text
mods/my_mod/
```

In a development checkout, use the repository's `mods/` folder. In a packaged game build, use the `mods/` folder next to the game executable. If there is no `mods` folder yet, create one.

The folder name is important because it becomes the mod id. Use a stable lowercase name such as `my_mod`, `new_champions`, or `balance_pack`.

## 2. Add `mod.mod_info`

Create this file:

```text
mods/my_mod/mod.mod_info
```

Example:

```json
{
  "name": "My Mod",
  "author": "Your Name",
  "version": "0.1.0",
  "description": "Adds a new champion.",
  "last_updated": "2026-05-14",
  "dependencies": [
    {
      "mod_id": "base",
      "version": ">=0.1.0"
    }
  ]
}
```

Use a version number like `0.1.0`, `1.0.0`, or `1.2.3`.

Mod dependency version requirements follow semantic versioning. The `base` dependency is the game version your mod is compatible with, not a dependency on base-game asset files. Players can check the current game version in the lower-right corner of the title screen.

For the special `base` dependency, the version parts mean:

- `major`: `0` during Early Access, `1` for the full release line.
- `minor`: increases when features, game content, or compatibility-affecting behavior changes.
- `patch`: increases for bug-fix-only updates.

For example, `">=0.1.0"` means the mod expects Teamfight Manager 2 version `0.1.0` or newer.

## 3. Add Content

For your first mod, add a data-only champion:

```text
mods/my_mod/champion/my_champion.data_champion
```

See [Data-Only Champions](data-champion.md) for a complete example you can adapt. When you need every available field and effect type, use [Data Champion Schema](data-champion-schema/index.md).

For champion names and skill descriptions, add:

```text
mods/my_mod/text/champion.i18n
```

For icons or sprites, add PNG or Aseprite files anywhere under your mod folder and reference them with `asset/my_mod/...`. If you are making an animated sprite or a packed icon sheet, read [Assets and Sprite Sheets](assets-and-sprite-sheets.md) before naming the files.

If you want to inspect the packaged base-game files, run `TFM2ModUploader.exe` and use **Unpack Base Bundle**. It reads the local `bundle.game_data` file next to the uploader and writes a reference copy to `mods/base_unpacked`. See [Workshop Upload](workshop-upload.md#viewing-base-game-data) for details.

## 4. Enable the Mod

Launch the game, open the Mods menu from the title screen, enable your mod, and restart the game.

Restarting is the safest way to test changes because mod metadata, native DLLs, and many startup assets are loaded when the game starts.

## 5. Read the Diagnostics Popup

If a mod fails to load cleanly, the game shows a diagnostics popup on the title screen. It usually points to the file that needs attention, such as invalid JSON, a missing dependency, a broken override, or a native DLL problem.

## 6. Publish It

When the mod works locally, open `TFM2ModUploader.exe` from the game folder.

Choose your mod folder, check the information from `mod.mod_info`, choose a visibility, write a change note, and publish it to Steam Workshop. The uploader creates `mod.workshop_id` after the first upload; keep that file so later uploads update the same Workshop item.

If you are sharing custom database files rather than an in-game mod, create a folder with `database_pack.info` instead of `mod.mod_info`. The uploader recognizes it as a **Database Pack** and creates `database_pack.workshop_id` after the first upload.

See [Workshop Upload](workshop-upload.md) for the full publishing flow.

## A Good First Mod

Start simple:

1. Copy the package layout from [Data-Only Champions](data-champion.md).
2. Change the champion `id`, stats, and text keys.
3. Reuse a base champion sprite at first.
4. Launch the game and confirm the champion appears.
5. Add custom icons and visuals after the basic version works.
6. Upload with `TFM2ModUploader.exe` when you are ready to share it.
