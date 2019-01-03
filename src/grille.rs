use crate::bateau::Bateau;
use crate::case::Case;
use std::cmp::max;
use std::collections::HashMap;

pub struct Grille {
    carte: HashMap<(i8, i8), Case>,
    visible: bool, // généralement oui pour celle du joueur et non pour celle de l’adversaire
    taille: i8,    // hauteur et largeur
    restant: i8,   // Cases avec des cibles pas encore touchées
}

impl Grille {
    pub fn new(taille: i8, visible: bool) -> Grille {
        let mut grille = Grille {
            carte: HashMap::new(),
            restant: 0,
            taille,
            visible,
        };
        grille.vide();
        grille
    }

    pub fn vide(&mut self) {
        self.restant = 0;
        for col in 0..self.taille {
            for lig in 0..self.taille {
                self.carte.insert((col, lig), Case::new());
            }
        }
    }

    pub fn taille(&self) -> i8 {
        self.taille
    }

    pub fn max_char(&self) -> char {
        std::char::from_digit((self.taille - 1) as u32, 36).unwrap()
    }

    pub fn pose_bateau(&mut self, bateau: Bateau, col: i8, lig: i8, horizontal: bool) -> bool {
        let cases: Vec<(i8, i8)> = (0..bateau.len)
            .map(|i| {
                if horizontal {
                    (col + i, lig)
                } else {
                    (col, lig + i)
                }
            })
            .collect();

        if cases
            .iter()
            .all(|case| self.carte.contains_key(&case) && self.carte[&case].libre())
        {
            for case in cases {
                self.carte.get_mut(&case).unwrap().add_bateau(bateau);
            }
            self.restant += bateau.len;
            true
        } else {
            false
        }
    }

    pub fn deja_tire(&self, col: i8, lig: i8) -> bool {
        self.carte[&(col, lig)].tir
    }

    pub fn feu(&mut self, col: i8, lig: i8) {
        if self.deja_tire(col, lig) {
            println!("Vous avez déjà tiré ici…");
        } else if self.carte.get_mut(&(col, lig)).unwrap().feu() {
            self.restant -= 1;
        }
    }

    pub fn fin(&self) -> bool {
        self.restant == 0
    }

    pub fn restant(&self) -> i8 {
        self.restant
    }
}

pub fn show_grilles(joueur: &Grille, ia_pnj: &Grille) {
    println!("\n Restant: {} | {}\n", joueur.restant(), ia_pnj.restant());
    for lig in -1..=max(joueur.taille, ia_pnj.taille) {
        let mut s = String::new();

        for col in -1..=joueur.taille {
            if lig <= joueur.taille {
                s.push(match joueur.carte.get(&(col, lig)) {
                    Some(case) => case.to_char(joueur.visible),
                    None => bordure(col, lig, joueur.taille),
                });
            } else {
                s.push(' ');
            }
            s.push(' ');
        }

        s += " |  ";

        for col in -1..=ia_pnj.taille {
            if lig <= ia_pnj.taille {
                s.push(match ia_pnj.carte.get(&(col, lig)) {
                    Some(case) => case.to_char(ia_pnj.visible),
                    None => bordure(col, lig, ia_pnj.taille),
                });
            } else {
                s.push(' ');
            }
            s.push(' ');
        }
        println!("{}", s);
    }
}

fn bordure(col: i8, lig: i8, taille: i8) -> char {
    let bord = if lig == -1 || lig == taille { col } else { lig };
    std::char::from_digit(bord as u32, taille as u32).unwrap_or('+')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bateau::BATEAUX;

    #[test]
    fn grille() {
        let mut grille = Grille::new(10, true);

        // trivial
        assert_eq!(grille.taille(), 10);
        assert_eq!(grille.max_char(), '9');
        assert!(grille.fin());

        // mettre des bateaux au même endroit ou en dehors de la carte
        for bateau in BATEAUX.iter().rev() {
            println!("{}", bateau.len);
            if bateau.len == 2 {
                assert!(grille.pose_bateau(*bateau, 7, 0 as i8, true));
            } else {
                assert!(!grille.pose_bateau(*bateau, 7, 0 as i8, true));
            }
        }

        // mettre des bateaux correctement
        for (i, bateau) in BATEAUX.iter().enumerate() {
            assert!(grille.pose_bateau(*bateau, 0, i as i8, true));
        }
        assert!(!grille.fin());
        assert_eq!(grille.restant(), 19); // 5 bateaux corrects plus un torpilleur

        // tentative de tir
        assert!(!grille.deja_tire(0, 0));
        grille.feu(0, 0);
        assert!(grille.deja_tire(0, 0));

        // nuke it.
        for col in 0..10 {
            for lig in 0..10 {
                grille.feu(col, lig);
            }
        }
        assert!(grille.fin());

        // affichage de grilles de tailles différentes
        let grille_s = Grille::new(8, true);
        show_grilles(&grille, &grille_s);
        show_grilles(&grille_s, &grille);
    }
}
