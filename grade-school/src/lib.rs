use std::collections::HashMap;

pub struct School { grades: HashMap<u32, Vec<String>>, }

impl School {
    pub fn new() -> School { School { grades: HashMap::new() } }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades.entry(grade).or_insert(Vec::new()).push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result = self.grades.keys().map(|&x| x).collect::<Vec<u32>>();
        result.sort();
        result
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.grades.get(&grade).map(|v| { let mut c = v.clone(); c.sort(); c })
    }
}
