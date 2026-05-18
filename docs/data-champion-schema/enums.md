# Enums

## `ChampionCategory`

```text
Melee
Range
Magician
Util
Assassin
```

Default: `Melee`.

## `ChampionTag`

```text
AD
AP
Heal
Shield
Dot
CC
Range
Melee
Tank
Magic
```

## `CastingType`

```text
Targeting
Position
Direction
None
```

Default: `Targeting`.

- `Targeting`: target entity input.
- `Position`: position input.
- `Direction`: direction input from caster.
- `None`: no target input.

## `CastingTarget`

```text
Ally
AllyChampion
AllyChampionInCC
AllyNotSelf
AllyOnlySelf
Enemy
EnemyWithoutTower
EnemyChampion
EnemyChampionInCC
EnemyChampionRecentlyAttacked
Both
BothWithoutTower
BothChampion
None
```

Default: `Ally`.

Because the default is `Ally`, damage and projectile effects should usually set `casting_target`, `applied_target`, or `target` explicitly.

## `AttackType`

```text
BaseAttack
Skill
Dot
DotIgnoreShield
Item
Well
```

Default: `BaseAttack`.

Use `BaseAttack` for the `attack` slot and `Skill` for `skill`, `skill2`, and `ult` unless you intentionally need another classification.

## `ProjectileShape`

```json
{ "Circle": { "radius": 10000 } }
```

```json
{ "Line": { "width": 8000, "from_x": 0, "from_y": 0, "to_x": 50000, "to_y": 0 } }
```

```json
{ "Rect": { "width": 30000, "height": 16000 } }
```

```json
{ "DirDot": { "radius": 8000, "range": 700 } }
```

Default: `{ "Circle": { "radius": 10000 } }`.

## `DataAttackEffectType`

```json
"Target"
```

```json
"EnemyTarget"
```

```json
{ "EnemyAll": { "range": 40000 } }
```

Default: `Target`.

## `DataHealType`

```json
"Caster"
```

```json
"Any"
```

```json
"Ally"
```

```json
{ "AllyAll": { "range": 40000 } }
```

Default: `Any`.

## `DataRangeApplyType`

```json
"AroundCaster"
```

```json
{ "Forward": { "offset": 24000 } }
```

Default: `AroundCaster`.

`Forward` places the area in the direction of the action input, offset from the caster.


## `DataCastedType`

```json
"Bleed"
```

```json
"Poison"
```

```json
"Fire"
```

```json
"Heal"
```

Default: `Fire`.

Used by `AddCasted` to select the over-time effect category/icon and attack-type behavior. `Poison` uses the shield-ignoring DOT attack type; the other damaging casted types use normal DOT behavior.

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

Default: `Permanent`.