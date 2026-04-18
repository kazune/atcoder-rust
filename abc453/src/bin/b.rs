use std::io::{self, Read};

fn solve(input: &str) -> String {
    let mut it = input.split_whitespace();
    let t: usize = it.next().unwrap().parse().unwrap();
    let x: i32 = it.next().unwrap().parse().unwrap();
    let ss: Vec<i32> = (0..(t + 1))
        .map(|_| it.next().unwrap().parse().unwrap())
        .collect();
    let mut latest = ss[0];
    let mut result = Vec::new();
    result.push(format!("{} {}", 0, ss[0]));
    for i in 1..t + 1 {
        let now = ss[i];
        let diff = (latest - now).abs();
        if diff >= x {
            latest = now;
            result.push(format!("{} {}", i, now));
        }
    }
    result.join("\n")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    print!("{}", solve(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "\
6 10
30 35 40 21 30 12 31
";
        assert_eq!(
            solve(input),
            "0 30
2 40
3 21
6 31"
        );
    }
}
