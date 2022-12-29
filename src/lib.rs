pub trait TennisGame {
    fn clear(&mut self);
    fn won_point2(&mut self, player: Player);
    fn won_point(&mut self, player_name: &str);
    fn get_score(&self) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Player {
    One,
    Two,
}

#[derive(Debug, Clone)]
pub struct TennisGame1 {
    state: TennisGame1State,
}
#[derive(Debug, Clone)]
enum TennisGame1State {
    InRange { score1: u8, score2: u8 },
    Deuce { advantage: i8 },
    Fin { winner: Player },
}
impl TennisGame1 {
    pub fn new() -> Self {
        TennisGame1 {
            state: TennisGame1State::InRange {
                score1: 0,
                score2: 0,
            },
        }
    }
    fn check(&self) {
        match self.state {
            TennisGame1State::InRange { score1, score2 } => {
                debug_assert!(score1 < 4);
                debug_assert!(score2 < 4);
                debug_assert!(score1 + score2 < 6);
            }
            TennisGame1State::Deuce { advantage: diff } => {
                debug_assert!(-1 <= diff && diff <= 1);
            }
            TennisGame1State::Fin { .. } => {}
        }
    }
}
impl TennisGame for TennisGame1 {
    fn clear(&mut self) {
        *self = Self::new();
    }
    fn won_point2(&mut self, player: Player) {
        match &mut self.state {
            TennisGame1State::InRange { score1, score2 } => match player {
                Player::One => {
                    *score1 += 1;
                }
                Player::Two => {
                    *score2 += 1;
                }
            },
            TennisGame1State::Deuce { advantage } => {
                *advantage += match player {
                    Player::One => 1,
                    Player::Two => -1,
                }
            }
            TennisGame1State::Fin { .. } => panic!("Cannot continue the game"),
        }
        if let TennisGame1State::InRange { score1, score2 } = &mut self.state {
            if *score1 >= 4 || *score2 >= 4 || *score1 + *score2 >= 6 {
                self.state = TennisGame1State::Deuce {
                    advantage: *score1 as i8 - *score2 as i8,
                }
            }
        }
        if let TennisGame1State::Deuce { advantage } = &mut self.state {
            if *advantage >= 2 {
                self.state = TennisGame1State::Fin {
                    winner: Player::One,
                }
            } else if *advantage <= -2 {
                self.state = TennisGame1State::Fin {
                    winner: Player::Two,
                }
            }
        }
        self.check();
    }
    fn won_point(&mut self, player_name: &str) {
        self.won_point2(match player_name {
            "player1" => Player::One,
            "player2" => Player::Two,
            _ => panic!("Invalid player_name: {:?}", player_name),
        })
    }
    fn get_score(&self) -> String {
        match self.state {
            TennisGame1State::InRange { score1, score2 } => {
                if score1 == score2 {
                    format!("{}-All", score_name(score1))
                } else {
                    format!("{}-{}", score_name(score1), score_name(score2))
                }
            }
            TennisGame1State::Deuce { advantage } => match advantage {
                1 => "Advantage player1".to_owned(),
                0 => "Deuce".to_owned(),
                -1 => "Advantage player2".to_owned(),
                _ => unreachable!(),
            },
            TennisGame1State::Fin { winner } => {
                format!(
                    "Win for {}",
                    match winner {
                        Player::One => "player1",
                        Player::Two => "player2",
                    }
                )
            }
        }
    }
}

fn score_name(score: u8) -> &'static str {
    match score {
        0 => "Love",
        1 => "Fifteen",
        2 => "Thirty",
        3 => "Forty",
        _ => unreachable!(),
    }
}
