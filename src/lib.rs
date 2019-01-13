mod bateau;
mod case;
mod grille;
mod ia_pnj;
mod joueur;

use crate::grille::*;

pub struct Config {
    taille_joueur: i8,
    taille_ia_pnj: i8,
    triche: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let mut taille_joueur = 10;
        let mut taille_ia_pnj = 10;
        let mut triche = false;

        if args.len() > 1 {
            taille_joueur = args[1].trim().parse().unwrap_or(10);
            if args.len() > 2 {
                taille_ia_pnj = args[2].trim().parse().unwrap_or(10);
                if args.len() > 3 {
                    triche = args[3].trim().parse().unwrap_or(false);
                }
            }
        }

        Config {
            taille_joueur,
            taille_ia_pnj,
            triche,
        }
    }
}

pub fn run(config: Config) {
    let mut grille_joueur = Grille::new(config.taille_joueur, true);
    let mut grille_ia_pnj = Grille::new(config.taille_ia_pnj, config.triche);

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
