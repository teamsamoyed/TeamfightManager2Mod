# Momentum Suppression

Momentum Suppression is a native Teamfight Manager 2 mod concept for a League-style early momentum reward.

## Rule

Each set judges momentum independently.

- The first team to reach 5 total kills in a set triggers the momentum event.
- The winning team receives a temporary combat buff until that set ends.
- When the set ends, the winner state and buff state reset.
- The next set starts from 0 kills and judges the first 5-kill team again.

## Initial Test Values

The first implementation intentionally starts overtuned so the effect is easy to notice during playtests.

Player team reward:

- Attack +30
- Magic power +35
- Defence +50
- Magic resistance +50
- Move speed +50%

AI team reward:

- 5% of the player team reward, rounded up to keep small values visible.

Gold acquisition +25% is part of the design target, but the public API docs for `BuffState` do not expose a gold acquisition multiplier. It should be implemented only after confirming an SDK-supported way to modify per-set gold gain or award bonus gold safely.

## Implementation Note

The public API docs expose kill log reads through `GameCtx::kill_log_count` / `GameCtx::kill_log_at` and combat stat buffs through `GameCtx::add_buff`.

The current source keeps the core rule and buff math isolated so it can be connected to the narrowest available simulation hook in the matching Mod SDK. If the SDK exposes a global per-tick simulation hook, wire `MomentumState::update` there. If not, the game/mod API needs a small global combat extension point for this mod to affect all existing champions without requiring a custom item or custom champion.
