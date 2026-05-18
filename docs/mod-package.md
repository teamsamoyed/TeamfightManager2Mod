# Mod Package Structure

A mod is a folder. The folder name is the mod id.

Here is a larger example:

```text
mods/example/
  mod.mod_info
  mod.override_info
  thumbnail.png
  preview.png
  example.dll
  champion/
    eagle.data_champion
  text/
    champion.i18n
    item.i18n
    ui.i18n
  icons/
    eagle_skill.png
    eagle_skill2.png
    eagle_ult.png
  sprite/
    eagle.png
    eagle.aseprite
    spell_icons#sheet.png
    spell_icons#data.sprite_sheet
  ui/
    layout/
      test_popup.ui
```

Only `mod.mod_info` is needed for the mod to show up in the Mods menu. Add the other files as your mod needs them.

## Asset Paths

Files inside a mod folder are referenced as:

```text
asset/<mod_id>/<relative_path_without_extension>
```

Examples:

```text
mods/example/text/champion.i18n      -> asset/example/text/champion
mods/example/icons/eagle_skill.png       -> asset/example/icons/eagle_skill
mods/example/ui/layout/test_popup.ui -> asset/example/ui/layout/test_popup
```

Aseprite files and manual sprite sheets can expose related assets through `#` suffixes:

```text
asset/example/aseprite_resources/champions/ghoul_king#sheet
asset/example/aseprite_resources/champions/ghoul_king#anim
asset/example/sprite/spell_icons#sheet
asset/example/sprite/spell_icons#data
```

`#sheet` is the image atlas, `#anim` is animation data, and `#data` is still-image tag data. See [Assets and Sprite Sheets](assets-and-sprite-sheets.md) before making custom animated sprites or icon sheets.

## `mod.mod_info`

`mod.mod_info` describes the mod:

```json
{
  "name": "Example Mod",
  "author": "TeamSamoyed",
  "version": "1.1.0",
  "description": "Adds example champions, items, text, and UI.",
  "last_updated": "2026-05-13",
  "dependencies": [
    {
      "mod_id": "base",
      "version": ">=0.1.0"
    }
  ]
}
```

Fields:

- `name`: Name shown in the Mods menu.
- `author`: Author name.
- `version`: Mod version, such as `0.1.0`.
- `description`: Short description shown to players.
- `last_updated`: Date or short update text.
- `dependencies`: Other mods or game-version requirements that must be satisfied for this mod to load.

## `base` Game Version Dependency

`base` is a special dependency id for the Teamfight Manager 2 game version. It is not the base-game asset folder and does not mean your mod depends on `asset/base/...` files.

Players can check the installed game version in the lower-right corner of the title screen. Use the `base` dependency to declare which game versions your mod supports:

```json
{
  "mod_id": "base",
  "version": ">=0.1.0"
}
```

Mod dependency version requirements follow semantic versioning. For the special `base` dependency, the `major.minor.patch` parts mean:

- `major`: `0` during Early Access, `1` for the full release line.
- `minor`: increases when features, game content, data formats, or compatibility-affecting behavior changes.
- `patch`: increases for bug-fix-only updates.

Most mods should include a `base` requirement so players get a clear diagnostics message if they try to load the mod on an unsupported game version.

Other dependencies are normal mod dependencies. They are also used for load order. When a player enables a mod, installed dependencies are included automatically and loaded first. If a required dependency is missing or its version does not match, the dependent mod is disabled and the game shows a diagnostics message.

Use dependencies for reusable native service mods too. For example, a native DLL consumer that calls a provider's runtime service should declare the provider in `mod.mod_info`:

```json
{
  "mod_id": "service_provider",
  "version": ">=1.0.0, <2.0.0"
}
```

## Database Pack Workshop Packages

A database pack package is a Workshop sharing folder, not a normal in-game mod. It uses `database_pack.info` instead of `mod.mod_info`, so it does not appear in the Mods menu automatically.

Use this structure when you want to share one or more database files through Steam Workshop:

```text
mods/my_database_pack/
  database_pack.info
  league_2026.tfm2db
  fantasy_rosters.tfm2db
  thumbnail.png
  preview.png
```

The folder name is the package id. Put the database files directly in that folder. Do not add another required inner folder unless your own importer expects that exact layout after download.

`database_pack.info` uses the same basic display fields as `mod.mod_info`:

```json
{
  "name": "2026 League Database Pack",
  "author": "Your Name",
  "version": "1.0.0",
  "description": "Custom databases for the 2026 league setup."
}
```

`TFM2ModUploader.exe` detects `database_pack.info`, shows the package type as **Database Pack**, uploads the files in that folder, and writes `database_pack.workshop_id` after the first upload.

## Optional Files

- `thumbnail.png`: Image shown in the in-game Mods menu and accepted as a Workshop preview fallback.
- `preview.png` or `preview.jpg`: Image used as the Steam Workshop preview. If these are missing, the uploader can fall back to `thumbnail.png` or `thumbnail.jpg`.
- `mod.override_info`: Asset merge and override rules for normal mods.
- `<mod_id>.dll`: Native Rust DLL for code-based mods.
- `mod.workshop_id`: Created by the Workshop uploader after the first normal mod upload.
- `database_pack.workshop_id`: Created by the Workshop uploader after the first database pack upload.

For native DLL mods, the DLL's registered mod id must match the folder name. This prevents accidentally loading a DLL from the wrong mod folder.

Workshop id files belong to your local working copy. Keep them if you want `TFM2ModUploader.exe` to update the same Steam Workshop item later. Do not include them in a public template that other creators are expected to copy.

## Upload Packaging

When uploading to Workshop, `TFM2ModUploader.exe` copies your package into a temporary upload folder.

For normal mods, it skips:

- `src/`
- `mod.workshop_id`
- `database_pack.workshop_id`

For database packs, it skips:

- `mod.workshop_id`
- `database_pack.workshop_id`

It includes compiled DLLs, JSON files, images, Aseprite files, UI layouts, thumbnails, preview images, and database pack files that remain in the selected folder.

For native Rust mods, install the matching Mod SDK next to the uploader if you want the tool to build the DLL before upload. Without the SDK, the uploader can still upload an already-built DLL.

## Naming Tips

- Use lowercase ASCII folder names: `my_mod`, `new_champions`, `balance_pack`, `league_database_pack`.
- Keep content ids stable after release. Saves and patches may refer to those ids.
- Avoid base-game ids and names that are likely to collide with other mods.
- Prefix new ids with your mod id when it helps readability, such as `my_mod_fire_mage`.