fn main() {
    for _ in 0..read_i32() {
        test_case();
    }
}

fn test_case() {
    let [n, m] = [read_i32() as usize, read_i32() as usize];
    let mut pos = vec![0; n as usize];
    for i in 0..n as usize {
        pos[i] = read_i32();
    }
    pos.sort();

    println!("{}", binary_search(&pos, m as i32, 1, pos[n - 1] - pos[0]));
}

fn binary_search(pos: &[i32], m: i32, l: i32, r: i32) -> i32 {
    if l == r {
        return l;
    }
    let mid = (l + r) / 2 + 1;
    if check(pos, m, mid) {
        binary_search(pos, m, mid, r)
    } else {
        binary_search(pos, m, l, mid - 1)
    }
}

fn check(pos: &[i32], m: i32, k: i32) -> bool {
    let mut m = m - 1;
    let mut pre = 0;
    for i in 1..pos.len() {
        if pos[i] - pos[pre] >= k {
            m -= 1;
            pre = i;
        }
        if m == 0 {
            return true;
        }
    }
    false
}

fn read_i32() -> i32 {
    use std::io::{self, Read, Write};
    let mut number = String::new();
    io::stdout().flush().unwrap();
    let mut stdin = io::stdin();
    let mut buffer = [0; 1];
    while let Ok(()) = stdin.read_exact(buffer.as_mut()) {
        let c = buffer[0] as char;
        if number.is_empty() && c == '-' {
            number.push(c);
            continue;
        }
        if c.is_numeric() {
            number.push(c);
        } else {
            break;
        }
    }
    number.parse::<i32>().unwrap_or(0)
}
