/*
 * @lc app=leetcode id=1406 lang=rust
 *
 * [1406] Stone Game III
 */

// NOTE: this is a minimax game, use brute force

struct Solution {}

// @lc code=start

use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Player {
    Alice,
    Bob,
}

impl Player {
    pub fn as_winner(&self) -> Winner {
        Winner::Player(*self)
    }

    pub fn next(&self) -> Player {
        match self {
            Player::Alice => Player::Bob,
            Player::Bob => Player::Alice,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Winner {
    Player(Player),
    Tie,
}

impl ToString for Winner {
    fn to_string(&self) -> String {
        match self {
            Winner::Player(Player::Alice) => String::from("Alice"),
            Winner::Player(Player::Bob) => String::from("Bob"),
            Winner::Tie => String::from("Tie"),
        }
    }
}


#[allow(dead_code)]
impl Solution {
    fn minimax(stone_value: &[i32], cur_player: Player, scores: &HashMap<Player, i32>, best_scores: &mut HashMap<Player, HashMap<i32, i32>>, pos: &mut i32) -> Winner {
        if stone_value.is_empty() {
            let head = scores.values().next().unwrap();
            if scores.values().into_iter().all(|x| x == head) {
                Winner::Tie
            } else {
                scores
                    .iter()
                    .max_by(|a, b| a.1.cmp(b.1))
                    .map(|(k, _)| k.as_winner())
                    .unwrap()
            }
        } else {
            let mut tied = false;
            let choices = 1..=std::cmp::min(3, stone_value.len());
            for choice in choices {
                let (taken, rest) = stone_value.split_at(choice);
                let mut new_scores = scores.clone();
                new_scores
                    .entry(cur_player)
                    .and_modify(|x| *x += taken.iter().sum::<i32>());
                *pos += choice as i32;

                best_scores.entry(cur_player).and_modify(|hm| {
                    let cur_score = new_scores.get(&cur_player).unwrap();
                    hm.entry(*pos).and_modify(|x| *x = std::cmp::max(*x, *cur_score)).or_insert(*cur_score);
                });

                if best_scores.get(&cur_player).unwrap().get(pos) > new_scores.get(&cur_player) {
                    continue;
                }

                let winner = Solution::minimax(rest, cur_player.next(), &new_scores, best_scores, pos);
                if winner == cur_player.as_winner() {
                    return cur_player.as_winner();
                } else if winner == Winner::Tie {
                    tied = true;
                }
            }
            if tied {
                Winner::Tie
            } else {
                cur_player.next().as_winner()
            }
        }
    }

    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let mut scores: HashMap<Player, i32> = HashMap::new();
        scores.insert(Player::Alice, 0);
        scores.insert(Player::Bob, 0);
        let mut best_scores: HashMap<Player, HashMap<i32, i32>> = HashMap::new();
        best_scores.insert(Player::Alice, HashMap::new());
        best_scores.insert(Player::Bob, HashMap::new());
        let winner = Solution::minimax(&stone_value, Player::Alice, &scores, &mut best_scores, &mut 0);
        winner.to_string()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 7]), "Bob");
        assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, -9]), "Alice");
        assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 6]), "Tie");
    }
}
