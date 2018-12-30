use crate::bateau::Bateau;

pub struct Case {
    pub tir: bool,  // si la case s’est prise un tir
    bateau: Option<Bateau>,  // si la case a un bateau
}

impl Case {
    pub fn new() -> Case {
        Case { bateau: None, tir: false}
    }

    pub fn libre(&self) -> bool {
        self.bateau.is_none()
    }

    pub fn add_bateau(&mut self, bateau: &Bateau) {
        self.bateau.replace(bateau.copy());
    }

    pub fn feu(&mut self) {
        self.tir = true;
        match &self.bateau {
            None => {
                println!("À l’eau !");
            },
            Some(bateau) => {
                println!("{} touché !", bateau.nom());
            },
        }
    }

    pub fn to_char(&self, visible: bool) -> char {
        if visible {
            match &self.bateau {
                Some(bateau) =>
                    if self.tir {
                        'x' // touché
                    } else {
                        bateau.rep
                    }
                ,
                None =>
                    if self.tir {
                        'o' // à l’eau
                    } else {
                        ' ' // rien
                    }
                ,
            }
        } else {
            if self.tir {
                match &self.bateau {
                    Some(bateau) => bateau.rep.to_ascii_lowercase(), // touché
                    None => 'O' // à l’eau
                }
            } else {
                ' ' // mystère
            }
        }
    }
}
