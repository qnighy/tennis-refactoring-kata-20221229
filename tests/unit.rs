use std::cmp;

use rust::{TennisGame, TennisGame1};

fn test_game(pt1: u32, pt2: u32, expected: &str) {
    let mut game = TennisGame1::new();
    let max = cmp::max(pt1, pt2);
    for i in 0..max {
        if i < pt1 {
            game.won_point("player1");
        }
        if i < pt2 {
            game.won_point("player2");
        }
    }
    assert_eq!(game.get_score(), expected);
}

#[test]
fn test_game1() {
    test_game(0, 0, "Love-All");
}

#[test]
fn test_game2() {
    test_game(0, 1, "Love-Fifteen");
}

#[test]
fn test_game3() {
    test_game(0, 2, "Love-Thirty");
}

#[test]
fn test_game4() {
    test_game(2, 0, "Thirty-Love");
}

#[test]
fn test_game5() {
    test_game(2, 1, "Thirty-Fifteen");
}

#[test]
fn test_game6() {
    test_game(1, 1, "Fifteen-All");
}
#[test]
fn test_game7() {
    test_game(2, 2, "Thirty-All");
}
