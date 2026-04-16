use std::io::{self, Read};

fn solve(input: &str) -> String {
    let mut it = input.split_whitespace();
    let h: i32 = it.next().unwrap().parse().unwrap();
    let w: i32 = it.next().unwrap().parse().unwrap();
    let mut result = Vec::new();
    for i in 0..h {
        let mut row = String::new();
        for j in 0..w {
            if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
                row.push('#');
            } else {
                row.push('.');
            }
        }
        result.push(row);
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
4 5
";
        let expected = "\
#####
#...#
#...#
#####";
        assert_eq!(solve(input), expected);
    }

    #[test]
    fn sample2() {
        let input = "\
5 6
";
        let expected = "\
######
#....#
#....#
#....#
######";
        assert_eq!(solve(input), expected);
    }
}
