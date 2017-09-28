#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Allergen {
    Eggs = 0,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
    Last,
}

pub struct Allergies { code: u32 }

impl Allergies {
    pub fn new(code: u32) -> Allergies { Allergies{ code: code } }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        0 != self.code & 2_u32.pow(allergen.clone() as u32)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..(Allergen::Last as u32))
            .map(|n| Allergies::from_u32(n))
            .filter(|a| self.is_allergic_to(&a))
            .collect()
    }

    fn from_u32(n: u32) -> Allergen {
        match n {
            0 => Allergen::Eggs,
            1 => Allergen::Peanuts,
            2 => Allergen::Shellfish,
            3 => Allergen::Strawberries,
            4 => Allergen::Tomatoes,
            5 => Allergen::Chocolate,
            6 => Allergen::Pollen,
            7 => Allergen::Cats,
            _ => panic!("Not a valid allergen"),
        }
    }
}
