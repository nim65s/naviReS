use std::collections::HashMap;
use crate::case::Case;
use crate::bateau::Bateau;
use std::cmp::max;

pub struct Grille {
    carte: HashMap<(i8, i8), Case>,
    visible: bool,
    taille: i8,
}

impl Grille {
    pub fn new(taille: i8, visible: bool) -> Grille {
        let mut grille = Grille { carte: HashMap::new(), taille, visible };
        grille.vide();
        grille
    }

    pub fn vide(&mut self) {
        for col in 0..self.taille {
            for lig in 0..self.taille {
                self.carte.insert((col, lig), Case::new());
            }
        }

    }

    pub fn max(&self) -> i8 {
        self.taille
    }

    pub fn max_char(&self) -> char {
        std::char::from_digit((self.max() - 1) as u32, 36).unwrap()
    }

    pub fn pose_bateau(&mut self, bateau: &Bateau, col: i8, lig: i8, horizontal: bool) -> bool {
        let c1 = col;
        let l1 = lig;
        let c2 = col + if horizontal { bateau.len - 1} else { 0 };
        let l2 = lig + if horizontal { 0 } else { bateau.len - 1};
        let mut cases = Vec::new();

        for col in c1..=c2 {
            for lig in l1..=l2 {
                match self.carte.get(&(col, lig)) {
                    None => {return false;},
                    Some(case) => match case.libre() {
                        false => {return false;},
                        true => {cases.push((col, lig));},
                    },
                }
            }
        }

        for (col, lig) in &cases {
            self.set(*col, *lig, bateau);
        }
        true
    }

    fn set(&mut self, col: i8, lig: i8, bateau: &Bateau) {
        self.carte.get_mut(&(col, lig)).unwrap().add_bateau(bateau);
    }

    pub fn deja_tire(& self, col: i8, lig: i8) -> bool{
        self.carte.get(&(col, lig)).unwrap().tir
    }

    pub fn feu(&mut self, col: i8, lig: i8) {
        if self.deja_tire(col, lig) {
            println!("Vous avez déjà tiré ici…");
        } else {
            self.carte.get_mut(&(col, lig)).unwrap().feu();
        }
    }
}

pub fn show_grilles(joueur: &Grille, ia_pnj: &Grille) {
    for lig in -1..=max(joueur.taille, ia_pnj.taille) {
        let mut s = String::new();

        for col in -1..=joueur.taille {
            if lig <= joueur.taille {
                s.push(match joueur.carte.get(&(col, lig)) {
                    Some(case) => case.to_char(joueur.visible),
                    None => bordure(col, lig, joueur.taille),
                });
            }
            else {
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
    match std::char::from_digit(bord as u32, taille as u32) {
        Some(c) => c,
        None => '+',
    }
}
