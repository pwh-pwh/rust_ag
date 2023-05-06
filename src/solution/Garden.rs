use std::ops::Index;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let diagram = diagram.replace("\n", "");
    let c = student.chars().next().unwrap() as u8 - 'A' as u8;
    let first = c * 2;
    let mut r = vec![];
    r.push(getG(diagram.as_bytes()[first as usize]));
    r.push(getG(diagram.as_bytes()[(first + 1) as usize]));
    let len = diagram.len() / 2;
    let second = first as usize + len;
    r.push(getG(diagram.as_bytes()[second as usize]));
    r.push(getG(diagram.as_bytes()[(second + 1) as usize]));
    r
}

fn getG(c: u8) -> &'static str {
    match c as char {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let d = "VCRC";
        let student = "Alice";
        println!("{:?}", plants(d, student));
    }
}
