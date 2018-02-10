use std::io::{BufRead, BufReader};
use std::fs::File;

// const DEBUG_MODE: u32 = 1;

/// 定数
const W: usize = 30;
const H: usize = 30;
const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

/// Solver
fn solve() -> bool {
    let mut cells = read_lines();
    // let mut cells = read_from_file();

    let mut p = search_top(&cells);
    while p.0 >= 0 {
        while true {
            let x = p.0;
            let y = p.1;
            cells[y as usize][x as usize] = cells[y as usize][x as usize] - 1;
            println!("{} {}", y + 1, x + 1);

            let mut next = (-1, -1);
            for d in DIRECTIONS.iter() {
                let nx = x + d.0;
                let ny = y + d.1;
                if !check_border(nx, ny) {
                    continue;
                }
                if cells[y as usize][x as usize] == cells[ny as usize][nx as usize] {
                    if cells[ny as usize][nx as usize] > 0 {
                        next = (nx, ny);
                        break;
                    }
                }
            }

            if next.0 >= 0 {
                p = next;
                continue;
            }

            break;
        }
        p = search_top(&cells);
    }

    // for row in cells {
    //     for x in row {
    //         print!("{:>3} ", x);
    //     }
    //     println!("")
    // }
    return true;
}

fn compare_around(cells: &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    for d in DIRECTIONS.iter() {
        let dx = x as i32 + d.0;
        let dy = y as i32 + d.1;
        if !check_border(dx, dy) {
            continue;
        }
        if cells[y][x] < cells[dy as usize][dx as usize] {
            return false;
        }
    }
    return true;
}

fn search_top(cells: &Vec<Vec<i32>>) -> (i32, i32) {
    for y in 0..H {
        for x in 0..W {
            if cells[y][x] <= 0 {
                continue;
            }
            if compare_around(&cells, x, y) {
                return (x as i32, y as i32);
            }
        }
    }
    return (-1, -1);
}

fn check_border(x: i32, y: i32) -> bool {
    if x < 0 || x >= W as i32 || y < 0 || y >= H as i32 {
        return false;
    }
    return true;
}

/// 標準入力を読む
fn read_lines() -> Vec<Vec<i32>> {
    let mut v2 = Vec::new();
    for _ in 0..H {
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

/// デバッグ用
fn read_from_file() -> Vec<Vec<i32>> {
    let mut cells = Vec::new();
    let f = File::open("src/test1.txt").unwrap();
    let fr = BufReader::new(f);

    for line in fr.lines() {
        // println!("{}", line.unwrap());
        // let s: String = line.unwrap();
        let tmp: Vec<i32> = line.unwrap()
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        cells.push(tmp);
    }
    cells
}

fn main() {
    solve();
}
