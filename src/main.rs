mod bateau;
mod case;
mod grille;
mod joueur;
mod ia;

use crate::grille::*;


fn main() {
    println!("Hello, world!");

    let mut joueur = Grille::new(11, true);
    let mut ia_pnj = Grille::new(10, true);

    ia::start_ia_pnj(&mut ia_pnj);
    joueur::start_joueur(&mut joueur);

    show_grilles(&joueur, &ia_pnj);
}
