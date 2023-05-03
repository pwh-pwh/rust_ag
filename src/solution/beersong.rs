pub fn verse(n: u32) -> String {
    //4 bottles of beer on the wall, 4 bottles of beer.
    // Take one down and pass it around, 3 bottles of beer on the wall.
    if n == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    } else if n == 1 {
        "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string()
    } else if n == 2 {
        "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string()
    } else {
        format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n",n,n,n-1)
    }
}

pub fn sing(from: u32, to: u32) -> String {
    (to..from + 1)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solution::beersong::{sing, verse};

    #[test]
    fn test() {
        println!("{}", verse(1));
    }
    #[test]
    fn test2() {
        println!("{}", sing(3, 0));
    }
}

//8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n
//8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n\n6 bottles of beer on the wall, 6 bottles of beer.\nTake one down and pass it around, 5 bottles of beer on the wall.\n
