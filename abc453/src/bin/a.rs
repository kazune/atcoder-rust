use std::io::{self, Read};

fn solve(input: &str) -> String {
    let mut it = input.split_whitespace();
    let _n: i32 = it.next().unwrap().parse().unwrap();
    let s = it.next().unwrap();
    s.trim_start_matches('o').to_string()
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
7
ooparts
";
        assert_eq!(solve(input), "parts");
    }

    #[test]
    fn sample2() {
        let input = "\
6
abcooo
";
        assert_eq!(solve(input), "abcooo");
    }

    #[test]
    fn sample3() {
        let input = "\
5
ooooo
";
        assert_eq!(solve(input), "");
    }
}
