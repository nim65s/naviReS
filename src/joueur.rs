use std::io;

use crate::grille::Grille;
use crate::bateau::*;

fn pose_bateau(grille: &mut Grille, bateau: &Bateau) -> bool {
    println!("\nPosons le {} ({} cases)", bateau.nom(), bateau.len);
    let mut horizontal = String::new();
    let mut col = String::new();
    let mut lig = String::new();

    let taille = std::char::from_digit((grille.max() - 1) as u32, 36).unwrap();

    println!("doit-il être horizontal? [true,false]: ");
    io::stdin().read_line(&mut horizontal).expect("failed to read line");
    let horizontal = horizontal.trim().parse().expect("raté…");

    println!("colonne? [0..{}]: ", taille);
    io::stdin().read_line(&mut col).expect("failed to read line");
    let col: char = col.trim().parse().expect("raté…");
    let col = col.to_digit(36).unwrap();

    println!("ligne? [0..{}]: ", taille);
    io::stdin().read_line(&mut lig).expect("failed to read line");
    let lig: char = lig.trim().parse().expect("raté…");
    let lig = lig.to_digit(36).unwrap();

    grille.pose_bateau(bateau, col as i8, lig as i8, horizontal)
}


pub fn start_joueur(grille: &mut Grille) {
    println!("Remplissage manuel de la grille du joueur…");
    'outer: loop {
        grille.vide();
        for bateau in BATEAUX.iter() {
            if !pose_bateau(grille, bateau) { continue 'outer; }
        }
        break;
    }
}
