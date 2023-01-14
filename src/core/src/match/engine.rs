use crate::r#match::squad::{PositionType, Squad, SquadPlayer, POSITION_POSITIONING};

pub struct FootballEngine {
    pub home_squad: Squad,
    pub away_squad: Squad,
}

impl FootballEngine {
    pub fn new(home_squad: Squad, away_squad: Squad) -> Self {
        FootballEngine {
            home_squad,
            away_squad,
        }
    }

    pub fn play(self) -> FootballMatchDetails {
        Field::new(150, 100, self.home_squad, self.away_squad).play()
    }
}

fn setup_players(home_squad: Squad, away_squad: Squad) -> Vec<(SquadPlayer, FieldPosition)> {
    let mut players: Vec<(SquadPlayer, FieldPosition)> = Vec::new();

    // home
    home_squad.main_squad.into_iter().for_each(|home_player| {
        POSITION_POSITIONING
            .iter()
            .filter(|(positioning, _, _)| *positioning == home_player.tactics_position)
            .map(|(_, home_position, _)| home_position)
            .for_each(|position| {
                if let PositionType::Home(x, y) = position {
                    players.push((home_player, FieldPosition::new(*x, *y)));
                }
            });
    });

    // away
    away_squad.main_squad.into_iter().for_each(|away_player| {
        POSITION_POSITIONING
            .iter()
            .filter(|(positioning, _, _)| *positioning == away_player.tactics_position)
            .map(|(_, _, away_position)| away_position)
            .for_each(|position| {
                if let PositionType::Away(x, y) = position {
                    players.push((away_player, FieldPosition::new(*x, *y)));
                }
            });
    });

    players
}

#[derive(Debug, Clone)]
pub struct FootballMatchDetails {
    pub score: Score,
    pub players_positions: Vec<PlayerPositionData>,
}

impl FootballMatchDetails {
    pub fn new(score: Score, players_positions: Vec<PlayerPositionData>) -> Self {
        FootballMatchDetails {
            score,
            players_positions,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerPositionData {
    pub player_id: u32,
    pub x: i16,
    pub y: i16,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct Score {
    pub home: i32,
    pub away: i32,
}

pub struct Field {
    pub width: u16,
    pub height: u16,
    pub ball: Ball,
    pub players: Vec<(SquadPlayer, FieldPosition)>,
}

impl Field {
    pub fn new(width: u16, height: u16, home_squad: Squad, away_squad: Squad) -> Self {
        let mut players_container =
            Vec::with_capacity(home_squad.main_squad.len() + away_squad.main_squad.len());

        for (player, position) in setup_players(home_squad, away_squad) {
            players_container.push((player, position));
        }

        Field {
            width,
            height,
            ball: Ball::new(width as i16 / 2, height as i16 / 2),
            players: players_container,
        }
    }

    pub fn play(&mut self) -> FootballMatchDetails {
        let mut match_details = FootballMatchDetails {
            score: Score { home: 0, away: 0 },
            players_positions: Vec::new(),
        };

        self.play_first_half(&mut match_details);

        self.play_rest(&mut match_details);

        self.play_second_half(&mut match_details);

        match_details
    }

    fn play_first_half(&mut self, match_details: &mut FootballMatchDetails) {
        let ms_step: i16 = 100;
        let mut current_time: u32 = 0;

        while current_time <= 45 * 60 * 1000 {
            self.ball.move_ball();

            // update player positions and decisions
            for (player, position) in self.players.iter_mut() {
                //player.decision_tree.predict(self.ball, position);
                position.x += player.speed * ms_step;
                position.y += player.speed * ms_step;
            }

            // check for collision with the ball
            for (player, position) in &mut self.players {
                if Self::is_collision(&self.ball.position, position) {
                    player.has_ball = true;
                } else {
                    player.has_ball = false;
                }
            }

            // check for goal
            if self.ball.position.x >= self.width as i16 {
                match_details.score.home += 1;
            } else if self.ball.position.x <= 0 {
                match_details.score.away += 1;
            }

            current_time += ms_step as u32;

            self.write_positions(match_details);
        }
    }

    fn is_collision(ball_position: &FieldPosition, player_position: &FieldPosition) -> bool {
        let threshold = 10;
        (ball_position.x - player_position.x).abs() <= threshold
            && (ball_position.y - player_position.y).abs() <= threshold
    }

    fn play_rest(&mut self, _match_details: &mut FootballMatchDetails) {}

    fn play_second_half(&mut self, _match_details: &mut FootballMatchDetails) {}

    pub fn write_positions(&self, match_details: &mut FootballMatchDetails) {
        self.players.iter().for_each(|(player, position)| {
            match_details.players_positions.push(PlayerPositionData {
                player_id: player.player_id,
                x: position.x,
                y: position.y,
                timestamp: 0,
            });
        });
    }
}

pub struct Ball {
    pub position: FieldPosition,
    pub speed: i16,
    pub direction: FieldPosition,
}

impl Ball {
    pub fn new(x: i16, y: i16) -> Self {
        Ball {
            position: FieldPosition { x, y },
            speed: 0,
            direction: FieldPosition { x: 0, y: 0 },
        }
    }

    pub fn move_ball(&mut self) {
        self.position.x += self.speed * self.direction.x;
        self.position.y += self.speed * self.direction.y;
    }
}

pub struct FieldPosition {
    pub x: i16,
    pub y: i16,
}

impl FieldPosition {
    pub fn new(x: i16, y: i16) -> Self {
        FieldPosition { x, y }
    }
}

pub enum MatchEvent {
    MatchPlayed(u32, bool, u8),
    Goal(u32),
    Assist(u32),
    Injury(u32),
}
