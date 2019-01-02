use rand::Rng;

use crate::bateau::*;
use crate::grille::Grille;

fn pose_bateau(grille: &mut Grille, bateau: Bateau) -> bool {
    let mut rng = rand::thread_rng();

    let horizontal = rng.gen();
    let col = rng.gen_range(0, grille.max() - if horizontal { bateau.len } else { 0 });
    let lig = rng.gen_range(0, grille.max() - if horizontal { 0 } else { bateau.len });

    grille.pose_bateau(bateau, col, lig, horizontal)
}

pub fn start(grille: &mut Grille) {
    println!("Remplissage automatique de la grille de l’IA…");
    'outer: loop {
        grille.vide();
        for bateau in BATEAUX.iter() {
            if !pose_bateau(grille, *bateau) {
                continue 'outer;
            }
        }
        break;
    }
}

pub fn joue(grille: &mut Grille) {
    println!("Au tour de l’IA !");

    let mut rng = rand::thread_rng();
    let mut col;
    let mut lig;
    loop {
        col = rng.gen_range(0, grille.max());
        lig = rng.gen_range(0, grille.max());
        if !grille.deja_tire(col, lig) {
            break;
        }
    }

    println!("feu sur ({}, {})", col, lig);
    grille.feu(col, lig)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ia() {
        let mut grille = Grille::new(10, true);
        assert_eq!(grille.restant(), 0);
        start(&mut grille);
        assert_eq!(grille.restant(), 17);  // 5 bateaux
        joue(&mut grille);
        assert!(grille.restant() == 16 || grille.restant() == 17); // touché ou pas
    }
}
