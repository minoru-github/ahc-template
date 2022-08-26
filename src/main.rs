#![allow(unused)]
use itertools::Itertools;
use num::{integer::Roots, Integer, ToPrimitive};
use proconio::input;
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
    let input = Input::read();

    let best_state = solver::mountain(&input);

    best_state.output();
}

mod solver {
    use super::*;

    pub fn mountain(input: &Input) -> State {
        let mut rng: Mcg128Xsl64 = rand_pcg::Pcg64Mcg::new(890482);

        let executable_steps = input.kinds * 100;

        let mut state = State::new(&input);
        while time::update() < 2.94 {
            let mut sim = Sim::new(input);
            state.executable_steps = executable_steps;

            let movable_steps = executable_steps / 3;
            for _ in 0..movable_steps {
                let old_state = state.clone();
                sim.move_within_steps(&mut rng, &mut state);
                sim.connect_same_row_or_col(1, &mut state);

                let score = sim.compute_score(&state);
                if score > state.best_score {
                    state.best_score = score;
                } else {
                    state = old_state;
                }
            }
        }

        state
    }
}

mod time {
    pub(super) fn update() -> f64 {
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

type Map = Vec<Vec<usize>>;

#[derive(Debug, Clone)]
pub struct Input {
    n: usize,
    kinds: usize,
    map: Map,
    pos_vec_at_cid: Vec<Vec<XY>>,
}

impl Input {
    fn read() -> Self {
        Self::read_console()
    }

    fn read_console() -> Input {
        // a : 型
        // (a,b) : (型, 型)
        // a_vec : [型;サイズ]
        // a_vec2 : [[型;サイズ];サイズ]
        // S : [chars; n] or Chars
        input! {
            (n, kinds) : (usize, usize),
            map : [String; n],
        };

        let map = Self::split_ahc013input(&map);

        let mut pos_vec_at_cid = vec![Vec::new(); kinds + 1];
        Self::create_pos_vec_at_cid(n, &map, &mut pos_vec_at_cid);

        return Input {
            n,
            kinds,
            map,
            pos_vec_at_cid,
        };
    }

    fn debug(result: &Result<Input, &str>) {
        println!("{:?}", result);
    }

    // for ahc013
    fn split_ahc013input(cid_map: &Vec<String>) -> Map {
        let mut splitted = Vec::new();
        for row in cid_map {
            let mut kind_vec = Vec::new();
            row.chars().collect::<Vec<char>>().iter().for_each(|kind| {
                let kind = (*kind).to_digit(10).unwrap() as usize;
                kind_vec.push(kind);
            });
            splitted.push(kind_vec);
        }
        splitted
    }

    fn create_pos_vec_at_cid(n: usize, map: &Map, pos_vec_at_cid: &mut Vec<Vec<XY>>) {
        for y in 0..n {
            for x in 0..n {
                let cid = map[y][x];
                let pos = XY::new(x, y, n);
                pos_vec_at_cid[cid].push(pos);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    map_width: usize,
    map: Map,
    pos_vec_at_cid: Vec<Vec<XY>>,
    move_vec: Vec<(usize, usize)>,
    connection_vec: Vec<(usize, usize)>,
    best_score: usize,
    executable_steps: usize,
}

impl State {
    fn new(input: &Input) -> Self {
        State {
            map_width: input.n,
            map: input.clone().map,
            pos_vec_at_cid: input.clone().pos_vec_at_cid,
            move_vec: Vec::new(),
            connection_vec: Vec::new(),
            best_score: 0,
            executable_steps: 0,
        }
    }

    fn output(&self) {
        println!("{}", self.move_vec.len());
        for (from, to) in self.move_vec.clone() {
            let from = XY::to_2d(from, self.map_width);
            let to = XY::to_2d(to, self.map_width);
            println!("{} {} {} {}", from.y, from.x, to.y, to.x);
        }
        println!("{}", self.connection_vec.len());
        for (from, to) in self.connection_vec.clone() {
            let from = XY::to_2d(from, self.map_width);
            let to = XY::to_2d(to, self.map_width);
            println!("{} {} {} {}", from.y, from.x, to.y, to.x);
        }

        eprintln!("{}", self.best_score);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct XY {
    y: usize, // ↓
    x: usize, // →
    width: usize,
}

impl XY {
    fn new(x: usize, y: usize, width: usize) -> Self {
        XY { x, y, width }
    }

    fn to_1d(&self) -> usize {
        self.y * self.width + self.x
    }

    fn to_2d(index: usize, width: usize) -> Self {
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

#[cfg(test)]
mod xy_tests {
    use super::*;

    #[test]
    fn test_add() {
        let pos = XY::new(4, 13, 15);
        let dir = direction::LEFT;
        let expect = XY::new(3, 13, 15);
        assert_eq!((pos + dir).ok(), Some(expect));
    }
}

mod direction {
    use super::*;
    pub const LEFT: XY = XY {
        x: !0,
        y: 0,
        width: 0,
    };
    pub const RIGHT: XY = XY {
        x: 1,
        y: 0,
        width: 0,
    };
    pub const UP: XY = XY {
        x: 0,
        y: !0,
        width: 0,
    };
    pub const DOWN: XY = XY {
        x: 0,
        y: 1,
        width: 0,
    };
    pub const LRUP: [XY; 4] = [LEFT, RIGHT, UP, DOWN];
}

#[derive(Debug, Clone)]
pub struct Sim {
    n: usize,
    kinds: usize,
}

impl Sim {
    fn new(input: &Input) -> Self {
        Sim {
            n: input.n,
            kinds: input.kinds,
        }
    }

    fn connect_same_row_or_col(&mut self, cid: usize, state: &mut State) {
        let mut has_seen = vec![vec![false; self.n]; self.n];
        state.connection_vec.clear();

        let width = self.n;
        for y in 0..width {
            for x in 0..width {
                if has_seen[y][x] || state.map[y][x] != cid {
                    continue;
                }
                self.connect_bfs(XY { x, y, width }, cid, &mut has_seen, state);
            }
        }
    }

    fn connect_bfs(
        &mut self,
        pos: XY,
        cid: usize,
        has_seen: &mut Vec<Vec<bool>>,
        state: &mut State,
    ) {
        let mut q = VecDeque::new();

        has_seen[pos.y][pos.x] = true;
        q.push_back(pos);
        while !q.is_empty() {
            let now_pos = q.pop_front().unwrap();
            for dir in direction::LRUP.iter() {
                let some_pos = self.search_in_same_row_or_col(cid, &now_pos, dir, has_seen, state);
                if let Some(connected_pos) = some_pos {
                    q.push_back(connected_pos.clone());
                    state
                        .connection_vec
                        .push((now_pos.to_1d(), connected_pos.to_1d()));
                }
            }
        }
    }

    fn search_in_same_row_or_col(
        &self,
        cid: usize,
        start_pos: &XY,
        dir: &XY,
        has_seen: &mut Vec<Vec<bool>>,
        state: &mut State,
    ) -> Option<XY> {
        let mut now_pos = start_pos.clone();
        loop {
            if let Ok(next_pos) = now_pos + dir.clone() {
                if has_seen[next_pos.y][next_pos.x] {
                    return None;
                }

                let next_cid = state.map[next_pos.y][next_pos.x];

                if next_cid == cid {
                    let mut cable_pos = (start_pos.clone() + dir.clone()).ok().unwrap();
                    while cable_pos != next_pos {
                        has_seen[cable_pos.y][cable_pos.x] = true;
                        cable_pos = (cable_pos + dir.clone()).ok().unwrap();
                    }
                    has_seen[next_pos.y][next_pos.x] = true;
                    return Some(next_pos);
                } else if next_cid == 0 {
                    // computerがない
                    // do notnihg
                } else {
                    // 別のcid
                    return None;
                }

                now_pos = next_pos.clone();
            } else {
                return None;
            }
        }
    }

    fn move_within_steps(&mut self, rng: &mut Mcg128Xsl64, state: &mut State) {
        for cnt in 0..1_usize {
            let cid = rng.gen_range(0, self.kinds) + 1;
            let node_index = rng.gen_range(0, 100) as usize;
            let start_pos = state.pos_vec_at_cid[cid][node_index].clone();

            let mut pos_within_steps_vec = Vec::new();
            // key = 座標1次元表記, value = XY
            let mut from_map = BTreeMap::new();

            Self::move_bfs(
                &self,
                &start_pos,
                &mut pos_within_steps_vec,
                &mut from_map,
                state,
            );

            if pos_within_steps_vec.is_empty() {
                continue;
            }

            let index = rng.gen_range(0, pos_within_steps_vec.len());
            let goal_pos = pos_within_steps_vec[index].clone();

            let mut path = VecDeque::new();
            Self::fukugen_move_path(start_pos.to_1d(), goal_pos.to_1d(), &from_map, &mut path);

            for (from, to) in path {
                state.move_vec.push((from, to));
            }

            state.map[goal_pos.y][goal_pos.x] = cid;
            state.map[start_pos.y][start_pos.x] = 0;
            state.pos_vec_at_cid[cid][node_index] = goal_pos;
        }
    }

    fn move_bfs(
        &self,
        start_pos: &XY,
        pos_within_steps_vec: &mut Vec<XY>,
        from_map: &mut BTreeMap<usize, usize>,
        state: &mut State,
    ) {
        let mut q = VecDeque::new();
        let mut now_pos = start_pos.clone();

        // TODO n*nもいらないので削減する
        let mut has_seen = vec![vec![false; self.n]; self.n];
        let mut dist = vec![vec![0_usize; self.n]; self.n];

        q.push_back(now_pos.clone());

        while !q.is_empty() {
            now_pos = q.pop_front().unwrap();
            has_seen[now_pos.y][now_pos.x] = true;

            for dir in direction::LRUP.iter() {
                if let Ok(next_pos) = now_pos.clone() + dir.clone() {
                    if state.map[next_pos.y][next_pos.x] != 0 {
                        continue;
                    }
                    if has_seen[next_pos.y][next_pos.x] {
                        continue;
                    }
                    has_seen[next_pos.y][next_pos.x] = true;
                    dist[next_pos.y][next_pos.x] = dist[now_pos.y][now_pos.x] + 1;
                    if dist[next_pos.y][next_pos.x] > 3 {
                        continue;
                    }

                    q.push_back(next_pos.clone());
                    pos_within_steps_vec.push(next_pos.clone());

                    from_map.insert(next_pos.to_1d(), now_pos.to_1d());
                }
            }
        }
    }

    fn fukugen_move_path(
        start: usize,
        goal: usize,
        from_map: &BTreeMap<usize, usize>,
        path: &mut VecDeque<(usize, usize)>,
    ) {
        let mut prev = from_map.get(&goal).unwrap();

        path.push_front((*prev, goal));

        while *prev != start {
            let now = prev.clone();
            prev = from_map.get(&now).unwrap();
            path.push_front((*prev, now));
        }
    }

    fn compute_score(&self, state: &State) -> usize {
        let mut uf = my_lib::Dsu::new(self.n * self.n);

        let mut connectable_cnt = state.executable_steps - state.move_vec.len();
        for (from, to) in state.connection_vec.clone() {
            if connectable_cnt == 0 {
                break;
            }
            connectable_cnt -= 1;
            uf.merge(from, to);
        }

        let leader_vec = uf.leader_vec();
        let mut total_score = 0;
        for leader in leader_vec {
            let size = uf.group_size(leader);
            if size > 0 {
                let score = (size - 1) * size / 2;
                total_score += score;
            }
        }

        return total_score;
    }
}

mod my_lib {
    pub struct Dsu {
        parent_or_size: Vec<i64>, // 親のindex or 親のときはグループのサイズを-1した値(for 経路圧縮)
        num_node: usize,
        num_group: usize,

        // extentions
        min_index: Vec<usize>,
    }

    impl Dsu {
        pub fn new(n: usize) -> Self {
            let mut min_index = Vec::<usize>::new();
            for index in 0..n as usize {
                min_index.push(index);
            }

            Dsu {
                parent_or_size: vec![-1; n],
                num_node: n,
                num_group: n,
                min_index: min_index,
            }
        }

        pub fn leader(&mut self, index: usize) -> usize {
            //! 代表元のindex取得
            assert!(index < self.num_node);

            let parent_index = self.parent_or_size[index];
            if self.parent_or_size[index] < 0 {
                index
            } else {
                let parent_index = self.leader(parent_index as usize);
                self.parent_or_size[index] = parent_index as i64;
                parent_index
            }
        }

        pub fn leader_vec(&self) -> Vec<usize> {
            let mut leaders = Vec::new();
            for (index, size_minus) in self.parent_or_size.iter().enumerate() {
                if *size_minus < 0 {
                    leaders.push(index as usize);
                }
            }
            leaders
        }

        pub fn merge(&mut self, a: usize, b: usize) -> usize {
            assert!(a < self.num_node);
            assert!(b < self.num_node);

            let mut leader_a = self.leader(a);
            let mut leader_b = self.leader(b);

            // 既に同じグループ
            if leader_a == leader_b {
                return leader_a;
            }

            // グループのサイズが大きいほうにマージする
            // 代表元のparent_or_sizeにはグループのサイズに-1した値が格納されている
            let group_size_a = -1 * self.parent_or_size[leader_a];
            let group_size_b = -1 * self.parent_or_size[leader_b];
            // aを基準にする
            if group_size_a < group_size_b {
                std::mem::swap(&mut leader_a, &mut leader_b);
            }
            // サイズ加算
            self.parent_or_size[leader_a] += self.parent_or_size[leader_b];
            self.parent_or_size[leader_b] = leader_a as i64;

            // グループ統合により、グループ数が減る
            self.num_group -= 1;

            // グループの最小index更新
            if self.min_index[leader_a] > self.min_index[leader_b] {
                self.min_index[leader_a] = self.min_index[leader_b];
            }

            leader_a
        }

        pub fn is_same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.num_node);
            assert!(b < self.num_node);

            if self.leader(a) == self.leader(b) {
                true
            } else {
                false
            }
        }

        pub fn group_size(&mut self, leader: usize) -> usize {
            assert!(leader < self.num_node);

            (-1 * self.parent_or_size[leader]) as usize
        }

        pub fn group_num(&mut self) -> usize {
            self.num_group
        }

        pub fn min_index(&mut self, leader: usize) -> usize {
            assert!(leader < self.num_node);

            self.min_index[leader]
        }
    }

    fn chmin<T: PartialOrd>(a: &mut T, b: T) -> bool {
        if *a > b {
            *a = b;
            true
        } else {
            false
        }
    }

    fn chmax<T: PartialOrd>(a: &mut T, b: T) -> bool {
        if *a < b {
            *a = b;
            true
        } else {
            false
        }
    }
}
