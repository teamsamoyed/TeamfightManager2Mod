# Visual Bindings

Data champion effects refer to visuals by `name`. `view_effects`, `view_projectiles`, and `view_buffs` register those names for the game view.

## Effect Visuals

Used by `ViewEffect` and `CasterViewEffect`.

### `Animation`

```json
{
  "type": "Animation",
  "name": "fire_burst",
  "anim": "asset/my_mod/effects/fire_burst",
  "tag": "burst",
  "z": 0,
  "is_follow": false
}
```

### `LoopAnimation`

```json
{
  "type": "LoopAnimation",
  "name": "fire_aura",
  "anim": "asset/my_mod/effects/fire_aura",
  "tag": "loop",
  "z": 0,
  "is_follow": true
}
```

Fields:

- `name`: effect name used by `ViewEffect` or `CasterViewEffect`.
- `anim`: animation asset base path.
- `tag`: animation tag inside `anim#anim`.
- `z`: render depth offset, default `0`.
- `is_follow`: whether the animation follows its target, default `false`. The current loop registration ignores this flag internally, but keep it explicit for readability.

## Projectile Visuals

Used by projectile effects whose `name` matches the binding.

### `Animated`

```json
{
  "type": "Animated",
  "name": "fire_skill",
  "anim": "asset/my_mod/projectiles/fire_bolt",
  "tag": "fly",
  "z": 0,
  "repeat": true
}
```

### `Sprite`

```json
{
  "type": "Sprite",
  "name": "fire_skill",
  "sprite": "asset/my_mod/projectiles/fire_bolt",
  "z": 0
}
```

### `ThreePhase`

```json
{
  "type": "ThreePhase",
  "name": "fire_orb",
  "anim": "asset/my_mod/projectiles/fire_orb",
  "pre_tag": "spawn",
  "loop_tag": "loop",
  "remove_tag": "remove"
}
```

## Buff Visuals

Used when a buff with the same `name` is active.

### `Animated`

```json
{
  "type": "Animated",
  "name": "fire_focus",
  "anim": "asset/my_mod/buffs/fire_focus",
  "tag": "loop",
  "z": 0
}
```

### `ThreePhase`

```json
{
  "type": "ThreePhase",
  "name": "fire_focus",
  "anim": "asset/my_mod/buffs/fire_focus",
  "pre_tag": "on",
  "loop_tag": "loop",
  "remove_tag": "off",
  "z": 0
}
```

## Name Matching

Names must match exactly:

```json
{
  "effect": { "type": "ViewEffect", "name": "fire_burst" },
  "view_effects": [
    { "type": "Animation", "name": "fire_burst", "anim": "asset/my_mod/effects/fire", "tag": "burst" }
  ]
}
```

Projectile names work the same way:

```json
{
  "effect": { "type": "LinearProjectile", "name": "fire_skill", "speed": 4200, "range": 65000 },
  "view_projectiles": [
    { "type": "Sprite", "name": "fire_skill", "sprite": "asset/my_mod/projectiles/fire_skill" }
  ]
}
```