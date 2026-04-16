use std::io::{self, Read};

fn solve(input: &str) -> String {
    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    match (a, b) {
        (1, 7) | (3, 3) | (5, 5) | (7, 7) | (9, 9) => "Yes".to_string(),
        _ => "No".to_string(),
    }
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
3 3
";
        assert_eq!(solve(input), "Yes");
    }

    #[test]
    fn sample2() {
        let input = "\
1 1
";
        assert_eq!(solve(input), "No");
    }

    #[test]
    fn sample3() {
        let input = "\
4 4
";
        assert_eq!(solve(input), "No");
    }

    #[test]
    fn sample4() {
        let input = "\
11 7
";
        assert_eq!(solve(input), "No");
    }

    #[test]
    fn sample5() {
        let input = "\
1 7
";
        assert_eq!(solve(input), "Yes");
    }

    #[test]
    fn sample6() {
        let input = "\
5 5
";
        assert_eq!(solve(input), "Yes");
    }

    #[test]
    fn sample7() {
        let input = "\
7 7
";
        assert_eq!(solve(input), "Yes");
    }

    #[test]
    fn sample8() {
        let input = "\
9 9
";
        assert_eq!(solve(input), "Yes");
    }
}
