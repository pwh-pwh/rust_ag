use rand::rngs::ThreadRng;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashSet;

pub struct Robot {
    name: String,
}

thread_local!(static User_Name:RefCell<HashSet<String>> = RefCell::new(HashSet::new()));

impl Robot {
    pub fn new() -> Self {
        Self {
            name: Self::next_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        User_Name.with(|c| c.borrow_mut().remove(&self.name));
        self.name = Self::next_name();
    }
    fn next_name() -> String {
        User_Name.with(|c| {
            let mut user_name = c.borrow_mut();
            let mut rng = rand::thread_rng();
            loop {
                let name = format!(
                    "{}{}{:03}",
                    rng.gen_range(b'A'..b'Z') as char,
                    rng.gen_range(b'A'..b'Z') as char,
                    rng.gen_range(0..1000)
                );
                if !user_name.contains(&name) {
                    user_name.insert(name.clone());
                    return name;
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_robot_name() {
        let robot = Robot::new();
        println!("{}", robot.name);
        panic!("test");
    }
}
