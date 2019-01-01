use crate::bateau::Bateau;

pub struct Case {
    pub tir: bool,          // si la case s’est prise un tir
    bateau: Option<Bateau>, // si la case a un bateau
}

impl Case {
    pub fn new() -> Case {
        Case {
            bateau: None,
            tir: false,
        }
    }

    pub fn libre(&self) -> bool {
        self.bateau.is_none()
    }

    pub fn add_bateau(&mut self, bateau: Bateau) {
        self.bateau.replace(bateau);
    }

    pub fn feu(&mut self) -> bool {
        self.tir = true;
        match &self.bateau {
            None => {
                println!("À l’eau !");
                false
            }
            Some(bateau) => {
                println!("{} touché !", bateau);
                true
            }
        }
    }

    pub fn to_char(&self, visible: bool) -> char {
        if visible {
            match &self.bateau {
                Some(bateau) => {
                    if self.tir {
                        'x' // touché
                    } else {
                        bateau.rep
                    }
                }
                None => {
                    if self.tir {
                        'o' // à l’eau
                    } else {
                        ' ' // rien
                    }
                }
            }
        } else if self.tir {
            match &self.bateau {
                Some(bateau) => bateau.rep.to_ascii_lowercase(), // touché
                None => 'O',                                     // à l’eau
            }
        } else {
            ' ' // mystère
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn libre() {
        let case = Case::new();
        assert!(case.libre());
    }

    #[test]
    fn to_char() {
        use crate::bateau::BATEAUX;

        let mut case = Case::new();
        assert_eq!(case.to_char(true), ' ');
        assert_eq!(case.to_char(false), ' ');
        assert!(!case.feu());
        assert_eq!(case.to_char(true), 'o');
        assert_eq!(case.to_char(false), 'O');

        let mut case = Case::new();
        case.add_bateau(BATEAUX[0]);
        assert_eq!(case.to_char(false), ' ');
        assert_eq!(case.to_char(true), 'A');
        assert!(case.feu());
        assert_eq!(case.to_char(false), 'a');
        assert_eq!(case.to_char(true), 'x');
    }
}
