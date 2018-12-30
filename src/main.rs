mod bateau;
mod case;
mod grille;
mod joueur;
mod ia_pnj;

use crate::grille::*;


fn main() {
    println!("Hello, world!");

    let mut joueur = Grille::new(11, true);
    let mut ia_pnj = Grille::new(9, false);

    ia_pnj::start(&mut ia_pnj);
    ia_pnj::start(&mut joueur);
    //joueur::start(&mut joueur);

    show_grilles(&joueur, &ia_pnj);
    loop {
        joueur::joue(&mut ia_pnj);
        show_grilles(&joueur, &ia_pnj);
    }
}
