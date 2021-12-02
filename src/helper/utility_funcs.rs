use num;

pub fn gcd_vec<T>(data: &Vec<T>) -> T
where
    T: num::Integer + Copy,
{
    data.iter()
        .fold(data[0], |acc, x| num::integer::gcd(acc, *x))
}

pub fn lcm_vec<T>(data: &Vec<T>) -> T
where
    T: num::Integer + Copy,
{
    data.iter()
        .fold(data[0], |acc, x| num::integer::lcm(acc, *x))
}
