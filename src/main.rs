fn read_lines<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    let mut v2 = Vec::new();
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let v = s.trim().split_whitespace()
            .map(|e| e.parse().ok().unwrap()).collect();
        v2.push(v);
    }
    v2
}

fn main() {
    let vecs = read_lines::<i32>(30);

    for vec in vecs {
        for x in vec {
            print!("{:>3} ", x);
        }
        println!("")
    }
}
