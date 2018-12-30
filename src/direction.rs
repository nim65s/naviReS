use rand::distributions::{Distribution, Standard};
use rand::Rng;

pub enum Direction {
    Haut,
    Bas,
    Gauche,
    Droite,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng +?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 4) {
            0 => Direction::Haut,
            1 => Direction::Bas,
            2 => Direction::Gauche,
            _ => Direction::Droite,
        }
    }
}
