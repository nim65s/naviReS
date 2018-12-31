#[derive(Copy, Clone)]
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
    pub fn nom(&self) -> &str {
        match self.rep {
            'A' => "Porte-Avion",
            'B' => "Croiseur",
            'C' => "Contre-Torpilleur",
            'D' => "Sous-Marin",
            _ => "Torpilleur",
        }
    }
}
