# Asset Overrides and i18n

Mods can bring their own assets and can also change selected base-game assets.

Use this for things like:

- adding champion names and skill descriptions,
- adding UI text,
- replacing a sprite,
- merging a few JSON keys without replacing the whole file.

## Asset Paths

Every file under your mod folder gets an asset path without its file extension:

```text
mods/example/text/ui.i18n -> asset/example/text/ui
mods/example/icons/q.png  -> asset/example/icons/q
```

Use these paths in data files, native code, and override rules.

## `mod.override_info`

Create `mod.override_info` in the mod root when you want to merge or replace existing assets:

```json
{
  "asset/base/text/ui": {
    "remapping": "asset/example/text/ui",
    "type": "merge"
  },
  "asset/base/aseprite_resources/champions/example_ghoul": {
    "remapping": "asset/example/aseprite_resources/champions/ghoul_king",
    "type": "override"
  }
}
```

The object key is the base asset you want to change. `remapping` points to your replacement or patch asset.

Types:

- `merge`: Merge JSON objects. This is best for i18n and small JSON additions.
- `override`: Replace the whole target asset path with your asset.

If this file contains invalid JSON, the game disables the mod and shows the error in the diagnostics popup.

## i18n Files

i18n files are JSON grouped by language:

```json
{
  "en": {
    "description": {
      "my_mod_fire_mage": {
        "name": "Fire Mage",
        "skill": "Launches a fire bolt.",
        "skill2": "Focuses flame energy.",
        "ult": "Burns all nearby enemies."
      }
    }
  },
  "ko": {
    "description": {
      "my_mod_fire_mage": {
        "name": "Fire Mage",
        "skill": "Launches a fire bolt.",
        "skill2": "Focuses flame energy.",
        "ult": "Burns all nearby enemies."
      }
    }
  }
}
```

To add this to the base champion text table, merge it into `asset/base/text/champion`:

```json
{
  "asset/base/text/champion": {
    "remapping": "asset/my_mod/text/champion",
    "type": "merge"
  }
}
```

Then your champion data can reference:

```text
#asset/base/text/champion?description.my_mod_fire_mage.skill
```

## Text Markup

Descriptions can use the same inline markup as the base game:

```text
Deals <#ff9028ff>60<> + <i#asset/base/ui/banpick/champion_stat_icon:ad_0><#ff9028ff>80% AD<> physical damage.
```

Common patterns:

- `<#rrggbbaa>text<>` colors text.
- `<iasset_path:tag>` inserts an icon from a tagged sprite sheet.

For inline icons, `asset_path` is the shared sheet path, not the `#sheet` path. The game reads rectangles from `asset_path#data` and draws from `asset_path#sheet`. See [Assets and Sprite Sheets](assets-and-sprite-sheets.md) for the sheet format.

When in doubt, look at the base text files and copy the style of an existing champion or item description.

## Compatibility Tips

- Prefer `merge` for text and JSON files.
- Use `override` only when you really want to replace the entire asset.
- Avoid replacing full-screen UI layouts for small changes. A native extension that adds one button or popup is usually easier to keep compatible.
- If multiple mods edit the same key, the enabled mod order can affect the final result.
