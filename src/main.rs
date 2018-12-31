mod bateau;
mod case;
mod grille;
mod joueur;
mod ia_pnj;

use crate::grille::*;


fn main() {
    println!("Hello, world!");

    let mut grille_joueur = Grille::new(10, true);
    let mut grille_ia_pnj = Grille::new(10, false);

    ia_pnj::start(&mut grille_ia_pnj);
    joueur::start(&mut grille_joueur);

    show_grilles(&grille_joueur, &grille_ia_pnj);
    loop {
        joueur::joue(&mut grille_ia_pnj);
        if grille_ia_pnj.fin() {
            println!("Le joueur Gagne !");
            break;
        }
        show_grilles(&grille_joueur, &grille_ia_pnj);

        ia_pnj::joue(&mut grille_joueur);
        if grille_joueur.fin() {
            println!("L’IA Gagne !");
            break;
        }
        show_grilles(&grille_joueur, &grille_ia_pnj);
    }
}
