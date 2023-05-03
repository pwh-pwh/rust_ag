use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut r = vec![];
    for (index, n) in nums.into_iter().enumerate().map(|(i, n)| (i as i32, n)) {
        if !map.is_empty() && map.contains_key(&(target - n)) {
            r.push(index);
            r.push(*map.get(&(target - n)).unwrap());
            break;
        }
        map.insert(n, index);
    }
    r
}

fn main() {
    let r = two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", r);
}
