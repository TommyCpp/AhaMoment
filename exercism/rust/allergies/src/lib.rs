pub struct Allergies(Vec<Allergen>);

#[derive(Debug, PartialEq)]
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

fn get_allergen_by_index(idx: u32) -> Option<Allergen> {
    match idx {
        0 => Some(Allergen::Eggs),
        1 => Some(Allergen::Peanuts),
        2 => Some(Allergen::Shellfish),
        3 => Some(Allergen::Strawberries),
        4 => Some(Allergen::Tomatoes),
        5 => Some(Allergen::Chocolate),
        6 => Some(Allergen::Pollen),
        7 => Some(Allergen::Cats),
        _ => None
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut res = Allergies(Vec::new());
        for i in 0..8 {
            if u32::pow(2, i) & score == u32::pow(2, i) {
                res.0.push(get_allergen_by_index(i).unwrap())
            }
        }
        res
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.iter()
            .filter(|a| *a == allergen)
            .collect::<Vec<&Allergen>>().len() != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut res = Vec::new();
        for allergen in &self.0{
            res.push(match allergen{
                &Allergen::Eggs => Allergen::Eggs,
                &Allergen::Peanuts => Allergen::Peanuts,
                &Allergen::Shellfish => Allergen::Shellfish,
                &Allergen::Strawberries => Allergen::Strawberries,
                &Allergen::Tomatoes => Allergen::Tomatoes,
                &Allergen::Chocolate => Allergen::Chocolate,
                &Allergen::Pollen => Allergen::Pollen,
                &Allergen::Cats => Allergen::Cats,
            })
        }
        res
    }
}
