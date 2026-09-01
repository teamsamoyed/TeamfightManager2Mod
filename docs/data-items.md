# Data-Only Items

Items no longer require a native DLL. A `.data_item` file carries the whole static definition — price, tier, stat block, upgrade edges, tags — and coded behaviors are borrowed by name from the native item proc registry, the same way `.data_champion` actions borrow `Native` effects.

Place the file anywhere under the mod folder, commonly:

```text
mods/my_mod/item/vampire_blade.data_item
```

A stat-only item needs no procs at all:

```json
{
  "key": "vampire_blade",
  "price": 900,
  "tier": 4,
  "category": "AD",
  "stat": {
    "attack": 30,
    "vamp": 12
  }
}
```

## Top-Level Fields

| Field | Type | Required | Default | Notes |
| --- | --- | --- | --- | --- |
| `key` | string | yes | none | Unique item id. Keep stable after release — saves and builds refer to it. |
| `icon` | string | no | `key` | Tag in the item icon sheet. |
| `price` | number | yes | none | Gold cost. Copy magnitudes from `mods/base/setting/item_setting.item_setting`, not from thin air. |
| `tier` | number | yes | none | Item tier. The match AI treats tier 4+ items as final build targets. |
| `stat` | `BuffState` | no | all zero | Stat block granted while owned — the same fields as a buff (see [Buffs and Stats](data-champion-schema/buffs-and-stats.md)), including `vamp`, `damage_reflect`, `cc_immune`, and the rest. |
| `next_tier` | string[] | no | `[]` | Keys of the items this one upgrades into. |
| `tags` | `ItemTag[]` | no | `[]` | Classification tags. |
| `category` | `ItemCategory` | no | `AD` | Shop/build category. |
| `procs` | `DataItemProcDef[]` | no | `[]` | Named native behaviors — see below. |

The item's option text comes from i18n key `<key>.option` in the `asset/base/text/item` namespace, so ship it with a text merge like any other translation (see [Asset Overrides and i18n](asset-overrides-and-i18n.md)).

## Borrowing Behaviors: `procs`

Each entry references a registered native item proc by name, with JSON params:

```json
{
  "key": "burn_blade",
  "price": 1100,
  "tier": 4,
  "category": "AD",
  "stat": { "attack": 25 },
  "procs": [
    { "proc_ref": "flame_shroud", "params": { "damage": 15, "range": 30000 } }
  ]
}
```

`params` overlays the proc's own fields on top of its defaults — the field names below are the exact JSON keys. If you leave out `key` inside `params`, the item's own `key` is filled in automatically (some behaviors use it to find visuals and text). An unknown `proc_ref` or rejected params logs one warning in `log.log` and the proc is skipped; the item still loads with its stat block.

An item can stack multiple procs; hooks fire in list order.

### Registered procs

Reusable behaviors from the base game's own items. Durations are in simulation ticks (60 per second); ranges are world units.

| `proc_ref` | Fires on | Params | Behavior |
| --- | --- | --- | --- |
| `hp_regen` | every tick | `flat_regen`, `max_hp_regen_ratio` | Regenerates flat HP plus a percentage of max HP. |
| `giants_horn_shard` | every tick | `flat_regen`, `max_hp_regen_ratio`, `flat_aoe_damage`, `max_hp_aoe_ratio`, `aoe_range` | The regen above, plus periodic damage (flat + % of the owner's max HP) to enemies within `aoe_range`. |
| `impregnable_fortress` | being hit | `flat_damage`, `defence_ratio` | Reflects `flat_damage` plus `defence_ratio`% of the owner's armor back at the attacker. |
| `aura_veil` | every tick | `magic_power`, `range` | Aura: allies within `range` gain `magic_power` magic power. |
| `cursed_dagger` | landing a hit | `heal_reduce` | Applies `heal_reduce` healing reduction to the target. |
| `flame_shroud` | every tick | `damage`, `range` | Periodically burns enemies within `range` for `damage`. |
| `growing_armor` | kills | `growing_hp`, `max_stack` | Each kill permanently adds `growing_hp` max HP, up to `max_stack` stacks. |
| `guardian_blade` | being hit | `hp_cut`, `shield_amount`, `shield_duration_tick`, `cooldown_tick` | When damage drops the owner below the `hp_cut` threshold, grants a `shield_amount` shield for `shield_duration_tick`, once per `cooldown_tick`. |
| `sacrificial_shield` | every tick | `range`, `damage_share_ratio` | Shares damage with allies within `range` at `damage_share_ratio` — the sacrificial-shield behavior. |
| `shatter_hammer` | landing a hit | `shield_break` | Destroys up to `shield_break` of the target's shield. |
| `slowing_shield` | being hit | `value`, `duration_tick` | Slows the attacker's attack speed by `value` for `duration_tick`. |
| `weakening_cloak` | every tick | `magic_resistance`, `range` | Enemies within `range` lose `magic_resistance` magic resist. |

More behaviors (including the tier-5 legendary actives) can be registered in later game versions without breaking existing files — missing names degrade to a skipped proc.

## How the Item Enters the Game

A registered data item joins the same pool as native mod items: it appears in the match item list while the mod is enabled, the purchase AI considers it, and disabling the mod deactivates it (existing saves keep the entry but it can no longer be bought). To make the AI actually build toward it, wire it into the upgrade graph — set its own `next_tier`, and add your key to a base piece's `next_tier` with a [merge override](asset-overrides-and-i18n.md) on `asset/base/setting/item_setting` — give it a sane `price`/`tier`, or steer builds directly with the stable API's item-build hook (`score_item` / `decide_build`).

Custom visuals for proc-like flair (played via named view effects) can ship in a standalone [`.view_effects` file](data-champion-schema/visual-bindings.md#standalone-view-bindings-view_effects-files) — no champion definition needed.
