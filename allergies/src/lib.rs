pub struct Allergies {score: u32}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies_vector: Vec<Allergen> = vec![];
        let all_allergies = vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        for (index, each_allergy) in all_allergies.into_iter().enumerate() {
            let value_of_allergy = 2_u32.pow(index as u32);
            if value_of_allergy & self.score == value_of_allergy {
                allergies_vector.push(each_allergy)
            } 
        }

        allergies_vector

    }
}
