
static OFFSETS: &'static [(i32,i32)] = &[
    (-1,-1),(-1,0),(-1,1),
    (0,-1),       (0,1),
    (1,-1),(1,0),(1,1)
];

pub fn annotate(field: &[&str]) -> Vec<String> {
    let height = field.len() as i32;
    (0..height).map(|y|{
        let width = field[y as usize].len() as i32;
        (0..width).map(|x| {
            if field[y as usize].as_bytes()[x as usize] == b'*' {
                '*'
            } else {
                match OFFSETS.iter()
                    .map(|&(ox,oy)|(x+ox,y+oy))
                    .filter(|&(x,y)|(0<=x&&x<width)&&(0<=y&&y<height))
                    .filter(|&(x,y)|field[y as usize].as_bytes()[x as usize] == b'*')
                    .count() {
                    0 => ' ',
                    n => (n as u8 + '0' as u8) as char
                }
            }
        }).collect()
    }).collect()
}