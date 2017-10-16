extern crate rand;

pub struct Robot { name: String }

impl Robot {
    pub fn new() -> Robot { Robot { name: Self::get_unique_name() } }
    pub fn name<'a>(&'a self) -> &'a str { &self.name }
    pub fn reset_name(&mut self) { self.name = Self::get_unique_name(); }

    fn get_unique_name() -> String {
        let letters: Vec<char> = (0..26).map(|n| (n + 'A' as u8) as char).collect();
        let digits: Vec<char> = (0..10).map(|n| (n + '0' as u8) as char).collect();
        let mut rng = rand::thread_rng();
        vec![&letters, &letters, &digits, &digits, &digits]
            .iter()
            .map(|set| rand::sample(&mut rng, set.iter(), 1)[0])
            .collect()
    }
}
