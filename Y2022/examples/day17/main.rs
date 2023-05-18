use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rock {
    // height, weight
    shape: (usize, usize),
    // 相对于左下角的偏移量
    points: Vec<(usize, usize)>,
}

impl Rock {
    fn rocks() -> Vec<Self> {
        vec![
            Self::zero(),
            Self::one(),
            Self::two(),
            Self::three(),
            Self::four(),
        ]
    }

    // ####
    fn zero() -> Self {
        Self {
            shape: (1, 4),
            points: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        }
    }

    // .#.
    // ###
    // .#.
    fn one() -> Self {
        Self {
            shape: (3, 3),
            points: vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        }
    }

    // ..#
    // ..#
    // ###
    fn two() -> Self {
        Self {
            shape: (3, 3),
            points: vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        }
    }

    // #
    // #
    // #
    // #
    fn three() -> Self {
        Self {
            shape: (4, 1),
            points: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        }
    }

    // ##
    // ##
    fn four() -> Self {
        Self {
            shape: (2, 2),
            points: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }
}

struct Tower {
    rocks: Vec<Rock>,
    d: Vec<Vec<bool>>,
    highest: usize,
    gas: Vec<u8>,
}

impl Tower {
    fn new(gas: Vec<u8>) -> Self {
        Self {
            rocks: Rock::rocks(),
            d: vec![vec![false; 7]; 10_0000],
            // d: vec![vec![false; 7]; 30],
            highest: 0,
            gas,
        }
    }

    fn put(&mut self, rock: &Rock, r: usize, c: usize) {
        for (dr, dc) in &rock.points {
            self.d[r + dr][c + dc] = true;
            self.highest = self.highest.max(r + dr + 1);
        }
    }

    fn gen(&self, cnt: usize) -> (Rock, (usize, usize)) {
        let rock = self.rocks[cnt % self.rocks.len()].clone();
        let r = self.highest + 3;
        let c = 2;
        (rock, (r, c))
    }

    fn can_move_left(&self, rock: &Rock, r: usize, c: usize) -> bool {
        if c < 1 {
            return false;
        }
        for (dr, dc) in &rock.points {
            if self.d[r + dr][c + dc - 1] {
                return false;
            }
        }
        true
    }

    fn can_move_right(&self, rock: &Rock, r: usize, c: usize) -> bool {
        if c + rock.shape.1 >= 7 {
            return false;
        }
        for (dr, dc) in &rock.points {
            if self.d[r + dr][c + dc + 1] {
                return false;
            }
        }
        true
    }

    fn can_move_down(&self, rock: &Rock, r: usize, c: usize) -> bool {
        if r < 1 {
            return false;
        }
        for (dr, dc) in &rock.points {
            if self.d[r + dr - 1][c + dc] {
                return false;
            }
        }
        true
    }

    fn run(&mut self) {
        let mut jet = 0;
        for i in 0..2022 {
            // dbg!(self.highest);
            let (rock, (mut r, mut c)) = self.gen(i);
            // self.print_with_rock(&rock, r, c);
            // println!("gen=================");
            loop {
                if self.gas[jet % self.gas.len()] == b'<' && self.can_move_left(&rock, r, c) {
                    c -= 1;
                } else if self.gas[jet % self.gas.len()] == b'>' && self.can_move_right(&rock, r, c)
                {
                    c += 1;
                }
                jet += 1;
                if self.can_move_down(&rock, r, c) {
                    r -= 1;
                } else {
                    break;
                }
                // self.print_with_rock(&rock, r, c);
                // println!("move================");
            }
            self.put(&rock, r, c);
            // self.print();
            // println!("done================");
        }
    }

    fn print_with_rock(&self, rock: &Rock, r: usize, c: usize) {
        let mut d = self.d.clone();
        for (dr, dc) in &rock.points {
            d[r + dr][c + dc] = true;
        }
        print_graph(&d);
    }

    fn print(&self) {
        print_graph(&self.d);
    }
}

fn print_graph(d: &[Vec<bool>]) {
    for row in d.iter().rev() {
        for col in row.iter() {
            print!("{}", if *col { '#' } else { '.' });
        }
        println!();
    }
}

fn main() -> Result<()> {
    // let f = include_str!("./example.txt");
    let f = include_str!("./input.txt");

    let mut tower = Tower::new(f.as_bytes().to_vec());
    tower.run();
    // tower.print();
    println!("higest: {}", tower.highest);

    Ok(())
}
