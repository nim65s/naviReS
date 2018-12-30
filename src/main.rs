mod bateau;
mod case;
mod grille;
mod joueur;
mod ia;
mod direction;

use crate::grille::*;


fn main() {
    println!("Hello, world!");

    let mut joueur = Grille::new(11, true);
    let mut ia_pnj = Grille::new(10, true);

    println!("Remplissage de la grille de l’IA…");
    ia::start_ia_pnj(&mut ia_pnj);
    println!("Remplissage de la grille du Joueur…");
    joueur::start_joueur(&mut joueur);
    //start_joueur(&mut joueur);

    show_grilles(&joueur, &ia_pnj);
}
