use std::collections::HashMap;

#[allow(clippy::new_without_default)]
pub struct School {
    data: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.data
            .entry(grade)
            .and_modify(|stus| {
                stus.push(student.into());
                stus.sort();
            })
            .or_insert(vec![student.into()]);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut r = self.data.keys().copied().collect::<Vec<u32>>();
        r.sort();
        r
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.data.get(&grade).cloned().unwrap_or_default()
    }
}
