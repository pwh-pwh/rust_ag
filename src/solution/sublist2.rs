use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: Eq+PartialEq>(f: &[T], s: &[T]) -> Comparison {
    use Comparison::*;
    use std::cmp::Ordering;
    match f.len().cmp(&s.len()) {
        Ordering::Equal => if f==s {
            Equal
        }else {
            Unequal
        },
        Ordering::Less => if isCt(s,f) {
            Sublist
        } else {
            Unequal
        },
        Ordering::Greater => if isCt(f,s) {
            Superlist
        } else {
            Unequal
        },
    }
}

fn isCt<T:PartialEq>(a:&[T],b:&[T]) ->bool {
    b.is_empty()|| a.windows(b.len()).any(|it|it==b)
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::thread;
    use std::thread::spawn;
    use super::{Comparison, sublist};
    #[test]
    fn test1() {
        assert_eq!(Comparison::Sublist, sublist(&[], &['a', 's', 'd', 'f']));
        assert_eq!(Comparison::Sublist, sublist(&[], &['a', 's', 'd', 'f']));
        assert_eq!(Comparison::Superlist, sublist(&['a', 's', 'd', 'f'], &[]));
        assert_eq!(Comparison::Sublist, sublist(&[1, 2, 3], &[1, 2, 3, 4, 5]));
        assert_eq!(Comparison::Unequal,sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 3, 2, 3, 2, 1]));
    }


    fn test() {

    }

    fn get_str<'a>() -> &'a str {
        "a"
    }


}