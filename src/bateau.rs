pub struct Bateau {
    pub rep: char,
    pub len: i8,
}

pub const BATEAUX : [Bateau; 5] = [
    Bateau { rep: 'A', len: 5},
    Bateau { rep: 'B', len: 4},
    Bateau { rep: 'C', len: 3},
    Bateau { rep: 'D', len: 3},
    Bateau { rep: 'E', len: 2},
];

impl Bateau {
    pub fn copy(&self) -> Bateau {
        Bateau { rep: self.rep, len: self.len }
    }

    pub fn nom(&self) -> String {
        match self.rep {
            'A' => String::from("Porte-Avion"),
            'B' => String::from("Croiseur"),
            'C' => String::from("Contre-Torpilleur"),
            'D' => String::from("Sous-Marin"),
            _ => String::from("Torpilleur"),
        }
    }
}
