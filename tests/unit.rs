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
fn test_game14() {
    test_game(0, 3, "Love-Forty");
}

#[test]
fn test_game15() {
    test_game(1, 0, "Fifteen-Love");
}

#[test]
fn test_game6() {
    test_game(1, 1, "Fifteen-All");
}

#[test]
fn test_game16() {
    test_game(1, 2, "Fifteen-Thirty");
}

#[test]
fn test_game17() {
    test_game(1, 3, "Fifteen-Forty");
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
fn test_game7() {
    test_game(2, 2, "Thirty-All");
}

#[test]
fn test_game18() {
    test_game(2, 3, "Thirty-Forty");
}

#[test]
fn test_game19() {
    test_game(3, 0, "Forty-Love");
}

#[test]
fn test_game20() {
    test_game(3, 1, "Forty-Fifteen");
}

#[test]
fn test_game21() {
    test_game(3, 2, "Forty-Thirty");
}

#[test]
fn test_game8() {
    test_game(3, 3, "Deuce");
}

#[test]
fn test_game9() {
    test_game(4, 4, "Deuce");
}

#[test]
fn test_game10() {
    test_game(4, 5, "Advantage player2");
}

#[test]
fn test_game11() {
    test_game(5, 4, "Advantage player1");
}

#[test]
fn test_game12() {
    test_game(4, 6, "Win for player2");
}

#[test]
fn test_game13() {
    test_game(6, 4, "Win for player1");
}
