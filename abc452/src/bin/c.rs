use std::{
    collections::{HashSet},
    io::{self, Read},
};

fn solve(input: &str) -> String {
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let ab: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let a = it.next().unwrap().parse().unwrap();
            let b = it.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect();
    let m: usize = it.next().unwrap().parse().unwrap();
    let ss: Vec<Vec<char>> = (0..m)
        .map(|_| it.next().unwrap().chars().collect())
        .collect();
    let mut table = vec![vec![]; 11];
    for s in &ss {
        let l = s.len();
        if table[l].is_empty() {
            table[l] = vec![HashSet::new(); l];
        }
        s.iter().enumerate().for_each(|(i, c)| {
            table[l][i].insert(c);
        });
    }

    let mut result = Vec::new();
    for s in &ss {
        if s.len() != n {
            result.push("No".to_string());
            continue;
        }
        if s.iter().enumerate().all(|(i, c)| {
            let l = ab[i].0;
            let d = ab[i].1 - 1;
            table[l][d].contains(c)
        }) {
            result.push("Yes".to_string())
        } else {
            result.push("No".to_string())
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
5
5 3
5 2
4 1
5 1
3 2
8
retro
chris
itchy
tuna
crab
rock
cod
ash";
        let expected = "\
Yes
Yes
No
No
No
No
No
No";
        assert_eq!(solve(input), expected);
    }

    #[test]
    fn sample2() {
        let input = "\
5
5 1
5 2
5 3
5 4
5 5
8
retro
chris
itchy
tuna
crab
rock
cod
ash
";
        let expected = "\
Yes
Yes
Yes
No
No
No
No
No";
        assert_eq!(solve(input), expected);
    }
}
