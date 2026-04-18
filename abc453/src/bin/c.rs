use std::io::{self, Read};

fn solve(input: &str) -> String {
    let mut it = input.split_whitespace();
    let t: usize = it.next().unwrap().parse().unwrap();
    let ls: Vec<_> = (0..t)
        .map(|_| {
            let l: i64 = it.next().unwrap().parse().unwrap();
            l * 2
        })
        .collect();
    let max = (0..(1 << t))
        .map(|mask| {
            let mut x = 1;
            let mut count = 0;
            for (i, l) in ls.iter().enumerate() {
                let s = if (mask >> i) & 1 == 1 { 1 } else { -1 };
                let next = s * l + x;
                if next.signum() * x.signum() < 0 {
                    count += 1;
                }
                x = next;
            }
            count
        })
        .max()
        .unwrap();
    max.to_string()
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
5
2 5 2 2 1
";
        assert_eq!(solve(input), "4");
    }

    #[test]
    fn sample2() {
        let input = "\
5
100 1 2 3 4
";
        assert_eq!(solve(input), "1");
    }

    #[test]
    fn sample3() {
        let input = "\
20
1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
";
        assert_eq!(solve(input), "20");
    }

    #[test]
    fn sample4() {
        let input = "\
20
1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000
";
        assert_eq!(solve(input), "20");
    }
}
