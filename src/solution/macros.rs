#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ($($elem:expr => $elemv:expr), +$(,)?) => {
        {
            use ::std::collections::HashMap;
            let mut m = HashMap::new();
        $(
            m.insert($elem,$elemv);
        )*
        m
        }
    };
}
//DESIGN OF STRACKING
//design of stracking

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::hash::Hash;

    #[test]
    fn test() {
        let mut expected = HashMap::new();
        expected.insert(1, "one");
        assert_eq!(hashmap!(1 => "one"), expected);
    }
    #[test]
    fn test1() {
        let s =
            "Design and implementation of Go-based campus epidemic prevention and control system"
                .to_uppercase();
        println!("{}", s);
    }
}
