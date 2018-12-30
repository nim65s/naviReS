#[derive(Copy, Clone)]
pub enum Bateau {
    PorteAvion,
    Croiseur,
    ContreTorpilleur,
    SousMarin,
    Torpilleur,
}

impl Bateau {
    pub fn to_char(&self) -> char {
        match self {
            Bateau::PorteAvion => 'A',
            Bateau::Croiseur => 'B',
            Bateau::ContreTorpilleur => 'C',
            Bateau::SousMarin => 'D',
            Bateau::Torpilleur => 'E',
        }
    }

    pub fn len(&self) -> i8 {
        match self {
            Bateau::PorteAvion => 5,
            Bateau::Croiseur => 4,
            Bateau::ContreTorpilleur => 3,
            Bateau::SousMarin => 3,
            Bateau::Torpilleur => 2,
        }
    }

    pub fn nom(&self) -> String {
        match self {
            Bateau::PorteAvion => String::from("Porte-Avion"),
            Bateau::Croiseur => String::from("Croiseur"),
            Bateau::ContreTorpilleur => String::from("Contre-Torpilleur"),
            Bateau::SousMarin => String::from("Sous-Marin"),
            Bateau::Torpilleur => String::from("Torpilleur"),
        }
    }
}
