#[derive(Debug)]
pub struct CustomSet<T: PartialEq + Clone> { set: Vec<T> }

impl<T: PartialEq + Clone> CustomSet<T>{
    pub fn new(items: Vec<T>) -> CustomSet<T> { 
        let mut set = CustomSet { set: vec![] };
        for item in items { set.add(item) }
        set
    }
    pub fn is_empty(&self) -> bool { self.set.is_empty() }
    pub fn add(&mut self, item: T) { if !self.contains(&item) { self.set.push(item) } }
    pub fn contains(&self, item: &T) -> bool { self.set.contains(item) }
    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool { 
        !self.set.iter().any(|i| other.contains(i))
    }
    pub fn is_subset(&self, other: &CustomSet<T>) -> bool { 
        self.set.iter().all(|i| other.contains(i))
    }
    pub fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> { 
        let mut result = Self::new(vec![]);
        for item in self.set.iter().cloned() { 
            if other.contains(&item) { result.add(item) }
        }
        result
    }
    pub fn union(&self, other: &CustomSet<T>) -> CustomSet<T> { 
        let mut union = Self::new(vec![]);
        for item in self.set.iter().chain(other.set.iter()).cloned() {
            union.add(item);
        }
        union
    }
    pub fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mut difference = Self::new(vec![]);
        for item in self.set.iter().cloned() {
            if !other.contains(&item) { difference.add(item); }
        }
        difference
    }
}

impl<T: Clone> PartialEq for CustomSet<T> where T: PartialEq {
    fn eq(&self, other: &CustomSet<T>) -> bool {
       self.is_subset(other) && other.is_subset(self)
    }
}
