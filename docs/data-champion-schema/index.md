# Data Champion Schema

This folder is the full reference for `.data_champion` files.

Start with [Data-Only Champions](../data-champion.md) for a small working example. Use this reference when you need every field, enum value, effect type, and visual binding that the current data champion loader accepts.

## Pages

- [Champion File](champion-file.md): top-level `.data_champion` fields, stats, icons, required action slots, and existing champion reworks.
- [Actions](actions.md): `attack`, `skill`, `skill2`, and `ult` action schema.
- [Effects](effects.md): every `effect.type` value and its JSON shape.
- [Buffs and Stats](buffs-and-stats.md): `stat`, `growth`, `buff_state`, and buff duration schema.
- [Visual Bindings](visual-bindings.md): `view_effects`, `view_projectiles`, `view_buffs`, and how names connect to effects.
- [Enums](enums.md): accepted enum strings and externally-tagged enum objects.
- [Patchable Fields](patchable-fields.md): fields that can be adjusted by patch/balance systems.
- [Recipes](recipes.md): common patterns you can copy and adapt.

## JSON Conventions

Most data champion effects use an internally tagged form:

```json
{
  "type": "LinearProjectile",
  "name": "fire_skill",
  "speed": 4200,
  "range": 65000
}
```

Some helper enums use Rust/serde's externally tagged form. Unit values are strings:

```json
"Targeting"
```

Enum values with fields are objects:

```json
{ "Time": { "tick": 180 } }
```

Unless a page says a field is required, fields marked as defaultable may be omitted. For clarity, examples usually include important defaults such as `casting_target` and `applied_target` because their Rust defaults are not always what a mod author wants.
