use mod_api::*;

const MOD_ID: &str = "momentum_suppression";
const KILL_THRESHOLD: usize = 5;

const PLAYER_ATTACK: i32 = 30;
const PLAYER_MAGIC_POWER: i32 = 35;
const PLAYER_DEFENCE: i32 = 50;
const PLAYER_MAGIC_RESISTANCE: i32 = 50;
const PLAYER_MOVE_SPEED_MULT: i32 = 50;

const AI_REWARD_PERCENT: i32 = 5;

fn init(_ctx: &GameCtx) -> ModRegistration {
    ModRegistration::new(MOD_ID)
}

declare_mod!(init);

#[derive(Clone, Debug, Default)]
struct MomentumState {
    winner_team: Option<usize>,
    last_seen_kill_log_count: usize,
}

impl MomentumState {
    fn update(&mut self, ctx: &mut GameCtx, player_team_id: usize) {
        if ctx.is_end() {
            self.reset();
            return;
        }

        if self.winner_team.is_none() {
            self.detect_winner(ctx);
        }

        if let Some(team) = self.winner_team {
            self.apply_to_team(ctx, team, team == player_team_id);
        }
    }

    fn reset(&mut self) {
        self.winner_team = None;
        self.last_seen_kill_log_count = 0;
    }

    fn detect_winner(&mut self, ctx: &GameCtx) {
        let kill_log_count = ctx.kill_log_count();
        if kill_log_count == self.last_seen_kill_log_count {
            return;
        }

        let mut team_kills = [0usize; 2];

        for index in 0..kill_log_count {
            let entry = ctx.kill_log_at(index);
            let killer_team = entry.killer_team;

            if killer_team < team_kills.len() {
                team_kills[killer_team] += 1;

                if team_kills[killer_team] >= KILL_THRESHOLD {
                    self.winner_team = Some(killer_team);
                    break;
                }
            }
        }

        self.last_seen_kill_log_count = kill_log_count;
    }

    fn apply_to_team(&self, ctx: &mut GameCtx, team: usize, is_player_team: bool) {
        for index in 0..ctx.entity_count() {
            let Some(entity) = ctx.entity_at(index) else {
                continue;
            };

            if !entity.is_champion() || entity.team() != team || !entity.is_alive() {
                continue;
            }

            let entity_id = entity.id();
            drop(entity);

            ctx.add_buff(entity_id, momentum_buff(is_player_team));
        }
    }
}

fn momentum_buff(is_player_team: bool) -> BuffState {
    let scale = if is_player_team { 100 } else { AI_REWARD_PERCENT };

    BuffState {
        duration: BuffType::Permanent,
        attack: scaled(PLAYER_ATTACK, scale),
        magic_power: scaled(PLAYER_MAGIC_POWER, scale),
        defence: scaled(PLAYER_DEFENCE, scale),
        magic_resistance: scaled(PLAYER_MAGIC_RESISTANCE, scale),
        move_speed_mult: scaled(PLAYER_MOVE_SPEED_MULT, scale),
        ..Default::default()
    }
}

fn scaled(value: i32, percent: i32) -> i32 {
    if percent >= 100 {
        return value;
    }

    let adjusted = (value * percent + 99) / 100;
    adjusted.max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ai_reward_is_five_percent_rounded_up() {
        assert_eq!(scaled(30, 5), 2);
        assert_eq!(scaled(35, 5), 2);
        assert_eq!(scaled(50, 5), 3);
    }
}
