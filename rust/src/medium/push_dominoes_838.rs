/*
 * @lc app=leetcode id=838 lang=rust
 *
 * [838] Push Dominoes
 */

struct Solution;

// @lc code=start
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Domino {
    Left,
    Right,
    Upright,
}

impl Domino {
    fn from_string(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            '.' => Self::Upright,
            _ => panic!("{} is not a valid position", c),
        }
    }

    fn into_char(self) -> char {
        match self {
            Self::Left => 'L',
            Self::Right => 'R',
            Self::Upright => '.',
        }
    }
}

struct Dominoes(Vec<Domino>);

impl Deref for Dominoes {
    type Target = Vec<Domino>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Dominoes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Dominoes {
    fn tick(&self) -> HashMap<usize, Domino> {
        let mut moves: HashMap<usize, Domino> = HashMap::new();
        for (i, d) in self.iter().enumerate() {
            match d {
                Domino::Right => {
                    if self.get(i + 1) == Some(&Domino::Upright) {
                        moves.insert(i + 1, Domino::Right);
                    }
                }
                Domino::Left => {
                    if i > 0 && self.get(i - 1) == Some(&Domino::Upright) {
                        moves
                            .entry(i - 1)
                            .and_modify(|prev_d| *prev_d = Domino::Upright)
                            .or_insert(Domino::Left);
                    }
                }
                Domino::Upright => {}
            };
        }

        moves.retain(|_, &mut v| v != Domino::Upright);
        assert!(moves.values().all(|&v| v != Domino::Upright));
        moves
        // moves.drain_filter(|_k, v| v == &Domino::Upright);
        // moves
    }

    fn apply_moves(&mut self, moves: &HashMap<usize, Domino>) -> bool {
        let mut made_move = false;
        moves.iter().for_each(|(&i, &d)| {
            if self.0[i] != d {
                self.0[i] = d;
                made_move = true
            }
        });
        made_move
    }

    fn new(dominoes: String) -> Self {
        Self(
            dominoes
                .chars()
                .map(Domino::from_string)
                .collect::<Vec<_>>(),
        )
    }
}

impl Display for Dominoes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.iter().map(|d| d.into_char()).collect::<String>();
        write!(f, "{}", s)
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn push_dominoes(dominoes: String) -> String {
        let mut dominoes = Dominoes::new(dominoes);
        let mut depth = 0;
        loop {
            let moves = dominoes.tick();
            if !dominoes.apply_moves(&moves) || depth > 200 {
                return dominoes.to_string();
            }
            depth += 1
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::push_dominoes(String::from("RR.L")),
            String::from("RR.L")
        );

        assert_eq!(
            Solution::push_dominoes(String::from("RL")),
            String::from("RL")
        );

        assert_eq!(
            Solution::push_dominoes(String::from(".L.R...LR..L..")),
            String::from("LL.RR.LLRRLL..")
        );

        assert_eq!(
            Solution::push_dominoes(String::from("...........L..")),
            String::from("LLLLLLLLLLLL..")
        )
    }
}
