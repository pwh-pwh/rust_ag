use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    //For want of a nail the shoe was lost.\n
    //And all for the want of a nail.
    match list.first() {
        None => Default::default(),
        Some(word) => list.windows(2)
            .map(|w|format!("For want of a {} the {} was lost.\n",w[0],w[1]))
            .chain(once(format!("And all for the want of a {}.",word)))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use super::*;
    #[test]
    fn test() {
        println!("{}", build_proverb(&["pin"]));
    }

    #[test]
    fn test2() {
        let mut n = 1;
        let mut f1 = move|| {
            n+=1;
            println!("f1:{}",n);
        };
        let mut f2 =  ||{
            n+=3;
        };
        f1();
        f2();
        f1();
        println!("{}",n);
    }
}

