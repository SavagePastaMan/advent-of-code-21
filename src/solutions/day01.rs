use crate::helper;

pub fn input() -> Vec<i32> {
    let data = helper::get_input(1).unwrap();

    data.split("\n").map(|x| x.parse().unwrap()).collect()
}

pub fn one(data: Vec<i32>) -> i32 {
    solve(data, 1)
}

pub fn two(data: Vec<i32>) -> i32 {
    solve(data, 3)
}

fn solve(data: Vec<i32>, n: usize) -> i32 {
    let mut c = 0;
    for w in data.windows(n + 1) {
        if w[n] > w[0] {
            c += 1;
        }
    }
    c
}
