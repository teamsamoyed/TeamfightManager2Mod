# Effects

`effect` fields use `DataEffectDef`. Every effect object has a `type` string. This page documents the effect types currently accepted by `.data_champion` JSON.

The game engine contains more native `EffectType` implementations than this page exposes. Data-only champions can use only the `DataEffectDef` variants listed here unless the game code adds another JSON mapping. See [Engine Effects Not Exposed To Data](#engine-effects-not-exposed-to-data) near the end for the remaining internal-only categories.

## Common Rules

Time values such as `tick`, `duration`, `delay`, `apply`, `period`, and `travel_time` are simulation ticks.

Most projectile and movement effects can contain nested effects. `applied_effects` entries are objects with an `effect` and an optional `casting_type`:

```json
{
  "effect": { "type": "Attack", "damage": 50 },
  "casting_type": "Targeting"
}
```

`casting_type` defaults to `Targeting`.

`end_effects` is usually a list of raw effect objects. `RangePeriodProjectile.end_effects` is the exception: it uses the same `{ "effect", "casting_type" }` entry format as `applied_effects` because the periodic area can apply end effects to targets inside the area.

Defaults for target selectors are often `Ally` because they come from Rust enum defaults. For damage, offensive projectiles, and enemy area effects, set `casting_target`, `applied_target`, or `target` explicitly.

## Supported Type Summary

| Category | Types |
| --- | --- |
| Damage and sustain | `Attack`, `ApAttack`, `FixedAttack`, `Heal`, `Shield` |
| Crowd control and disables | `Stun`, `Airborne`, `Knockback`, `Grab`, `Pull`, `Fear`, `Charm`, `Bind`, `Taunt`, `BlockAttack`, `BlockSkill`, `BlockMoveSkill`, `Invisible`, `Banish` |
| Movement | `Rush`, `RushTime`, `Teleport`, `DirTeleport`, `MoveBack`, `MoveTo`, `MoveToTarget`, `RushMoveToBack` |
| Projectiles and delayed areas | `LinearProjectile`, `BackToCasterLinearProjectile`, `TargetProjectile`, `TargetProjectileFromProjectile`, `TargetSplashProjectile`, `AutoTargetProjectile`, `RangeProjectile`, `LineRangeProjectile`, `RangePeriodProjectile`, `ApplyInProjectile`, `ParabolicProjectile` |
| Area and barriers | `RangeEffect`, `ShrinkingBarrier` |
| Buffs and over-time effects | `AddBuff`, `AddCasterBuff`, `RemoveCasterBuff`, `AddCasted` |
| Composition and branching | `Combine`, `Delayed`, `WithSelf`, `SwitchByBuff`, `SwitchByLevel3`, `RandomTarget` |
| Visual and audio triggers | `ViewEffect`, `CasterViewEffect`, `CasterAnimation`, `RemoveCasterAnimation`, `Sfx`, `TargetSfx` |

## Damage And Sustain

### `Attack`

Deals physical attack damage. Use this for AD-style skill damage or basic-attack-like damage.

```json
{
  "type": "Attack",
  "damage": 50,
  "attack_ratio": 100,
  "hp_ratio": 0,
  "target_hp_ratio": 0,
  "attack_effect_type": "Target"
}
```

- `damage`: flat damage.
- `attack_ratio`: percent of the caster's attack added to the damage.
- `hp_ratio`: percent of the caster's max HP added to the damage.
- `target_hp_ratio`: percent of the target's max HP added to the damage.
- `attack_effect_type`: target routing. See [Enums](enums.md#dataattackeffecttype).

Defaults: `damage = 0`, `attack_ratio = 100`, `hp_ratio = 0`, `target_hp_ratio = 0`, `attack_effect_type = "Target"`.

### `ApAttack`

Deals magic-power-based damage. Use this for AP-style skill damage.

```json
{
  "type": "ApAttack",
  "damage": 80,
  "attack_ratio": 100,
  "hp_ratio": 0,
  "attack_effect_type": "Target"
}
```

- `damage`: flat magic damage.
- `attack_ratio`: percent of the caster's magic power added to the damage.
- `hp_ratio`: percent of the caster's max HP added to the damage.
- `attack_effect_type`: target routing.

Defaults: `damage = 0`, `attack_ratio = 100`, `hp_ratio = 0`, `attack_effect_type = "Target"`.

### `FixedAttack`

Deals fixed damage through the fixed-damage path. Use it when the effect should not behave like normal AD or AP damage.

```json
{
  "type": "FixedAttack",
  "damage": 120,
  "attack_ratio": 0,
  "hp_ratio": 0,
  "target_hp_ratio": 0,
  "attack_effect_type": "Target"
}
```

Fields and defaults match `Attack`, but the damage resolution is fixed-damage resolution.

### `Heal`

Restores HP. Use `heal_type` to decide whether the effect heals the caster, the selected target, an ally only, or nearby allies.

```json
{
  "type": "Heal",
  "amount": 80,
  "attack_ratio": 0,
  "ap_ratio": 40,
  "heal_type": "Ally"
}
```

- `amount`: flat heal.
- `attack_ratio`: percent of caster attack added to the heal.
- `ap_ratio`: percent of caster magic power added to the heal.
- `heal_type`: heal routing. See [Enums](enums.md#datahealtype).

Defaults: `amount = 0`, `attack_ratio = 0`, `ap_ratio = 0`, `heal_type = "Any"`.

### `Shield`

Adds a temporary shield to the target.

```json
{
  "type": "Shield",
  "amount": 120,
  "attack_ratio": 0,
  "ap_ratio": 50,
  "tick": 300
}
```

- `amount`: flat shield value.
- `attack_ratio`: percent of caster attack added to the shield.
- `ap_ratio`: percent of caster magic power added to the shield.
- `tick`: shield lifetime.

Defaults: `amount = 0`, `attack_ratio = 0`, `ap_ratio = 0`, `tick = 300`.

## Crowd Control And Disables

### `Stun`

Prevents the target from acting for `duration` ticks.

```json
{ "type": "Stun", "duration": 60 }
```

### `Airborne`

Lifts/disables the target for `duration` ticks. Use it for knock-up style hard CC.

```json
{ "type": "Airborne", "duration": 45 }
```

### `Knockback`

Pushes the target away with the given `speed` for `tick` ticks.

```json
{ "type": "Knockback", "speed": 2000, "tick": 10 }
```

### `Grab`

Pulls the target toward the caster. `tick` is optional; if omitted, the engine uses the grab effect's internal timing behavior.

```json
{ "type": "Grab", "speed": 3500, "tick": 12 }
```

### `Pull`

Pulls the target toward the effect point/caster for a fixed number of ticks.

```json
{ "type": "Pull", "speed": 2500, "tick": 15 }
```

### `Fear`

Forces the target into fear behavior for `tick` ticks.

```json
{ "type": "Fear", "tick": 90 }
```

### `Charm`

Forces the target into charm behavior for `tick` ticks.

```json
{ "type": "Charm", "tick": 90 }
```

### `Bind`

Roots or movement-locks the target for `duration` ticks.

```json
{ "type": "Bind", "duration": 90 }
```

### `Taunt`

Taunts the target for `duration` ticks.

```json
{ "type": "Taunt", "duration": 90 }
```

### `BlockAttack`

Prevents basic attacks for `tick` ticks.

```json
{ "type": "BlockAttack", "tick": 90 }
```

### `BlockSkill`

Prevents skill use for `tick` ticks.

```json
{ "type": "BlockSkill", "tick": 90 }
```

### `BlockMoveSkill`

Prevents movement skills for `tick` ticks.

```json
{ "type": "BlockMoveSkill", "tick": 90 }
```

### `Invisible`

Makes the target invisible for `tick` ticks.

```json
{ "type": "Invisible", "tick": 120 }
```

### `Banish`

Temporarily removes/locks the target with optional visual effect names for the lock and ending moments.

```json
{
  "type": "Banish",
  "duration": 120,
  "lock_effect_name": "banish_lock",
  "end_effect_name": "banish_end"
}
```

`lock_effect_name` and `end_effect_name` default to empty strings.

## Movement

### `Rush`

Moves the caster toward a position input. While rushing, the caster can apply nested effects to entities that match `casting_target`.

```json
{
  "type": "Rush",
  "speed": 3500,
  "range": 50000,
  "move_speed_ratio": 0,
  "casting_target": "Enemy",
  "penetrate": false,
  "applied_effects": []
}
```

- `speed`: base rush speed.
- `range`: hit/check range during the rush.
- `move_speed_ratio`: extra speed from caster move speed as a percent.
- `casting_target`: entities affected during the rush.
- `penetrate`: continue through targets instead of ending on first hit.
- `applied_effects`: effects applied to collided targets.

Defaults: `range = 0`, `move_speed_ratio = 0`, `casting_target = "Ally"`, `penetrate = false`, `applied_effects = []`.

### `RushTime`

Moves the caster in the input direction for a fixed number of ticks instead of moving to a clicked destination.

```json
{
  "type": "RushTime",
  "speed": 3500,
  "tick": 30,
  "range": 50000,
  "casting_target": "Enemy",
  "penetrate": false,
  "applied_effects": []
}
```

### `Teleport`

Instantly moves the caster to a position input or target position.

```json
{ "type": "Teleport" }
```

### `DirTeleport`

Instantly moves the caster `moved` distance along a direction input.

```json
{ "type": "DirTeleport", "moved": 32000 }
```

### `MoveBack`

Moves the caster away from the target's position for `tick` ticks at `speed`.

```json
{ "type": "MoveBack", "speed": 2500, "tick": 12 }
```

### `MoveTo`

Starts a movement state toward a target, position, or direction. When the movement ends, `end_effects` fire.

```json
{
  "type": "MoveTo",
  "speed": 3500,
  "range": 50000,
  "end_effects": [
    { "type": "RangeEffect", "target": "Enemy", "effects": [{ "type": "Attack", "damage": 40 }] }
  ]
}
```

`range` is used for direction input. Defaults: `range = 0`, `end_effects = []`.

### `MoveToTarget`

Starts a movement state that tracks a target entity. If the target is gone when the effect resolves, `end_effects` run immediately against the original input.

```json
{
  "type": "MoveToTarget",
  "speed": 3500,
  "range": 50000,
  "end_effects": []
}
```

### `RushMoveToBack`

Rushes behind the target and triggers `applied_effects` after the travel delay. Use this for dash-behind or backstab-style skills.

```json
{
  "type": "RushMoveToBack",
  "speed": 4500,
  "applied_effects": [
    { "type": "Attack", "damage": 80, "attack_ratio": 100 }
  ]
}
```

## Projectiles And Delayed Areas

### `LinearProjectile`

Spawns a projectile that travels in a straight line from the caster toward a target, position, or direction. It applies `applied_effects` on hit and runs `end_effects` when it ends.

```json
{
  "type": "LinearProjectile",
  "penetrate": true,
  "speed": 4200,
  "range": 65000,
  "name": "fire_skill",
  "shape": { "Circle": { "radius": 8000 } },
  "applied_target": "Enemy",
  "applied_effects": [
    { "effect": { "type": "ApAttack", "damage": 50, "attack_ratio": 80 } }
  ],
  "end_effects": []
}
```

Defaults: `penetrate = false`, `shape = { "Circle": { "radius": 10000 } }`, `applied_target = "Ally"`, `applied_effects = []`, `end_effects = []`.

### `BackToCasterLinearProjectile`

Spawns a straight projectile from a position back to the caster. This is mainly useful as an `end_effect` of another projectile because projectile end effects pass a position input.

```json
{
  "type": "BackToCasterLinearProjectile",
  "penetrate": true,
  "speed": 4200,
  "range": 65000,
  "name": "return_blade",
  "shape": { "Circle": { "radius": 8000 } },
  "applied_target": "Enemy",
  "applied_effects": [],
  "end_effects": []
}
```

### `TargetProjectile`

Spawns a projectile that follows the selected target and applies effects on hit.

```json
{
  "type": "TargetProjectile",
  "speed": 4500,
  "name": "arrow_hit",
  "y_offset": 0,
  "applied_target": "Enemy",
  "applied_effects": []
}
```

### `TargetProjectileFromProjectile`

Spawns a target-following projectile from the current projectile position. This only works inside an effect chain that has projectile optional info, such as another projectile's `applied_effects` or `end_effects`.

```json
{
  "type": "TargetProjectileFromProjectile",
  "speed": 4500,
  "name": "split_bolt",
  "y_offset": 0,
  "applied_target": "Enemy",
  "applied_effects": []
}
```

### `TargetSplashProjectile`

Spawns a projectile that follows a target, then can jump/splash to another target within `range`.

```json
{
  "type": "TargetSplashProjectile",
  "speed": 4500,
  "name": "splash_hit",
  "range": 22000,
  "y_offset": 0,
  "applied_target": "Enemy",
  "applied_effects": []
}
```

### `AutoTargetProjectile`

Automatically chooses an enemy target in `range`, preferring the caster's recently attacked target when valid, then fires a target-following projectile.

```json
{
  "type": "AutoTargetProjectile",
  "speed": 4500,
  "range": 60000,
  "name": "auto_bolt",
  "y_offset": 0,
  "applied_target": "Enemy",
  "applied_effects": []
}
```

### `RangeProjectile`

Creates a delayed area at a position input. After `delay` ticks, it applies effects for `apply` ticks to targets in `shape`.

```json
{
  "type": "RangeProjectile",
  "name": "delayed_zone",
  "delay": 30,
  "apply": 60,
  "shape": { "Circle": { "radius": 26000 } },
  "applied_target": "Enemy",
  "applied_effects": []
}
```

### `LineRangeProjectile`

Creates a delayed line-shaped area from the caster toward the input direction/target/position.

```json
{
  "type": "LineRangeProjectile",
  "width": 8000,
  "length": 70000,
  "delay": 20,
  "apply": 30,
  "name": "line_blast",
  "applied_target": "Enemy",
  "applied_effects": []
}
```

### `RangePeriodProjectile`

Creates a stationary periodic area at a position input. It applies `applied_effects` every `period` ticks until `tick` expires, then applies `end_effects` to entities still in the area.

```json
{
  "type": "RangePeriodProjectile",
  "name": "burning_ground",
  "tick": 180,
  "period": 30,
  "first_delay": 0,
  "shape": { "Circle": { "radius": 26000 } },
  "applied_target": "Enemy",
  "applied_effects": [
    { "effect": { "type": "ApAttack", "damage": 20 }, "casting_type": "Targeting" }
  ],
  "end_effects": []
}
```

`period` should be greater than `0`; the loader clamps it to at least `1` to avoid invalid data.

### `ApplyInProjectile`

Creates an invisible projectile/area that waits `tick` ticks and then applies its effects to targets in `shape`. If `follow_caster` is true, the area follows the caster until it applies.

```json
{
  "type": "ApplyInProjectile",
  "name": "delayed_self_aura",
  "follow_caster": true,
  "tick": 45,
  "shape": { "Circle": { "radius": 24000 } },
  "applied_target": "Enemy",
  "applied_effects": []
}
```

### `ParabolicProjectile`

Spawns a projectile that travels in a fixed arc to a target or position. On arrival it applies hit effects and then `end_effects`. `range_effect_name` can show a preview/impact range visual while it travels.

```json
{
  "type": "ParabolicProjectile",
  "name": "meteor",
  "travel_time": 45,
  "range": 70000,
  "range_effect_name": "meteor_impact",
  "shape": { "Circle": { "radius": 24000 } },
  "applied_target": "Enemy",
  "applied_effects": [],
  "end_effects": []
}
```

`range_effect_name` defaults to an empty string.

## Area And Barriers

### `RangeEffect`

Immediately applies nested effects to targets inside `shape`. The area can be centered around the caster or placed forward from the caster.

```json
{
  "type": "RangeEffect",
  "shape": { "Circle": { "radius": 42000 } },
  "target": "Enemy",
  "apply_type": "AroundCaster",
  "effects": [
    { "type": "ApAttack", "damage": 120, "attack_ratio": 100 },
    { "type": "Stun", "duration": 45 }
  ]
}
```

Defaults: `target = "Ally"`, `apply_type = "AroundCaster"`, `effects = []`.

### `ShrinkingBarrier`

Creates a circle around a target that follows that target and shrinks over time. Effects are applied on the barrier edge according to the engine barrier behavior.

```json
{
  "type": "ShrinkingBarrier",
  "name": "closing_ring",
  "start_radius": 70000,
  "end_radius": 16000,
  "shrink_per_tick": 800,
  "tick": 120,
  "edge_thickness": 6000,
  "applied_effects": [
    { "effect": { "type": "Bind", "duration": 20 } }
  ]
}
```

## Buffs And Over-Time Effects

### `AddBuff`

Adds a stat/status `buff_state` to the selected target.

```json
{
  "type": "AddBuff",
  "buff_state": {
    "name": "burning",
    "duration": { "Time": { "tick": 180 } },
    "damaged_amplify": 115
  }
}
```

See [Buffs and Stats](buffs-and-stats.md) for every `buff_state` field.

### `AddCasterBuff`

Adds a stat/status `buff_state` to the caster. If `only_to_enemy` is true, the buff is added only when the original target is an enemy.

```json
{
  "type": "AddCasterBuff",
  "only_to_enemy": false,
  "buff_state": {
    "name": "focus",
    "duration": { "Time": { "tick": 180 } },
    "magic_power": 20
  }
}
```

`only_to_enemy` defaults to `false`.

### `RemoveCasterBuff`

Removes a named normal buff from the caster. Use it for temporary self-state cleanup, such as ending a mode buff.

```json
{ "type": "RemoveCasterBuff", "name": "focus" }
```

### `AddCasted`

Adds a periodic over-time effect to the selected target. Every `period` ticks until `duration`, each nested effect is applied to that target.

```json
{
  "type": "AddCasted",
  "duration": 180,
  "period": 30,
  "casted_type": "Fire",
  "effects": [
    { "type": "ApAttack", "damage": 20, "attack_ratio": 20 }
  ]
}
```

`casted_type` controls the over-time category/icon and attack type. Values: `Bleed`, `Poison`, `Fire`, `Heal`. Default: `Fire`. `period` should be greater than `0`; the loader clamps it to at least `1`.

## Composition And Branching

### `Combine`

Runs multiple effects immediately with the same input.

```json
{
  "type": "Combine",
  "effects": [
    { "type": "Attack", "damage": 50 },
    { "type": "Heal", "amount": 30, "heal_type": "Caster" }
  ]
}
```

### `Delayed`

Queues nested effects to run after `tick` ticks.

```json
{
  "type": "Delayed",
  "tick": 30,
  "effects": [
    { "type": "Stun", "duration": 30 }
  ]
}
```

### `WithSelf`

Runs nested effects with the caster as the target/self context.

```json
{
  "type": "WithSelf",
  "effects": [
    { "type": "AddCasterBuff", "buff_state": { "name": "self_mark" } }
  ]
}
```

### `SwitchByBuff`

Chooses one of two effects depending on whether the caster currently has `buff_name`.

```json
{
  "type": "SwitchByBuff",
  "buff_name": "empowered",
  "effect_none": { "type": "Attack", "damage": 40 },
  "effect_buff": { "type": "Attack", "damage": 90 }
}
```

### `SwitchByLevel3`

Chooses one effect before level 3 and another at level 3 or higher.

```json
{
  "type": "SwitchByLevel3",
  "effect_start": { "type": "Shield", "amount": 80 },
  "effect_level3": { "type": "Shield", "amount": 160 }
}
```

### `RandomTarget`

Chooses a random target in `range` that matches `casting_target`, then applies nested effects to it.

```json
{
  "type": "RandomTarget",
  "range": 65000,
  "casting_target": "EnemyChampion",
  "from_projectile": false,
  "effects": [
    { "type": "ApAttack", "damage": 80 }
  ]
}
```

Defaults: `casting_target = "Ally"`, `from_projectile = false`.

## Visual And Audio Triggers

These trigger named visual/audio systems. Register custom systems through [Visual Bindings](visual-bindings.md).

### `ViewEffect`

Plays a named visual effect at the effect target/input position.

```json
{ "type": "ViewEffect", "name": "fire_burst" }
```

### `CasterViewEffect`

Plays a named visual effect at the caster position.

```json
{ "type": "CasterViewEffect", "name": "caster_flash" }
```

### `CasterAnimation`

Adds a named caster animation state for `tick` ticks.

```json
{ "type": "CasterAnimation", "name": "cast_pose", "tick": 30 }
```

### `RemoveCasterAnimation`

Removes a named caster animation state.

```json
{ "type": "RemoveCasterAnimation", "name": "cast_pose" }
```

### `Sfx`

Plays a named sound effect at the caster position.

```json
{ "type": "Sfx", "name": "fire_cast" }
```

### `TargetSfx`

Plays a named sound effect at the target, position input, or current projectile position.

```json
{ "type": "TargetSfx", "name": "fire_hit" }
```

## Engine Effects Not Exposed To Data

The engine has additional effect implementations that are not currently mapped to `.data_champion` JSON. They are intentionally not listed as supported `type` strings above.

| Internal category | Why it is not a normal data effect yet |
| --- | --- |
| `AddEffectBuff`, `AddCasterEffectBuff`, `DamageShare` | These use `EffectBuff`, a runtime buff behavior trait. Data JSON currently models normal stat/status buffs, not arbitrary effect-buff behavior objects. |
| Summons such as `SpawnBear`, `SpawnEagle`, `SpawnGhoul`, `SpawnIllusion`, `SpawnRevenant` | These depend on hard-coded summoned entity behavior and champion-specific assumptions. |
| `TowerAttack` | Tied to tower/minion combat rules, including epic timing behavior. Not a general champion skill effect. |
| Champion-specific effects such as Bard, Druid, Werewolf, Voodoo Shaman, Ice Mage, and similar ult/skill modules | These encode bespoke champion mechanics. They need separate data schema work before they can be safely exposed to mod authors. |
| Additional projectile movement internals such as bouncing target movement | Some are reachable only through hard-coded champion effects. They need explicit JSON fields and validation before public support. |

When adding another effect to data support, update `DataEffectDef` in `game-core/src/setting/champion/data_driven.rs` and document its JSON shape here.