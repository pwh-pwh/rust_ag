use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug)]
pub struct CustomSet<T>
where
    T: PartialEq,
{
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    data: HashSet<T>,
}

impl<T> PartialEq for CustomSet<T>
where
    T: PartialEq + Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.data.difference(&other.data).count() == 0
            && other.data.difference(&self.data).count() == 0
    }
}

impl<T> CustomSet<T>
where
    T: Hash + PartialEq + Eq + Clone,
{
    pub fn new(input: &[T]) -> Self {
        Self {
            data: {
                let mut set = HashSet::new();
                for x in input {
                    set.insert(x.clone());
                }
                set
            },
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.data.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.is_subset(&other.data)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.data.is_disjoint(&other.data)
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let set = self
            .data
            .intersection(&other.data)
            .map(|c| c.clone())
            .collect::<HashSet<T>>();
        Self { data: set }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let set: HashSet<T> = self
            .data
            .difference(&other.data)
            .map(|c| c.clone())
            .collect();
        Self { data: set }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let set: HashSet<T> = self.data.union(&other.data).map(|c| c.clone()).collect();
        Self { data: set }
    }
}
