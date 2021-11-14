use std::io;

use crate::bateau::*;
use crate::grille::Grille;

fn pose_bateau(grille: &mut Grille, bateau: Bateau) -> bool {
    println!("\nPosons le {} ({} cases)", bateau, bateau.len);

    println!("doit-il être horizontal? [true,false]: ");
    let mut horizontal = String::new();
    io::stdin()
        .read_line(&mut horizontal)
        .expect("failed to read line");
    let horizontal = horizontal.trim().parse().expect("raté…");

    let col = input_number("colonne", grille.max_char());
    let lig = input_number("ligne", grille.max_char());

    grille.pose_bateau(bateau, col as i8, lig as i8, horizontal)
}

pub fn start(grille: &mut Grille) {
    println!("Remplissage manuel de la grille du joueur…");
    loop {
        grille.vide();
        if BATEAUX.iter().all(|bateau| pose_bateau(grille, *bateau)) {
            break;
        }
    }
}

pub fn joue(grille: &mut Grille) {
    println!("Au tour du joueur !");

    let col = input_number("colonne", grille.max_char());
    let lig = input_number("ligne", grille.max_char());

    grille.feu(col as i8, lig as i8);
}

fn input_number(name: &str, max: char) -> i8 {
    println!("{}? [0..{}]: ", name, max);
    let mut ret = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("failed to read line");
    let ret: char = ret.trim().parse().expect("raté…");
    ret.to_digit(36).unwrap() as i8
}
