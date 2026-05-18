# Recipes

These snippets are small patterns you can copy into an action `effect`.

## Basic Target Projectile Attack

```json
{
  "type": "TargetProjectile",
  "speed": 4500,
  "name": "fire_attack",
  "applied_target": "Enemy",
  "applied_effects": [
    {
      "effect": {
        "type": "Attack",
        "damage": 0,
        "attack_ratio": 100
      }
    }
  ]
}
```

## Directional Piercing Skill

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
    {
      "effect": {
        "type": "ApAttack",
        "damage": 50,
        "attack_ratio": 80
      }
    }
  ]
}
```

Pair with action fields:

```json
{
  "action_name": "skill",
  "casting_type": "Direction",
  "casting_target": "Enemy",
  "attack_type": "Skill"
}
```

## Self Buff

```json
{
  "type": "AddCasterBuff",
  "buff_state": {
    "name": "fire_focus",
    "duration": { "Time": { "tick": 180 } },
    "magic_power": 20,
    "skill_cooldown_mult": 15
  }
}
```

Pair with:

```json
{
  "casting_type": "None",
  "casting_target": "AllyOnlySelf",
  "attack_type": "Skill"
}
```

## Area Ultimate Around Caster

```json
{
  "type": "RangeEffect",
  "shape": { "Circle": { "radius": 42000 } },
  "target": "Enemy",
  "apply_type": "AroundCaster",
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
```

Pair with:

```json
{
  "action_name": "ult",
  "casting_type": "None",
  "casting_target": "Enemy",
  "attack_type": "Skill"
}
```

## Dash Then Delayed Hit

```json
{
  "type": "Combine",
  "effects": [
    {
      "type": "Rush",
      "speed": 3500,
      "range": 50000,
      "casting_target": "Enemy",
      "penetrate": false
    },
    {
      "type": "Delayed",
      "tick": 12,
      "effects": [
        {
          "type": "Attack",
          "damage": 70,
          "attack_ratio": 100
        }
      ]
    }
  ]
}
```

## Random Enemy Bolt

```json
{
  "type": "RandomTarget",
  "range": 65000,
  "casting_target": "EnemyChampion",
  "from_projectile": false,
  "effects": [
    {
      "type": "ApAttack",
      "damage": 80,
      "attack_ratio": 70
    }
  ]
}
```

## Trigger a Registered Visual

```json
{
  "type": "ViewEffect",
  "name": "fire_burst"
}
```

Register the visual in `view_effects`:

```json
{
  "view_effects": [
    {
      "type": "Animation",
      "name": "fire_burst",
      "anim": "asset/my_mod/effects/fire_burst",
      "tag": "burst"
    }
  ]
}
```