use rand::Rng;

use crate::grille::Grille;
use crate::bateau::Bateau;

fn pose_bateau(grille: &mut Grille, bateau: Bateau) -> bool {
    let mut rng = rand::thread_rng();

    let horizontal = rng.gen();
    let col = rng.gen_range(0, grille.max() - if horizontal { bateau.len() } else { 0 });
    let lig = rng.gen_range(0, grille.max() - if horizontal { 0 } else { bateau.len() });

    grille.pose_bateau(bateau, col, lig, horizontal)
}


pub fn start_ia_pnj(grille: &mut Grille) {
    loop {
        grille.vide();
        if !pose_bateau(grille, Bateau::PorteAvion) { continue; }
        if !pose_bateau(grille, Bateau::Croiseur) { continue; }
        if !pose_bateau(grille, Bateau::ContreTorpilleur) { continue; }
        if !pose_bateau(grille, Bateau::SousMarin) { continue; }
        if !pose_bateau(grille, Bateau::Torpilleur) { continue; }
        break;
    }
}
