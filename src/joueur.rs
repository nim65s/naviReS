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

    println!("colonne? [0..{}]: ", grille.max_char());
    let mut col = String::new();
    io::stdin()
        .read_line(&mut col)
        .expect("failed to read line");
    let col: char = col.trim().parse().expect("raté…");
    let col = col.to_digit(grille.taille() as u32).unwrap();

    println!("ligne? [0..{}]: ", grille.max_char());
    let mut lig = String::new();
    io::stdin()
        .read_line(&mut lig)
        .expect("failed to read line");
    let lig: char = lig.trim().parse().expect("raté…");
    let lig = lig.to_digit(grille.taille() as u32).unwrap();

    grille.pose_bateau(bateau, col as i8, lig as i8, horizontal)
}

pub fn start(grille: &mut Grille) {
    println!("Remplissage manuel de la grille du joueur…");
    'outer: loop {
        grille.vide();
        for bateau in BATEAUX.iter() {
            if !pose_bateau(grille, *bateau) {
                continue 'outer;
            }
        }
        break;
    }
}

pub fn joue(grille: &mut Grille) {
    println!("Au tour du joueur !");

    println!("colonne? [0..{}]: ", grille.max_char());
    let mut col = String::new();
    io::stdin()
        .read_line(&mut col)
        .expect("failed to read line");
    let col: char = col.trim().parse().expect("raté…");
    let col = col.to_digit(grille.taille() as u32).unwrap();

    println!("ligne? [0..{}]: ", grille.max_char());
    let mut lig = String::new();
    io::stdin()
        .read_line(&mut lig)
        .expect("failed to read line");
    let lig: char = lig.trim().parse().expect("raté…");
    let lig = lig.to_digit(grille.taille() as u32).unwrap();

    grille.feu(col as i8, lig as i8);
}
