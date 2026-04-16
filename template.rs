use std::io::{self, Read};

fn solve(input: &str) -> String {
    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    (a + b).to_string()
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
2 3
";
        let expected = "\
5";
        assert_eq!(solve(input), expected);
    }
}
