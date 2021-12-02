use crate::helper;

type Data = Vec<(String, i32)>;

pub fn input() -> Data {
    let raw = helper::get_input(2).unwrap();

    let mut data = Vec::new();
    for s in raw.split("\n") {
        let mut t = s.split(" ");

        data.push((
            t.next().unwrap().to_string(),
            t.next().unwrap().parse().unwrap(),
        ));
    }

    data
}

pub fn one(data: Data) -> i32 {
    let mut x = 0;
    let mut y = 0;
    for (dir, d) in data {
        match &dir as &str {
            "down" => y += d,
            "forward" => x += d,
            "up" => y -= d,
            _ => panic!(),
        }
    }

    x * y
}

pub fn two(data: Data) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for (dir, d) in data {
        match &dir as &str {
            "down" => aim += d,
            "up" => aim -= d,
            "forward" => {
                x += d;
                y += aim * d;
            }
            _ => panic!(),
        }
    }
    x * y
}

mod tests {
    #[test]
    fn test_one() {
        let ans = super::one(super::input());
        assert_eq!(ans, 1670340);
    }

    #[test]
    fn test_two() {
        let ans = super::two(super::input());
        assert_eq!(ans, 1954293920);
    }
}
