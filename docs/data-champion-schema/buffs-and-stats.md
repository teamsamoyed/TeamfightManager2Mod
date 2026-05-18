# Buffs and Stats

## `EntityStat`

Used by top-level `stat` and `growth`.

```json
{
  "attack": 40,
  "magic_power": 65,
  "hp": 620,
  "defence": 20,
  "magic_resistance": 30,
  "move_speed": 1050,
  "hp_regen": 2,
  "stack": 0,
  "crit_chance": 0
}
```

| Field | Type | Notes |
| --- | --- | --- |
| `attack` | integer | Attack stat. |
| `magic_power` | integer | Magic power stat. |
| `hp` | integer | Health. |
| `defence` | integer | Physical defense. |
| `magic_resistance` | integer | Magic resistance. |
| `move_speed` | integer | Movement speed. |
| `hp_regen` | integer | Health regeneration. |
| `stack` | integer | Generic stack stat used by some champion logic. |
| `crit_chance` | integer | Critical chance percent, 0-100. |

Rust defaults are nonzero for many fields, but mods should set every field explicitly so the champion is readable and stable.

## `DataBuffStateDef`

Used by `AddBuff` and `AddCasterBuff`.

```json
{
  "name": "fire_focus",
  "duration": { "Time": { "tick": 180 } },
  "attack": 0,
  "attack_mult": 0,
  "magic_power": 20,
  "magic_power_mult": 0,
  "defence": 0,
  "defence_mult": 0,
  "hp": 0,
  "hp_mult": 0,
  "hp_regen": 0,
  "magic_resistance": 0,
  "magic_resistance_mult": 0,
  "move_speed_mult": 0,
  "attack_speed_mult": 0,
  "skill_cooldown_mult": 15,
  "ult_cooldown_mult": 0,
  "damaged_amplify": 0,
  "damaged_reduce": 0,
  "dot_amplify": 0,
  "base_attack_enemy_max_hp_damage": 0,
  "skill_enemy_max_hp_damage": 0,
  "self_max_hp_damage": 0,
  "base_attack_damaged_reduce": 0,
  "skill_damaged_reduce": 0,
  "defence_penetration": 0,
  "magic_resistance_penetration": 0,
  "range": 0,
  "heal_reduce": 0,
  "toughness": 0,
  "crit_chance": 0,
  "radius_mult": 0,
  "vamp": 0,
  "damage_reflect": 0,
  "cc_immune": false,
  "undying": false,
  "ignore_wall": false
}
```

Only `name` is required by the Rust struct. Every other field has a default.

| Field | Type | Default | Notes |
| --- | --- | --- | --- |
| `name` | string | required | Buff id. Keep stable if visuals or logic refer to it. |
| `duration` | `BuffType` | `Permanent` | `Permanent`, `Time`, or `WithShield`. |
| `attack` | integer | `0` | Flat attack. |
| `attack_mult` | integer | `0` | Attack percent modifier. |
| `magic_power` | integer | `0` | Flat magic power. |
| `magic_power_mult` | integer | `0` | Magic power percent modifier. |
| `defence` | integer | `0` | Flat defense. |
| `defence_mult` | integer | `0` | Defense percent modifier. |
| `hp` | integer | `0` | Flat HP. |
| `hp_mult` | integer | `0` | HP percent modifier. |
| `hp_regen` | integer | `0` | Flat HP regen. |
| `magic_resistance` | integer | `0` | Flat magic resistance. |
| `magic_resistance_mult` | integer | `0` | Magic resistance percent modifier. |
| `move_speed_mult` | integer | `0` | Move speed percent modifier. |
| `attack_speed_mult` | integer | `0` | Attack speed percent modifier. |
| `skill_cooldown_mult` | integer | `0` | Skill cooldown modifier. |
| `ult_cooldown_mult` | integer | `0` | Ultimate cooldown modifier. |
| `damaged_amplify` | integer | `0` | Incoming damage amplification. |
| `damaged_reduce` | integer | `0` | Incoming damage reduction. |
| `dot_amplify` | integer | `0` | Dot damage amplification. |
| `base_attack_enemy_max_hp_damage` | integer | `0` | Enemy max-HP damage on base attacks. |
| `skill_enemy_max_hp_damage` | integer | `0` | Enemy max-HP damage on skills. |
| `self_max_hp_damage` | integer | `0` | Self max-HP damage modifier. |
| `base_attack_damaged_reduce` | integer | `0` | Damage reduction for base attacks. |
| `skill_damaged_reduce` | integer | `0` | Damage reduction for skills. |
| `defence_penetration` | integer | `0` | Physical defense penetration. |
| `magic_resistance_penetration` | integer | `0` | Magic resistance penetration. |
| `range` | integer | `0` | Range modifier. |
| `heal_reduce` | integer | `0` | Healing reduction. |
| `toughness` | integer | `0` | Crowd-control resistance related stat. |
| `crit_chance` | integer | `0` | Critical chance modifier. |
| `radius_mult` | integer | `0` | Radius modifier. |
| `vamp` | integer | `0` | Vamp/lifesteal style modifier. |
| `damage_reflect` | integer | `0` | Damage reflection. |
| `cc_immune` | boolean | `false` | Crowd-control immunity. |
| `undying` | boolean | `false` | Prevents death while active. |
| `ignore_wall` | boolean | `false` | Allows wall ignoring while active. |

## `BuffType`

```json
"Permanent"
```

```json
{ "Time": { "tick": 180 } }
```

```json
"WithShield"
```

Use `Time` for normal temporary buffs. `WithShield` ends when the related shield is gone.