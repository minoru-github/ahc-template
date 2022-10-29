#![allow(unused)]
use itertools::Itertools;
use my_lib::*;
use num::{integer::Roots, Integer, ToPrimitive};
use procon_input::*;
use proconio::{
    fastout,
    //input,
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

//#[fastout]
fn main() {
    let start_time = my_lib::time::update();

    Sim::new().run();

    let end_time = my_lib::time::update();
    let duration = end_time - start_time;
    eprintln!("{:?} ", duration);
}

#[derive(Debug, Clone)]
pub struct State {
    score: usize,
}

impl State {
    fn new() -> Self {
        State { score: 0 }
    }

    fn change(&mut self, output: &mut Output, rng: &mut Mcg128Xsl64) {
        //let val = rng.gen_range(-3, 4);
        //self.x += val;
    }

    fn compute_score(&mut self) {
        //self.score = 0;
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

    pub fn run(&mut self) {
        let mut rng: Mcg128Xsl64 = rand_pcg::Pcg64Mcg::new(890482);
        let mut cnt = 0 as usize; // 試行回数

        //let mut initial_state = State::new();
        let mut best_output = Output::new();
        let mut best_state = State::new();
        best_state.compute_score();

        'outer: loop {
            let current_time = my_lib::time::update();
            if current_time >= my_lib::time::LIMIT {
                break;
            }

            cnt += 1;

            let mut output = Output::new();

            // A:近傍探索
            let mut state: State = best_state.clone();
            state.change(&mut output, &mut rng);

            // B:壊して再構築
            // best_outputの一部を破壊して、それまでのoutputを使ってstateを作り直して再構築したり
            // outputの変形
            // best_output.remove(&mut output, &mut rng);
            // let mut state: State = initial_state.clone();
            // stateを新outputの情報で復元
            // そこから続きやる

            // スコア計算
            state.compute_score();

            // 状態更新
            solver::mountain(&mut best_state, &state, &mut best_output, &output);
            //solver::simulated_annealing(&mut best_state, &state, &mut best_output, &output, self.current_time, &mut rng);
        }

        best_output.submit();

        eprintln!("{} ", cnt);
        eprintln!("{} ", best_state.score);
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    n: usize,
}

impl Input {
    fn read() -> Self {
        let n = read_u();

        Input { n }
    }
}

#[derive(Debug, Clone)]
pub struct Output {
    //score: usize,
}

impl Output {
    fn new() -> Self {
        Output {}
    }

    fn remove(&self, output: &mut Self, rng: &mut Mcg128Xsl64) {
        // https://atcoder.jp/contests/ahc014/submissions/35567589 L558
    }

    fn submit(&self) {
        //println!("{}", );
    }
}

mod solver {
    use super::*;

    pub fn mountain(
        best_state: &mut State,
        state: &State,
        best_output: &mut Output,
        output: &Output,
    ) {
        //! bese_state(self)を更新する。

        // 最小化の場合は > , 最大化の場合は < 。
        if best_state.score > state.score {
            *best_state = state.clone();
            *best_output = output.clone();
        }
    }

    const T0: f64 = 2e3;
    //const T1: f64 = 6e2; // 終端温度が高いと最後まで悪いスコアを許容する
    const T1: f64 = 6e1; // 終端温度が高いと最後まで悪いスコアを許容する
    pub fn simulated_annealing(
        best_state: &mut State,
        state: &State,
        best_output: &mut Output,
        output: &Output,
        current_time: f64,
        rng: &mut Mcg128Xsl64,
    ) {
        //! 焼きなまし法
        //! https://scrapbox.io/minyorupgc/%E7%84%BC%E3%81%8D%E3%81%AA%E3%81%BE%E3%81%97%E6%B3%95

        static mut T: f64 = T0;
        static mut CNT: usize = 0;
        let temperature = unsafe {
            CNT += 1;
            if CNT % 100 == 0 {
                let t = current_time / my_lib::time::LIMIT;
                T = T0.powf(1.0 - t) * T1.powf(t);
            }
            T
        };

        // 最大化の場合
        let delta = (best_state.score as f64) - (state.score as f64);
        // 最小化の場合
        //let delta = (state.score as f64) - (best_state.score as f64);

        let prob = f64::exp(-delta / temperature).min(1.0);

        if delta < 0.0 {
            *best_state = state.clone();
            *best_output = output.clone();
        } else if rng.gen_bool(prob) {
            *best_state = state.clone();
            *best_output = output.clone();
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

        pub const LIMIT: f64 = 0.3;
    }

    pub trait Mat<S, T> {
        fn set(&mut self, p: S, value: T);
        fn get(&self, p: S) -> T;
        fn swap(&mut self, p1: S, p2: S);
    }

    impl<T> Mat<&Point, T> for Vec<Vec<T>>
    where
        T: Copy,
    {
        fn set(&mut self, p: &Point, value: T) {
            self[p.y][p.x] = value;
        }

        fn get(&self, p: &Point) -> T {
            self[p.y][p.x]
        }

        fn swap(&mut self, p1: &Point, p2: &Point) {
            let tmp = self[p1.y][p1.x];
            self[p1.y][p1.x] = self[p2.y][p2.x];
            self[p2.y][p2.x] = tmp;
        }
    }

    impl<T> Mat<Point, T> for Vec<Vec<T>>
    where
        T: Copy,
    {
        fn set(&mut self, p: Point, value: T) {
            self[p.y][p.x] = value;
        }

        fn get(&self, p: Point) -> T {
            self[p.y][p.x]
        }

        fn swap(&mut self, p1: Point, p2: Point) {
            let tmp = self[p1.y][p1.x];
            self[p1.y][p1.x] = self[p2.y][p2.x];
            self[p2.y][p2.x] = tmp;
        }
    }

    impl Add for Point {
        type Output = Result<Point, &'static str>;
        fn add(self, rhs: Self) -> Self::Output {
            let (x, y) = if cfg!(debug_assertions) {
                // debugではオーバーフローでpanic発生するため、オーバーフローの溢れを明確に無視する(※1.60場合。それ以外は不明)
                (self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y))
            } else {
                (self.x + rhs.x, self.y + rhs.y)
            };

            unsafe {
                if let Some(width) = WIDTH {
                    if x >= width || y >= width {
                        return Err("out of range");
                    }
                }
            }

            Ok(Point { x, y })
        }
    }

    static mut WIDTH: Option<usize> = None;

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub struct Point {
        pub x: usize, // →
        pub y: usize, // ↑
    }

    impl Point {
        pub fn new(x: usize, y: usize) -> Self {
            Point { x, y }
        }

        pub fn set_width(width: usize) {
            unsafe {
                WIDTH = Some(width);
            }
        }
    }
}

mod procon_input {
    //! This input module is written with reference to MoSoon.
    //! (https://atcoder.jp/users/MoSooN)
    //! Very, Very thank to MoSoon!
    use std::io::*;

    fn read<T: std::str::FromStr>() -> T {
        let mut s = String::new();
        stdin().read_line(&mut s).ok();
        s.trim().parse().ok().unwrap()
    }

    pub fn read_vec<T: std::str::FromStr>() -> Vec<T> {
        read::<String>()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect()
    }

    // pub fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    //     (0..n).map(|_| read_vec()).collect()
    // }

    pub fn read_i() -> (i64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        iter.next().unwrap().parse::<i64>().unwrap()
    }

    pub fn read_ii() -> (i64, i64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
        )
    }

    pub fn read_iii() -> (i64, i64, i64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
            iter.next().unwrap().parse::<i64>().unwrap(),
        )
    }

    pub fn read_u() -> (usize) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        iter.next().unwrap().parse::<usize>().unwrap()
    }

    pub fn read_uu() -> (usize, usize) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    }

    pub fn read_uuu() -> (usize, usize, usize) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    }

    pub fn read_u_vec<T: std::str::FromStr>() -> Vec<usize> {
        read::<String>()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect()
    }

    pub fn read_f() -> (f64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        iter.next().unwrap().parse::<f64>().unwrap()
    }

    pub fn read_ff() -> (f64, f64) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<f64>().unwrap(),
            iter.next().unwrap().parse::<f64>().unwrap(),
        )
    }

    pub fn read_c() -> (char) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        iter.next().unwrap().parse::<char>().unwrap()
    }

    pub fn read_cc() -> (char, char) {
        let mut str = String::new();
        let _ = stdin().read_line(&mut str).unwrap();
        let mut iter = str.split_whitespace();
        (
            iter.next().unwrap().parse::<char>().unwrap(),
            iter.next().unwrap().parse::<char>().unwrap(),
        )
    }

    pub fn read_chars() -> Vec<char> {
        let mut vec = Vec::new();
        read::<String>()
            .as_bytes()
            .iter()
            .for_each(|&b| vec.push(b as char));
        vec
    }

    // pub fn read_string() -> String {
    //     read::<String>()
    // }
}
