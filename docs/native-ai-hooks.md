# Native AI Hooks

Native Rust mods can customize two AI surfaces:

- Ban/pick candidate scoring.
- The final player input produced by the in-game player AI.

These hooks expose public context and input types, not the game's internal AI plan types. Internal types such as strategy, operation, or small-action plans are implementation details and may change without becoming part of the mod API. When a mod needs behavior that matches an internal small action, use the helper methods on `PlayerAiContext` to request a public `Input`.

## Registering AI Hooks

Register AI hooks from your native mod entry function:

```rust
use mod_api::*;

const MOD_ID: &str = "my_ai_mod";

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);
    reg.add_draft_score_hook(InvertDraftScoreHook);
    reg.add_player_input_ai(LowHpRecallInputAi::default());
    reg
}

declare_mod!(init);
```

If multiple mods register hooks, lower `priority()` values run first. Higher priority hooks run later and can replace the result produced by earlier hooks.

The bundled example native mod demonstrates both hooks: it inverts draft scores and replaces low-HP player input with run-away or recall input.

## Ban/Pick Score Hooks

Implement `ModDraftScoreHook` when you want to adjust how the draft AI scores ban or pick candidates.

The game computes its normal score first. Your hook then receives the current score and returns a `DraftScoreDecision`:

- `DraftScoreDecision::Pass`: keep the score unchanged.
- `DraftScoreDecision::Add(delta)`: add a score delta.
- `DraftScoreDecision::Replace(score)`: replace the score.

The draft AI still chooses the highest final score.

Example: invert every ban and pick score.

```rust
#[derive(Debug)]
struct InvertDraftScoreHook;

impl ModDraftScoreHook for InvertDraftScoreHook {
    fn id(&self) -> &str {
        "my_ai_mod:invert_draft_score"
    }

    fn score_ban(
        &self,
        _ctx: &DraftScoreContext,
        _candidate: usize,
        current_score: f32,
    ) -> DraftScoreDecision {
        DraftScoreDecision::Replace(-current_score)
    }

    fn score_pick(
        &self,
        _ctx: &DraftScoreContext,
        _candidate: usize,
        current_score: f32,
    ) -> DraftScoreDecision {
        DraftScoreDecision::Replace(-current_score)
    }
}
```

`DraftScoreContext` contains draft-side information such as the phase, existing picks and bans, available candidates, difficulty, and whether the game is exploring draft alternatives. The candidate value is the champion index used by the game draft candidate list.

Existing champion reworks keep the base champion id and draft candidate position stable. A ban/pick hook that already scores that candidate continues to see the same candidate value after the champion's gameplay data or runtime logic is replaced.

## Player Input AI Hooks

Implement `ModPlayerInputAi` when you want to replace the final `Input` selected for a player.

The hook runs after the game's normal player AI has produced its base input. Return:

- `PlayerInputDecision::Pass`: keep the base input.
- `PlayerInputDecision::Replace(input)`: use your input instead.

If the replacement input is invalid for the current frame, the game ignores it and keeps the normal behavior. Call `ctx.is_valid_input(&input)` when you build an input manually.

Example: when HP is below 50%, run away until it is safe to recall, then recall.

```rust
#[derive(Clone, Debug, Default)]
struct LowHpRecallInputAi;

impl ModPlayerInputAi for LowHpRecallInputAi {
    fn clone_box(&self) -> Box<dyn ModPlayerInputAi> {
        Box::new(self.clone())
    }

    fn id(&self) -> &str {
        "my_ai_mod:low_hp_recall"
    }

    fn matches(&self, _ctx: &PlayerAiInitContext) -> bool {
        true
    }

    fn think(
        &mut self,
        ctx: &mut PlayerAiContext<'_, '_, '_>,
        _base_input: Option<Input>,
    ) -> PlayerInputDecision {
        if !ctx.is_hp_below_percent(50) {
            return PlayerInputDecision::Pass;
        }

        if ctx.is_safe_to_recall() {
            if let Some(input) = ctx.get_recall_input() {
                return PlayerInputDecision::Replace(input);
            }
        }

        if let Some(input) = ctx.get_run_away_input() {
            return PlayerInputDecision::Replace(input);
        }

        PlayerInputDecision::Pass
    }
}
```

The helper calls above do not expose internal small-action objects. For example, `get_run_away_input()` asks the game to create the corresponding internal run-away behavior and returns only the resulting public `Input`.

## Player AI Context

`PlayerAiInitContext` is passed to `matches()` when the AI instance is attached to a player. Use it to limit a hook to a team, position, athlete, or champion.

`PlayerAiContext` is passed every simulation update. It provides stable read-only state and helper functions, including:

- `player_id()`, `athlete_id()`, `team()`, `position()`, and `champion_name()`.
- `tick()`, `hp()`, `max_hp()`, `hp_ratio_percent()`, and `is_hp_below_percent(percent)`.
- `is_safe_to_recall()`.
- `get_recall_input()`.
- `get_run_away_input()`.
- `get_run_away_without_skill_input()`.
- `is_valid_input(&input)`.

`is_safe_to_recall()` is a game-provided utility for recall decisions. It is intended for behavior selection, not as a permanent balance contract; the exact heuristic can evolve with the base AI.

## Compatibility Notes

- Rebuild the native DLL whenever the game SDK changes.
- Use the `Input` type and context helpers as the boundary. Do not depend on internal AI implementation details.
- Keep hooks small and deterministic. They run inside the match simulation.
- For multiplayer, all players should use the same enabled native mods and SDK-compatible DLLs so draft scoring and player-input replacement stay deterministic.
- If a hook panics, the game logs the error and falls back where possible, but the hook should still handle its own edge cases.
- Use a release-built DLL with the release game package.
