# Data-Only Champions

Data-only champions are the best place to start. They are JSON files with the `.data_champion` extension, and they let you add a playable champion or rework an existing champion without writing Rust code.

Place them anywhere under the mod folder, commonly:

```text
mods/my_mod/champion/fire_mage.data_champion
```

The game sees that file as this asset path:

```text
asset/my_mod/champion/fire_mage
```

## Full Schema Reference

This page is a tutorial-style example. For every field, enum value, effect type, visual binding, and patchable field, see [Data Champion Schema](data-champion-schema/index.md).

## Example Champion

The example below defines a ranged magic champion with a basic attack, two skills, an ultimate, skill icons, and projectile visuals.

```json
{
  "id": "my_mod_fire_mage",
  "category": "Magician",
  "tags": ["AP", "Range"],
  "sprite": "asset/base/aseprite_resources/champions/pyromancer",
  "anim_prefix": "",
  "skill_icons": [
    "asset/my_mod/icons/fire_mage_skill",
    "asset/my_mod/icons/fire_mage_skill2",
    "asset/my_mod/icons/fire_mage_ult"
  ],
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
  "attack": {
    "action_name": "attack",
    "duration": 18,
    "cooltime": 60,
    "start_timing": 10,
    "cancelable": true,
    "range": 52000,
    "casting_type": "Targeting",
    "casting_target": "Enemy",
    "attack_type": "BaseAttack",
    "effect": {
      "type": "TargetProjectile",
      "speed": 4500,
      "name": "fire_mage_attack",
      "applied_target": "Enemy",
      "applied_effects": [
        {
          "effect": {
            "type": "Attack",
            "damage": 0,
            "attack_ratio": 100
          },
          "casting_type": "Targeting"
        }
      ]
    }
  },
  "skill": {
    "action_name": "skill",
    "description": "#asset/my_mod/text/champion?description.my_mod_fire_mage.skill",
    "duration": 20,
    "cooltime": 240,
    "start_timing": 10,
    "range": 65000,
    "casting_type": "Direction",
    "casting_target": "Enemy",
    "attack_type": "Skill",
    "effect": {
      "type": "LinearProjectile",
      "penetrate": true,
      "speed": 4200,
      "range": 65000,
      "name": "fire_mage_skill",
      "shape": { "Circle": { "radius": 8000 } },
      "applied_target": "Enemy",
      "applied_effects": [
        {
          "effect": {
            "type": "ApAttack",
            "damage": 50,
            "attack_ratio": 80
          },
          "casting_type": "Targeting"
        }
      ]
    }
  },
  "skill2": {
    "action_name": "skill2",
    "description": "#asset/my_mod/text/champion?description.my_mod_fire_mage.skill2",
    "duration": 16,
    "cooltime": 360,
    "start_timing": 8,
    "range": 0,
    "casting_type": "None",
    "casting_target": "AllyOnlySelf",
    "attack_type": "Skill",
    "effect": {
      "type": "AddCasterBuff",
      "buff_state": {
        "name": "fire_focus",
        "duration": { "Time": { "tick": 180 } },
        "magic_power": 20,
        "skill_cooldown_mult": 15
      }
    }
  },
  "ult": {
    "action_name": "ult",
    "description": "#asset/my_mod/text/champion?description.my_mod_fire_mage.ult",
    "duration": 25,
    "cooltime": 900,
    "start_timing": 12,
    "range": 42000,
    "casting_type": "None",
    "casting_target": "Enemy",
    "attack_type": "Skill",
    "effect": {
      "type": "RangeEffect",
      "shape": { "Circle": { "radius": 42000 } },
      "target": "Enemy",
      "apply_type": "AroundCaster",
      "effects": [
        {
          "type": "Combine",
          "effects": [
            {
              "type": "ApAttack",
              "damage": 120,
              "attack_ratio": 100
            },
            {
              "type": "Stun",
              "duration": 45
            }
          ]
        }
      ]
    }
  },
  "view_projectiles": [
    {
      "type": "Sprite",
      "name": "fire_mage_attack",
      "sprite": "asset/base/sprite/arrow"
    },
    {
      "type": "Sprite",
      "name": "fire_mage_skill",
      "sprite": "asset/base/sprite/arrow"
    }
  ]
}
```

## Top-Level Fields

- `id`: Champion id. Use a new unique id to add a champion, or use an existing base-game champion id to rework that champion. Keep it stable after release because saves, ban/pick data, and patches may refer to it.
- `category`: `Melee`, `Range`, `Magician`, `Util`, or `Assassin`.
- `tags`: Any of `AD`, `AP`, `Heal`, `Shield`, `Dot`, `CC`, `Range`, `Melee`, `Tank`, `Magic`.
- `stat`: Base level 1 stats.
- `growth`: Per-level stat growth.
- `attack`, `skill`, `skill2`, `ult`: Required actions.
- `sprite`: Optional asset path for the champion visual. This can be a PNG, an Aseprite asset, or a manual `#sheet` plus `#anim` animation pair.
- `anim_prefix`: Used with animated sprite sources. Set it to `""` to keep animation tags as-is, or to a prefix such as `"eagle_"` to strip that prefix from tags.
- `skill_icon`: Optional shared icon sheet definition. Use this when skill, skill2, and ult icons are packed into one sheet with tags.
- `skill_icons`: Optional list of three direct icon asset paths for skill, skill2, and ult. Use this when each icon is its own PNG.
- `view_effects`, `view_projectiles`, `view_buffs`: Optional view bindings.

## Action Fields

- `action_name`: Animation tag to play, such as `attack`, `skill`, `skill2`, or `ult`.
- `duration`: Total action duration in simulation ticks.
- `cooltime`: Cooldown in ticks.
- `start_timing`: Tick inside the action when the effect fires.
- `cancelable`: Whether the action can be interrupted.
- `range`: Base cast range.
- `growth_range`: Additional range per level.
- `casting_type`: `Targeting`, `Position`, `Direction`, or `None`.
- `casting_target`: See target list below.
- `attack_type`: `BaseAttack`, `Skill`, `Dot`, `DotIgnoreShield`, `Item`, or `Well`.
- `can_use_with_move`: Whether the action can be used while moving.
- `description`: i18n key or plain string.
- `effect`: Effect definition.

## Animation Tags

`action_name` is both a gameplay action name and a visual lookup key. When an action starts, the champion renderer tries to play an animation tag with the same name.

Common champion tags are:

```text
idle
run
attack
skill
skill2
ult
dead
```

If your `skill` action uses:

```json
{
  "action_name": "fire_cast"
}
```

then the sprite's animation data should also have a `fire_cast` tag. In Aseprite, that means a timeline tag named `fire_cast`. In a manual `.fanim` file, that means an entry under `anims.fire_cast`.

Use the familiar names first unless you need a special animation. A missing tag usually means the champion loads but does not show the expected animation during that action.

## Common Targets

Useful `casting_target` values:

- `Enemy`
- `EnemyWithoutTower`
- `EnemyChampion`
- `EnemyChampionInCC`
- `Ally`
- `AllyChampion`
- `AllyNotSelf`
- `AllyOnlySelf`
- `Both`
- `BothChampion`
- `None`

## Effect Types

Data champions can use these effect types:

- Damage and healing: `Attack`, `ApAttack`, `FixedAttack`, `Heal`, `Shield`.
- Crowd control: `Stun`, `Airborne`, `Knockback`, `Grab`, `Pull`, `Fear`, `Charm`, `Bind`, `Taunt`, `BlockAttack`, `BlockSkill`, `BlockMoveSkill`, `Invisible`, `Banish`.
- Movement: `Rush`, `RushTime`, `Teleport`, `DirTeleport`, `MoveBack`, `MoveTo`, `MoveToTarget`, `RushMoveToBack`.
- Projectiles: `LinearProjectile`, `TargetProjectile`, `TargetSplashProjectile`, `AutoTargetProjectile`, `RangeProjectile`, `ParabolicProjectile`, `BackToCasterLinearProjectile`, `TargetProjectileFromProjectile`, `LineRangeProjectile`, `RangePeriodProjectile`, `ApplyInProjectile`.
- Area and barriers: `RangeEffect`, `ShrinkingBarrier`.
- Buffs and casted effects: `AddBuff`, `AddCasterBuff`, `RemoveCasterBuff`, `AddCasted`.
- Composition and branching: `Combine`, `Delayed`, `WithSelf`, `RandomTarget`, `SwitchByBuff`, `SwitchByLevel3`.
- Visual/audio triggers: `ViewEffect`, `CasterViewEffect`, `CasterAnimation`, `RemoveCasterAnimation`, `Sfx`, `TargetSfx`.

This page only summarizes the common groups. For exact fields and nested examples for every effect type, use [Effect Schema](data-champion-schema/effects.md).

## Shapes

Projectile and range effects use `ProjectileShape`:

```json
{ "Circle": { "radius": 10000 } }
```

```json
{ "Rect": { "width": 12000, "height": 8000 } }
```

```json
{ "DirDot": { "radius": 8000, "range": 700 } }
```

`Line` is also supported, but it requires explicit endpoints and is usually less convenient for authored champion data.

## Text

Descriptions usually point to i18n keys:

```json
"#asset/base/text/champion?description.my_mod_fire_mage.skill"
```

Add those keys through an i18n merge. See [Asset Overrides and i18n](asset-overrides-and-i18n.md).

## Reworking an Existing Champion

To rework a base-game champion with JSON, set the data champion `id` to the exact existing champion id instead of making a new mod-prefixed id:

```json
{
  "id": "fighter",
  "category": "Melee",
  "tags": ["AD", "Melee"],
  "stat": {
    "attack": 55,
    "magic_power": 0,
    "hp": 700,
    "defence": 35,
    "magic_resistance": 25,
    "move_speed": 1100,
    "hp_regen": 3,
    "stack": 0,
    "crit_chance": 0
  },
  "growth": {
    "attack": 5,
    "magic_power": 0,
    "hp": 90,
    "defence": 4,
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

The game treats this as a replacement for lookup and gameplay while keeping the champion's public id and internal ordering stable. Existing saves, ban/pick references, and balance patch references that point to that id continue to point to the same champion.

If you also need to change the champion name or skill descriptions, merge the matching text keys into `asset/base/text/champion` through `mod.override_info`. Do not put player-facing text directly into the JSON when it should be localized.

If a data champion and a native Rust champion both use the same existing id, the native Rust runtime takes priority for custom logic. Use one approach per champion unless you intentionally need the Rust side to own the runtime behavior.

## Sprite Binding

The `sprite` field is written without `#sheet` or `#anim`:

```json
{
  "sprite": "asset/example/champions/fire_mage"
}
```

The game decides how to bind it based on what exists at that path.

### Static PNG

If `sprite` points to a PNG, do not set `anim_prefix`:

```json
{
  "sprite": "asset/example/champions/fire_mage_idle"
}
```

The whole image is used as a one-frame sprite. The game creates simple fallback animations for `idle`, `attack`, `skill`, `skill2`, `ult`, `dead`, and `run`.

This is useful while testing a champion. It is not ideal for a finished animated champion.

### Animated Aseprite or Manual Animation Sheet

If `sprite` points to an Aseprite animation or to a manual `#sheet` plus `#anim` pair, set `anim_prefix`.

Use an empty string when the tags already match the standard champion tags:

```json
{
  "sprite": "asset/example/champions/fire_mage",
  "anim_prefix": ""
}
```

This keeps tags such as `idle`, `attack`, `skill`, and `ult` unchanged.

Use a real prefix when one source file contains several variants. For example:

```json
{
  "sprite": "asset/base/aseprite_resources/champions/druid",
  "anim_prefix": "eagle_"
}
```

The source tags `eagle_idle`, `eagle_attack`, and `eagle_ult` become `idle`, `attack`, and `ult` for the new champion.

Tags without the prefix are also kept. That lets a shared source file contain common tags that every variant can use.

For a full explanation of `#sheet`, `#anim`, Aseprite metadata, and manual PNG/JSON sprite sheets, see [Assets and Sprite Sheets](assets-and-sprite-sheets.md).

## Skill Icons

There are two ways to define skill, skill2, and ult icons.

Use `skill_icons` when each icon is its own PNG:

```json
{
  "skill_icons": [
    "asset/example/icons/fire_skill",
    "asset/example/icons/fire_skill2",
    "asset/example/icons/fire_ult"
  ]
}
```

Use `skill_icon` when the icons are packed into one sprite sheet:

```json
{
  "skill_icon": {
    "source": "asset/example/icons/fire_skills",
    "tags": ["fire_skill", "fire_skill2", "fire_ult"]
  }
}
```

In the second form, the game looks for:

```text
asset/example/icons/fire_skills#sheet
asset/example/icons/fire_skills#data
```

and then draws the tagged rectangles from that sheet.
