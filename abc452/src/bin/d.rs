use std::io::{self, Read};

fn solve(input: &str) -> String {
    let mut it = input.split_whitespace();
    let a = it.next().unwrap().to_string();
    let b = it.next().unwrap().to_string();
    let alen = a.len();
    let blen = b.len();
    let mut i = 0;
    let mut sum: usize = 0;
    for (p, _) in a.match_indices(&b) {
        sum += (p + blen - i) * (p + blen - i - 1) / 2;
        i = p + 1;
    }
    sum += (alen - i) * (alen - i - 1) / 2;
    sum.to_string()
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
abrakadabra
aba
";
        let expected = "\
51";
        assert_eq!(solve(input), expected);
    }

    #[test]
    fn sample2() {
        let input = "\
aaaaa
a
";
        let expected = "\
0";
        assert_eq!(solve(input), expected);
    }

    #[test]
    fn sample3() {
        let input = "\
rdddrdtdcdrrdcredctdordoeecrotet
dcre
";
        let expected = "\
263";
        assert_eq!(solve(input), expected);
    }

    #[test]
    fn sample4() {
        let input = "\
a
aa
";
        let expected = "\
1";
        assert_eq!(solve(input), expected);
    }
}

