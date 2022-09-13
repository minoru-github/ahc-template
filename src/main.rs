#![allow(unused)]
use itertools::Itertools;
use my_lib::XY;
use num::{integer::Roots, Integer, ToPrimitive};
use proconio::{
    input,
    marker::{Bytes, Chars},
};
use rand::prelude::*;
use rand_pcg::Mcg128Xsl64;
use std::{
    clone,
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    iter::FromIterator,
    ops::Range,
    ops::*,
    slice::SliceIndex,
};
use superslice::Ext;

fn main() {
    Sim::new().run();
}

#[derive(Debug, Clone)]
pub struct State {
    score: usize,
}

impl State {
    fn new() -> Self {
        State { score: 0 }
    }

    pub fn change(&mut self, rng: &mut Mcg128Xsl64) {
        //let val = rng.gen_range(-3, 4);
        //self.x += val;
    }

    fn output(&self) {
        eprintln!("{}", self.score);
    }
}

#[derive(Debug, Clone)]
pub struct Sim {
    input: Input,
}

impl Sim {
    fn new() -> Self {
        let input = Input::read();
        Sim { input }
    }

    fn compute_score(&self, state: &mut State) {
        //state.score = 0;
    }

    pub fn run(&self) {
        let mut rng: Mcg128Xsl64 = rand_pcg::Pcg64Mcg::new(890482);

        let mut state = State::new();
        let mut best_state = state.clone();
        while my_lib::time::update() < 0.3 {
            // 近傍探索
            state.change(&mut rng);

            // スコア計算
            self.compute_score(&mut state);

            // 状態更新
            solver::mountain(&mut best_state, &mut state);
        }

        best_state.output();
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    n: usize,
}

impl Input {
    fn read() -> Self {
        // a : 型
        // (a,b) : (型, 型)
        // a_vec : [型;サイズ]
        // a_vec2 : [[型;サイズ];サイズ]
        // S : [char; n] or Chars ← Vec<char>
        // s_vec : [String; n]
        // bytes : Bytes ← Vec<u8>
        input! {
            n:usize,
        };

        Input { n }
    }

    fn debug(result: &Result<Input, &str>) {
        println!("{:?}", result);
    }
}

mod solver {
    use super::State;
    pub fn mountain(best_state: &mut State, state: &mut State) {
        //! bese_state(self)を更新する。<br>
        //! 新しいStateのほうが悪いStateの場合は、stateをbest_stateに戻す。

        // 最小化の場合は > , 最大化の場合は < 。
        if best_state.score > state.score {
            *best_state = state.clone();
        } else {
            *state = best_state.clone();
        }
    }
}

mod my_lib {
    //! 基本的に問題によらず変えない自作ライブラリ群
    use super::*;
    pub mod time {
        //! 時間管理モジュール
        pub fn update() -> f64 {
            static mut STARTING_TIME_MS: Option<f64> = None;
            let t = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap();
            let time_ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
            unsafe {
                let now = match STARTING_TIME_MS {
                    Some(starting_time_ms) => time_ms - starting_time_ms,
                    None => {
                        STARTING_TIME_MS = Some(time_ms);
                        0.0 as f64
                    }
                };
                now
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct XY {
        y: usize, // ↓
        x: usize, // →
        width: usize,
    }

    impl XY {
        pub fn new(x: usize, y: usize, width: usize) -> Self {
            XY { x, y, width }
        }

        pub fn to_1d(&self) -> usize {
            self.y * self.width + self.x
        }

        pub fn to_2d(index: usize, width: usize) -> Self {
            XY {
                x: index % width,
                y: index / width,
                width,
            }
        }
    }

    impl Add for XY {
        type Output = Result<XY, &'static str>;
        fn add(self, rhs: Self) -> Self::Output {
            let (x, y) = if cfg!(debug_assertions) {
                // debugではオーバーフローでpanic発生するため、オーバーフローの溢れを明確に無視する(※1.60場合。それ以外は不明)
                (self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y))
            } else {
                (self.x + rhs.x, self.y + rhs.y)
            };

            if x >= self.width || y >= self.width {
                Err("out of range")
            } else {
                Ok(XY {
                    x,
                    y,
                    width: self.width,
                })
            }
        }
    }
}
