pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    let mut left = 0;
    let mut right = array.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if array[mid] > key {
            if mid == 0 {
                return None;
            }
            right = mid - 1;
        } else if array[mid] < key {
            left = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        println!("{:?}", find(&[1, 2], 0));
    }
}
