use rand::Rng;

use crate::grille::Grille;
use crate::bateau::*;

fn pose_bateau(grille: &mut Grille, bateau: &Bateau) -> bool {
    let mut rng = rand::thread_rng();

    let horizontal = rng.gen();
    let col = rng.gen_range(0, grille.max() - if horizontal { bateau.len } else { 0 });
    let lig = rng.gen_range(0, grille.max() - if horizontal { 0 } else { bateau.len });

    grille.pose_bateau(bateau, col, lig, horizontal)
}


pub fn start_ia_pnj(grille: &mut Grille) {
    println!("Remplissage automatique de la grille de l’IA…");
    'outer: loop {
        grille.vide();
        for bateau in BATEAUX.iter() {
            if !pose_bateau(grille, bateau) { continue 'outer; }
        }
        break;
    }
}
