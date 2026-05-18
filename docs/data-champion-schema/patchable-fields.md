# Patchable Fields

Data champion actions implement the game's `PatchableFields` interface. This means some values can be read and adjusted by patch/balance systems.

## Action-Level Fields

Every `DataActionDef` exposes:

```text
cooltime
range
duration
start_timing
```

## Effect-Level Fields

The action also exposes some fields from its effect.

| Effect | Patchable fields |
| --- | --- |
| `Attack` | `damage`, `attack_ratio`, `hp_ratio`, `target_hp_ratio` |
| `ApAttack` | `damage`, `attack_ratio`, `hp_ratio` |
| `FixedAttack` | `damage`, `attack_ratio`, `hp_ratio`, `target_hp_ratio` |
| `Heal` | `amount`, `attack_ratio`, `ap_ratio` |
| `Shield` | `amount`, `attack_ratio`, `ap_ratio`, `tick` |
| `Stun` | `duration` |
| `Airborne` | `duration` |
| `Knockback` | `speed`, `tick` |
| `Fear` | `tick` |
| `Charm` | `tick` |
| `Bind` | `duration` |
| `Taunt` | `duration` |
| `Rush` | `speed`, `range` |
| `LinearProjectile` | `speed`, `range` |
| `Combine` | fields from the first child effect only |

Effects not listed here can still work in gameplay, but they do not currently expose patchable numeric fields through this interface.

## Patch Type Name

The patch type name for a data action is its `action_name`. Prefer stable names:

```text
attack
skill
skill2
ult
```

Changing `action_name` after release can affect patch/balance references and animation lookup, so treat it like a stable id.