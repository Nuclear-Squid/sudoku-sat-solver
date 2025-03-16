use std::fs;
use std::io;

// import the contents of `lib.rs`
use sudoku_dimacs::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Faut y donner un nom de fichier en entrée.");
    }

    let contenu_sortie_sat = fs::read_to_string(&args[1])?;
    let lignes_sortie_sat: Vec<&str> = contenu_sortie_sat.lines().collect();

    let grille_opt = match lignes_sortie_sat[..] {
        ["SAT", clauses] => Grille::from_dimacs(clauses),
        ["UNSAT"] => {
            println!("Le sudoku n’a pas de solutions");
            return Ok(());
        },
        _ => { panic!("Fichier d’entrée invalide"); }
    };

    let Some(grille) = grille_opt else {
        println!("Erreur : Plusieures chiffres sont présents sur une même case.");
        return Ok(());
    };

    if !grille.is_valid() {
        println!("/!\\ GRILLE INVALIDE :");
    }
    println!("{grille}");

    Ok(())
}
