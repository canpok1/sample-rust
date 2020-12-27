mod math;

fn main() {
    let stdout = solve(1);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(n: usize) -> Vec<String> {
    let mut buf = Vec::new();
    buf.push(format!("args = {}", n));
    buf
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(1), vec!("args = 1"));
}

#[test]
fn test_solve_2() {
    assert_eq!(solve(2), vec!("args = 2"));
}
