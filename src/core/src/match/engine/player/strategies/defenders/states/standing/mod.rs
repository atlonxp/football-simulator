use nalgebra::Vector3;

use crate::r#match::defenders::states::DefenderState;
use crate::r#match::{
    ConditionContext, MatchPlayerLite, StateChangeResult, StateProcessingContext,
    StateProcessingHandler,
};

const INTERCEPTION_DISTANCE: f32 = 200.0;
const CLEARING_DISTANCE: f32 = 50.0;
const STANDING_TIME_LIMIT: u64 = 300;
const WALK_DISTANCE_THRESHOLD: f32 = 15.0;
const MARKING_DISTANCE: f32 = 15.0;
const FIELD_THIRD_THRESHOLD: f32 = 0.33;

#[derive(Default)]
pub struct DefenderStandingState {}

impl StateProcessingHandler for DefenderStandingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let ball_ops = ctx.ball();
        let team_ops = ctx.team();

        if ctx.player.has_ball(ctx) {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::Running,
            ));
        } else {
            if ctx.ball().on_own_side() {
                if ball_ops.is_towards_player_with_angle(0.8)
                    && ball_ops.distance() < INTERCEPTION_DISTANCE
                {
                    return Some(StateChangeResult::with_defender_state(
                        DefenderState::Intercepting,
                    ));
                }

                if !team_ops.is_control_ball() && ball_ops.distance() < 150.0 {
                    return Some(StateChangeResult::with_defender_state(
                        DefenderState::Pressing,
                    ));
                }
            }
        }
        if ball_ops.distance() < 200.0 {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::Tackling,
            ));
        }

        if self.should_push_up(ctx) {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::PushingUp,
            ));
        }

        if self.should_hold_defensive_line(ctx) {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::HoldingLine,
            ));
        }

        if self.should_cover_space(ctx) {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::Covering,
            ));
        }

        // Walk or hold line more readily on attacking side
        if self.should_transition_to_walking(ctx) {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::Walking,
            ));
        }
        if ctx.in_state_time > 30 {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::Walking,
            ));
        }

        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, _ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::zeros())
    }

    fn process_conditions(&self, _ctx: ConditionContext) {
        // Implement condition processing if needed
    }
}

impl DefenderStandingState {
    fn should_transition_to_walking(&self, ctx: &StateProcessingContext) -> bool {
        let player_ops = ctx.player();
        let ball_ops = ctx.ball();

        let is_tired = player_ops.is_tired();
        let standing_too_long = ctx.in_state_time > STANDING_TIME_LIMIT;
        let ball_far_away = ball_ops.distance() > INTERCEPTION_DISTANCE * 2.0;

        let no_immediate_threat = ctx
            .players()
            .opponents()
            .nearby(CLEARING_DISTANCE)
            .next()
            .is_some();

        let close_to_optimal_position =
            player_ops.distance_from_start_position() < WALK_DISTANCE_THRESHOLD;
        let team_in_control = ctx.team().is_control_ball();

        (is_tired || standing_too_long)
            && (ball_far_away || close_to_optimal_position)
            && no_immediate_threat
            && team_in_control
    }

    fn should_push_up(&self, ctx: &StateProcessingContext) -> bool {
        let ball_ops = ctx.ball();
        let player_ops = ctx.player();

        let ball_in_attacking_third = ball_ops.distance_to_opponent_goal()
            < ctx.context.field_size.width as f32 * FIELD_THIRD_THRESHOLD;
        let team_in_possession = ctx.team().is_control_ball();
        let defender_not_last_man = !self.is_last_defender(ctx);

        ball_in_attacking_third
            && team_in_possession
            && defender_not_last_man
            && player_ops.distance_from_start_position()
                < ctx.context.field_size.width as f32 * 0.25
    }

    fn should_hold_defensive_line(&self, ctx: &StateProcessingContext) -> bool {
        let ball_ops = ctx.ball();

        let defenders: Vec<MatchPlayerLite> = ctx.players().teammates().defenders().collect();
        let avg_defender_x =
            defenders.iter().map(|d| d.position.x).sum::<f32>() / defenders.len() as f32;

        (ctx.player.position.x - avg_defender_x).abs() < 5.0
            && ball_ops.distance() > INTERCEPTION_DISTANCE
            && !ctx.team().is_control_ball()
    }

    fn should_cover_space(&self, ctx: &StateProcessingContext) -> bool {
        let ball_ops = ctx.ball();
        let player_ops = ctx.player();

        let ball_in_middle_third = ball_ops.distance_to_opponent_goal()
            > ctx.context.field_size.width as f32 * FIELD_THIRD_THRESHOLD
            && ball_ops.distance_to_own_goal()
                > ctx.context.field_size.width as f32 * FIELD_THIRD_THRESHOLD;
        let no_immediate_threat = ctx
            .players()
            .opponents()
            .nearby(MARKING_DISTANCE)
            .next()
            .is_some();

        let not_in_optimal_position =
            player_ops.distance_from_start_position() > WALK_DISTANCE_THRESHOLD;

        ball_in_middle_third && no_immediate_threat && not_in_optimal_position
    }

    fn is_last_defender(&self, ctx: &StateProcessingContext) -> bool {
        ctx.players()
            .teammates()
            .defenders()
            .all(|d| d.position.x >= ctx.player.position.x)
    }
}
