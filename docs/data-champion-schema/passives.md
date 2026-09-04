# Passives

Data champions can attach native passive behaviors by name — the same decoupling `Native` effects use: the definition stays JSON, the mechanics run native code. The base game's own champion passives (the ogre/dancer-style stacking ones included) are registered under the names below.

Three slots exist on the champion file:

- `passive`: active from spawn.
- `passive_skill2`: activates once skill2 is learned.
- `passive_ult`: activates once the ultimate is learned.

Each slot takes a `DataPassiveDef`:

```json
{
  "passive": {
    "passive_ref": "ogre",
    "params": { "hit_hp": 5 }
  },
  "stack_skill_index": 0
}
```

- `passive_ref`: registered passive name.
- `params`: integer map. Every listed param is **required** — a missing or negative value logs one warning in `log.log` and the passive is skipped (the champion still loads).
- An unknown `passive_ref` behaves the same way: one warning, no passive.

`stack_skill_index` picks which skill icon (0 = skill, 1 = skill2, 2 = ult) shows the stack counter in the match UI, for passives that stack.

## Death persistence

Passive state lives on the player, not on the champion entity, and is re-applied on every respawn. Stacks accumulated before dying survive death — exactly how the base game's own stacking champions work. No extra setup is needed.

## Registered passives

Durations are in simulation ticks (60 per second); ranges are world units. Copy param magnitudes from the base champion sheet, not from thin air.

| `passive_ref` | Params | Behavior |
| --- | --- | --- |
| `ogre` | `hit_hp` | Every time a champion hits the owner: max HP permanently +`hit_hp`, +1 stack. |
| `dancer` | `vamp` | Permanent `vamp` lifesteal; every kill adds one persistent stack (shown on the stack counter). |
| `ghost` | `heal`, `add_attack`, `add_attack_speed` | Kills and assists heal the owner for `heal` and add one permanent stack of +`add_attack` attack and +`add_attack_speed` attack speed; at level 5+ they also reset skill cooldowns. |
| `circus_blade` | `charge_count` | Basic attacks that hit champions refund the skill's cooldown in `charge_count` steps — `charge_count` hits equal one full cooldown. |
| `gunner` | `move_speed_up`, `move_speed_up_duration` | Basic attacks on champions grant +`move_speed_up` move speed for `move_speed_up_duration` ticks. |
| `hunter` | `recast_duration`, `kill_extend_count` | After casting the ultimate it can be recast within `recast_duration` ticks (2 uses by default); kills raise the window's uses by `kill_extend_count`. |
| `berserker` | `cooltime_reduction`, `max_cooltime_reduction` | Ultimate cooldown speeds up with missing HP: reduction = missing-HP% × `cooltime_reduction`, capped at `max_cooltime_reduction`. Typically used as `passive_ult`. |
| `poison_dart_hunter` | `add_move_speed`, `range` | While a poisoned enemy is within `range`, gain +`add_move_speed` move speed when moving away from it. Typically used as `passive_skill2`. |
| `swordman` | none | While the `swordman_ult` buff is on the owner, kills reset the ultimate cooldown. Only useful together with logic that applies that named buff. |
| `vampire` | none | Persists the vampire kit's stack counter across respawns. The stacks are added by the vampire kit itself, so on its own this does nothing visible. |

More passives can be registered in later game versions without breaking existing files.

## Mod-Registered Passives

Native mods can add their own names to this registry with the stable API's `add_native_passive(name, passive)` — see the [Stable API Reference](../stable-api-reference.md). A data champion then references the mod's passive exactly like a built-in one:

```json
{ "passive": { "passive_ref": "my_mod:frenzy", "params": { "stacks": 5 } } }
```

The `params` object reaches the mod's passive instance as sorted-key JSON (`StablePassive::configure`), so one registered passive can serve many data champions with different numbers. If the registering mod is disabled, the reference degrades like an unknown name: one warning, no passive, the champion still loads.
