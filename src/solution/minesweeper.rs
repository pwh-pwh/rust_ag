pub fn isIn(row:usize,col:usize,a:usize,b:usize) -> bool {
    a>=0&&b>=0&&a<row&&b<col
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let row = minefield.len();
    if row ==0 {
        return vec![];
    }
    let col = minefield[0].len();
    let mut res = vec![];
    for i in 0..row {
        let mut s = String::from("");
        for j in 0..col {
            match minefield[i].as_bytes()[j] {
                b'*' => {
                    s.push('*');
                },
                b' ' => {
                    let mut c = 0;
                    let start = if i==0 {0}else {i-1};
                    for a in start..=i+1 {
                        let start = if j==0{0} else {j-1};
                        for b in start..=j+1 {
                            if isIn(row,col,a,b) {
                                if minefield[a].as_bytes()[b] == b'*' {
                                    c+=1;
                                }
                            }
                        }
                    }
                    if c==0 {
                        s.push(' ');
                    } else {
                        s.push_str(c.to_string().as_str());
                    }
                },
                _ => {}
            }
        }
        res.push(s);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::solution::minesweeper::annotate;

    #[test]
    fn test1() {

        let m = &[" * * "];
        let vec = annotate(m);
        for x in vec {
            println!("{}",x);
        }
    }


}