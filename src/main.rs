use std::io::{BufRead, BufReader};
use std::fs::File;

const DEBUG_MODE: u32 = 1;

fn read_lines<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    let mut v2 = Vec::new();
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let v = s.trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        v2.push(v);
    }
    v2
}

/*
オブジェクト指向みたいにプログラミングできない？
関数型っぽく考える必要がある
 */
struct Solver {
    w: i32,
    h: i32
}

impl Solver {
    fn hello(&self) {
        println!("Hello",);
    }

    fn input(&self) {

    }

    fn input_debug(&self) {
        let mut cells = Vec::new();
        let f = File::open("src/test1.txt").unwrap();
        let fr = BufReader::new(f);

        for line in fr.lines() {
            // println!("{}", line.unwrap());
            // let s: String = line.unwrap();
            let tmp : Vec<i32> = line.unwrap().trim()
                .split_whitespace()
                .map(|e| e.parse().ok().unwrap())
                .collect();
            cells.push(tmp);
        }
    }

    fn show_w(&self) {
        println!("{}", self.w);
    }
}

fn main() {
    if DEBUG_MODE == 0 {
        let vecs = read_lines::<i32>(30);
        for vec in vecs {
            for x in vec {
                print!("{:>3} ", x);
            }
            println!("")
        }
    } else {
        let mut vecs = Vec::new();
        let f = File::open("src/test1.txt").unwrap();
        let fr = BufReader::new(f);

        for line in fr.lines() {
            // println!("{}", line.unwrap());
            // let s: String = line.unwrap();
            let tmp : Vec<i32> = line.unwrap().trim()
                .split_whitespace()
                .map(|e| e.parse().ok().unwrap())
                .collect();
            vecs.push(tmp);
        }

        for vec in vecs {
            for x in vec {
                print!("{:>3} ", x);
            }
            println!("")
        }
    }

    // let mut vec = Vec::new();
    let solver: Solver = Solver { w: 30, h: 30 };
    solver.input_debug();
    solver.hello();
    solver.show_w();
}
