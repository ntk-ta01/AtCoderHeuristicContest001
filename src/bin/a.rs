use proconio::input;
use std::fmt;

const W: i32 = 10000;
fn main() {
    input! {
        n: usize,
        xys: [(i32, i32, i32); n],
    }
    let ps = xys.iter().map(|&(x, y, _)| (x, y)).collect::<Vec<_>>();
    let size = xys.iter().map(|&(_, _, s)| s).collect::<Vec<_>>();
    let input = Input { n, ps, size };
    let mut out = vec![
        Rect {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
        };
        n
    ];
    for (i, (x, y)) in input.ps.iter().enumerate() {
        out[i] = Rect {
            x1: *x,
            y1: *y,
            x2: x + 1,
            y2: y + 1,
        };
    }
    // 答えを出力
    for a in out.iter() {
        println!("{}", a);
    }
    eprintln!("{}", compute_score(&input, &out));
}

struct Input {
    n: usize,
    ps: Vec<(i32, i32)>,
    size: Vec<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.x1, self.y1, self.x2, self.y2)
    }
}

impl Rect {
    fn size(&self) -> i32 {
        (self.x2 - self.x1) * (self.y2 - self.y1)
    }
}

fn intersect(r1: &Rect, r2: &Rect) -> bool {
    r1.x2.min(r2.x2) > r1.x1.max(r2.x1) && r1.y2.min(r2.y2) > r1.y1.max(r2.y1)
}

fn compute_score(input: &Input, out: &Vec<Rect>) -> i64 {
    let mut score = 0.0;
    for i in 0..input.n {
        if out[i].x1 < 0 || out[i].x2 > W || out[i].y1 < 0 || out[i].y2 > W {
            eprintln!("rectangle {} is out of range", i);
            return 0;
        }
        if out[i].x1 >= out[i].x2 || out[i].y1 >= out[i].y2 {
            eprintln!("rectangle {} does not have positive area", i);
            return 0;
        }
        if !(out[i].x1 <= input.ps[i].0
            && input.ps[i].0 < out[i].x2
            && out[i].y1 <= input.ps[i].1
            && input.ps[i].1 < out[i].y2)
        {
            eprintln!("rectangle {} does not contain point {}", i, i);
            continue;
        }
        for j in 0..i {
            if intersect(&out[i], &out[j]) {
                eprintln!("rectangles {} and {} overlap", j, i);
                return 0;
            }
        }
        let s = out[i].size().min(input.size[i]) as f64 / out[i].size().max(input.size[i]) as f64;
        score += 1.0 - (1.0 - s) * (1.0 - s);
    }
    (1e9 * score / input.n as f64).round() as i64
}
