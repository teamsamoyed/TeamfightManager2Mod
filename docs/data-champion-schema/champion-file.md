# Champion File

A data champion is a JSON file with the `.data_champion` extension. The asset loader parses it into `DataChampionInfo`.

## Minimal Shape

```json
{
  "id": "my_mod_fire_mage",
  "category": "Magician",
  "tags": ["AP", "Range"],
  "stat": {
    "attack": 40,
    "magic_power": 65,
    "hp": 620,
    "defence": 20,
    "magic_resistance": 30,
    "move_speed": 1050,
    "hp_regen": 2,
    "stack": 0,
    "crit_chance": 0
  },
  "growth": {
    "attack": 3,
    "magic_power": 7,
    "hp": 75,
    "defence": 3,
    "magic_resistance": 3,
    "move_speed": 0,
    "hp_regen": 1,
    "stack": 0,
    "crit_chance": 0
  },
  "attack": { "action_name": "attack" },
  "skill": { "action_name": "skill" },
  "skill2": { "action_name": "skill2" },
  "ult": { "action_name": "ult" }
}
```

`attack`, `skill`, and `skill2` are required by the loader. `ult` is optional; if omitted, the champion has no ultimate action.

## Top-Level Fields

| Field | Type | Required | Default | Notes |
| --- | --- | --- | --- | --- |
| `id` | string | yes | none | Unique champion id. Keep stable after release. |
| `category` | `ChampionCategory` | no | `Melee` | See [Enums](enums.md). |
| `tags` | `ChampionTag[]` | no | `[]` | Used for UI/search/balance classification. |
| `stat` | `EntityStat` | no | nonzero default stat | Base level 1 stats. See [Buffs and Stats](buffs-and-stats.md). |
| `growth` | `EntityStat` | no | nonzero default stat | Per-level stat growth. Usually set every field explicitly. |
| `attack` | `DataActionDef` | yes | none | Basic attack action. |
| `skill` | `DataActionDef` | yes | none | First skill action. |
| `skill2` | `DataActionDef` | yes | none | Second skill action. |
| `ult` | `DataActionDef` | no | no ultimate | Ultimate action. |
| `sprite` | string | no | none | Asset path for the champion visual, without `#sheet` or `#anim`. |
| `anim_prefix` | string or null | no | null | Required when using an animated source that should be rebound. Use `""` to copy tags as-is. |
| `skill_icon` | `DataSkillIconDef` | no | none | Shared sprite-sheet icon source plus three tags. |
| `skill_icons` | string[] | no | none | Three direct icon asset paths for `skill`, `skill2`, `ult`. Takes priority over `skill_icon`. |
| `view_effects` | `DataViewEffectDef[]` | no | `[]` | Registers named effect visuals. |
| `view_projectiles` | `DataViewProjectileDef[]` | no | `[]` | Registers named projectile visuals. |
| `view_buffs` | `DataViewBuffDef[]` | no | `[]` | Registers named buff visuals. |

## Skill Icons

Use `skill_icons` when each icon is its own PNG:

```json
{
  "skill_icons": [
    "asset/my_mod/icons/fire_skill",
    "asset/my_mod/icons/fire_skill2",
    "asset/my_mod/icons/fire_ult"
  ]
}
```

Use `skill_icon` when the icons are packed in one sprite sheet:

```json
{
  "skill_icon": {
    "source": "asset/my_mod/icons/fire_skills",
    "tags": ["fire_skill", "fire_skill2", "fire_ult"]
  }
}
```

The UI uses icon index `0` for `skill`, `1` for `skill2`, and `2` for `ult`.

## Sprite Binding

`sprite` points at an asset base path:

```json
{
  "sprite": "asset/my_mod/champions/fire_mage",
  "anim_prefix": ""
}
```

For an animated Aseprite/manual animation source, set `anim_prefix`. The game creates or remaps:

```text
asset/base/aseprite_resources/champions/<id>#sheet
asset/base/aseprite_resources/champions/<id>#anim
```

That lets the normal champion renderer find your data champion by id.

For a static PNG, omit `anim_prefix`. The game creates one-frame fallback animations for common tags: `idle`, `attack`, `skill`, `skill2`, `ult`, `dead`, and `run`.