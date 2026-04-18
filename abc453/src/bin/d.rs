use std::{collections::VecDeque, io::{self, Read}};

fn find_s(board: Vec<Vec<u8>>) -> (usize, usize) {
    board
        .iter()
        .enumerate()
        .find_map(|(y, v)| {
            v.iter()
                .position(|c| *c == b'S')
                .map(|x| (x, y))
        })
        .unwrap()
}

fn read_board(board: Vec<Vec<u8>>, x: usize, y: usize) -> u8 {
    board[y][x]
}

enum Step {
    Up,
    Down,
    Right,
    Left,
}

fn solve(input: &str) -> String {
    let mut it = input.split_whitespace();
    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();
    let board: Vec<Vec<u8>> = (0..h)
        .map(|_| it.next().unwrap().bytes().collect())
        .collect();
    let mut last_s = Step::Up;
    let mut p = find_s(board);
    let mut q = VecDeque::new();
    q.push_back((p, last_s));
    "No".to_string()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    print!("{}", solve(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_valid_point(h: usize, w: usize, x: i32, y: i32) {
        assert!(x >= 0);
        assert!(y >= 0);
        assert!((x as usize) < w);
        assert!((y as usize) < h);
    }

    fn find_s(vec: &Vec<&str>) -> Option<(i32, i32)> {
        vec.iter().enumerate().find_map(|(i, s)| {
            s.chars()
                .position(|c| c == 'S')
                .map(|j| (j as i32, i as i32))
        })
    }

    fn read_board(board: &Vec<&str>, x: usize, y: usize) -> Option<char> {
        board[y].chars().nth(x)
    }

    fn assert_valid_route(input: &str, output: &str) {
        let mut it = input.split_whitespace();
        let h: usize = it.next().unwrap().parse().unwrap();
        let w: usize = it.next().unwrap().parse().unwrap();
        let board: Vec<&str> = (0..h).map(|_| it.next().unwrap()).collect();

        let mut it = output.split_whitespace();
        assert_eq!(it.next().unwrap(), "Yes");
        let mut p = find_s(&board).unwrap();
        assert_valid_point(h, w, p.0, p.1);
        let mut last_c = 'U';
        for c in it.next().unwrap().chars() {
            let b = read_board(&board, p.0 as usize, p.1 as usize).unwrap();
            let step = match c {
                'U' => (0, -1),
                'D' => (0, 1),
                'R' => (1, 0),
                'L' => (-1, 0),
                _ => panic!("Invalid move {}", c),
            };
            match b {
                '#' => panic!("invalid position ({}, {}) = {}", p.0, p.1, b),
                '.' => {}
                'o' => assert_eq!(c, last_c),
                'x' => assert_ne!(c, last_c),
                'S' => {}
                'G' => {}
                _ => panic!("invalid board input {}", b),
            }
            p = (p.0 + step.0, p.1 + step.1);
            assert_valid_point(h, w, p.0, p.1);
            last_c = c;
        }
        let b = read_board(&board, p.0 as usize, p.1 as usize).unwrap();
        assert_eq!(b, 'G')
    }

    fn assert_valid_answer(input: &str, output: &str, answer: &str) {
        match answer {
            "Yes" => {
                assert_eq!(output.split_whitespace().next().unwrap(), answer);
                assert_valid_route(input, output);
            }
            "No" => {
                assert_eq!(output.split_whitespace().next().unwrap(), answer);
            }
            _ => {
                panic!("invalid answer str {}", answer);
            }
        }
    }

    #[test]
    fn sample1() {
        let input = "\
3 5
.#...
.Sooo
..x.G
";
        let output = solve(input);
        assert_valid_answer(input, &output, "Yes");
    }

    #[test]
    fn sample2() {
        let input = "\
3 3
#So
xoG
..#
";
        let output = solve(input);
        assert_valid_answer(input, &output, "Yes");
    }

    #[test]
    fn sample3() {
        let input = "\
2 2
So
oG
";
        let output = solve(input);
        assert_valid_answer(input, &output, "No");
    }
}
