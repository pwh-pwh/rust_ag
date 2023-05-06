pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    string.chars().into_iter().for_each(|c| match c {
        '(' | '{' | '[' => {
            stack.push(c);
        }
        ')' => {
            if stack.pop() != Some('(') {
                stack.push(c);
                return;
            }
        }
        ']' => {
            if stack.pop() != Some('[') {
                stack.push(c);
                return;
            }
        }
        '}' => {
            if stack.pop() != Some('{') {
                stack.push(c);
                return;
            }
        }
        _ => {}
    });
    stack.is_empty()
}
