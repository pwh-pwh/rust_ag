#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: Eq + PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use Comparison::*;
    let fLen = first_list.len();
    let sLen = second_list.len();
    match (fLen, sLen) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => {
            let x = first_list.windows(sLen).any(|c| c == second_list);
            if x {
                Superlist
            } else {
                Unequal
            }
        }
        (m, n) if m < n => {
            let x = second_list.windows(fLen).any(|c| c == first_list);
            if x {
                Sublist
            } else {
                Unequal
            }
        }
        _ => {
            if first_list == second_list {
                Equal
            } else {
                Unequal
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{sublist, Comparison};
    #[test]
    fn test1() {
        assert_eq!(Comparison::Sublist, sublist(&[], &['a', 's', 'd', 'f']));
        assert_eq!(Comparison::Sublist, sublist(&[], &['a', 's', 'd', 'f']));
        assert_eq!(Comparison::Superlist, sublist(&['a', 's', 'd', 'f'], &[]));
        assert_eq!(Comparison::Sublist, sublist(&[1, 2, 3], &[1, 2, 3, 4, 5]));
        assert_eq!(
            Comparison::Unequal,
            sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 3, 2, 3, 2, 1])
        );
    }
}
