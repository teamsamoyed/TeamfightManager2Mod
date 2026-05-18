# Actions

`attack`, `skill`, `skill2`, and `ult` use `DataActionDef`.

## Shape

```json
{
  "action_name": "skill",
  "description": "#asset/my_mod/text/champion?description.my_mod_fire_mage.skill",
  "duration": 20,
  "cooltime": 240,
  "start_timing": 10,
  "cancelable": true,
  "range": 65000,
  "growth_range": 0,
  "casting_type": "Direction",
  "casting_target": "Enemy",
  "attack_type": "Skill",
  "can_use_with_move": false,
  "effect": {
    "type": "LinearProjectile",
    "name": "fire_skill",
    "speed": 4200,
    "range": 65000,
    "applied_target": "Enemy",
    "applied_effects": []
  }
}
```

## Fields

| Field | Type | Default | Notes |
| --- | --- | --- | --- |
| `action_name` | string | `""` | Animation tag and patch type name. Prefer `attack`, `skill`, `skill2`, `ult`. |
| `duration` | integer | `0` | Total action duration in simulation ticks. |
| `cooltime` | integer | `0` | Cooldown in ticks. |
| `start_timing` | integer | `0` | Tick inside the action when the effect fires. |
| `cancelable` | boolean | `false` | Whether the action can be interrupted. |
| `range` | integer | `0` | Base cast/effect range. |
| `growth_range` | integer | `0` | Additional range per level. |
| `casting_type` | `CastingType` | `Targeting` | How player/AI input selects a target. |
| `casting_target` | `CastingTarget` | `Ally` | Which entity types can be selected. Usually set this explicitly. |
| `attack_type` | `AttackType` | `BaseAttack` | Damage classification used by combat systems. |
| `can_use_with_move` | boolean | `false` | Whether the action can be used while moving. |
| `description` | string | `""` | Plain text or i18n key. i18n keys should include the full `#asset/...?...` form. |
| `effect` | `DataEffectDef` or null | null | What happens at `start_timing`. |

## Slots

Use these public slot names consistently:

- `attack`: basic attack.
- `skill`: first skill.
- `skill2`: second skill.
- `ult`: ultimate.

`action_name` may be any animation tag you provide, but using the same names keeps data, animation, and docs predictable.

## Targeting Tips

- `casting_type: "Targeting"` expects a target entity.
- `casting_type: "Position"` targets a location.
- `casting_type: "Direction"` targets a direction from the caster.
- `casting_type: "None"` needs no target input and often pairs with `AllyOnlySelf`, `None`, or area effects around the caster.

Set both the action's `casting_target` and any projectile/effect `applied_target` or `target`. The Rust default for target enums is `Ally`, which is rarely correct for damage projectiles.