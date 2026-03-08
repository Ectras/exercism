pub struct Allergies {
    score: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs = 0b00000001,
    Peanuts = 0b00000010,
    Shellfish = 0b00000100,
    Strawberries = 0b00001000,
    Tomatoes = 0b00010000,
    Chocolate = 0b00100000,
    Pollen = 0b01000000,
    Cats = 0b10000000,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score: score as u8 }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.score & (*allergen as u8)) > 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        const VARIANTS: [Allergen; 8] = [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        VARIANTS
            .iter()
            .filter(|al| self.is_allergic_to(al))
            .copied()
            .collect()
    }
}
