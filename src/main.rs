mod bateau;
mod case;
mod grille;
mod joueur;
mod ia_pnj;

use crate::grille::*;


fn main() {
    println!("Hello, world!");

    let mut grille_joueur = Grille::new(11, true);
    let mut grille_ia_pnj = Grille::new(9, false);

    ia_pnj::start(&mut grille_ia_pnj);
    ia_pnj::start(&mut grille_joueur);
    //joueur::start(&mut grille_joueur);

    show_grilles(&grille_joueur, &grille_ia_pnj);
    loop {
        joueur::joue(&mut grille_ia_pnj);
        ia_pnj::joue(&mut grille_joueur);
        show_grilles(&grille_joueur, &grille_ia_pnj);
    }
}
