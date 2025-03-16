use itertools::iproduct;
use std::str::FromStr;

/// Tuple-Struct compris des champs (signe, n, x, y), où :
/// - signe vaut `false` si on considère la négation du literal, `true` sinon
/// - n est le chiffre inscrit à la case donnée (n ∈ { 1, …, 9 })
/// - x est la position de la case sur l’axe des abscices (x ∈ { 1, …, 9 })
/// - y est la position de la case sur l’axe des ordonnées (y ∈ { 1, …, 9 })
/// Exemple : Literal(true, 2, 1, 1) signifie que le chiffre 2 est présent
/// dans le coin suppérieur gauche de la grille.
#[derive(PartialEq, Debug)]
pub struct Literal(pub bool, pub u8, pub u8, pub u8);

impl Literal {
    const DIMACS_MAX: i32 = Literal(true, 9, 9, 9).to_dimacs();

    /// Renvoie le litéral inscrit dans le format dimacs
    pub const fn to_dimacs(&self) -> i32 {
        let Literal(signe, n, x, y) = *self;
        let litteral_formate = (n as i32 - 1) * 81 + (x as i32 - 1) * 9 + y as i32;
        if signe == true {
            litteral_formate
        }
        else {
            -litteral_formate
        }
    }

    /// Renvoie un littéral à partir d’un littéral dimacs
    pub fn from_dimacs(mut litteral_dimacs: i32) -> Option<Self> {
        if litteral_dimacs == 0 || litteral_dimacs.abs() > Self::DIMACS_MAX {
            return None;
        }

        let signe: bool;
        if litteral_dimacs < 0 {
            signe = false;
            litteral_dimacs *= -1;
        }
        else {
            signe = true;
        }

        litteral_dimacs -= 1;
        let pos_y = ((litteral_dimacs % 9) + 1) as u8;
        let pos_x = (((litteral_dimacs / 9) % 9) + 1) as u8;
        let n = (((litteral_dimacs / 81) % 9) + 1) as u8;

        Some(Literal(signe, n, pos_x, pos_y))
    }
}

/// Un ensemble de littéraux.
/// note: Une clause peut être instancié à partir de tuples (bool, u8, u8, u8)
/// avec les implémentation de `From` et `FromIterator`, pour un peu de sucre syntaxique.
#[derive(PartialEq, Debug)]
pub struct Clause(Vec<Literal>);

impl Clause {
    /// Affiche des clauses lisibles par un humain
    #[allow(dead_code)]
    pub fn print_debug(&self) -> () {
        if self.0.len() == 0 { panic!("Cannot print an empty clause") };

        let format_literal = |lit: &Literal| -> String {
            let Literal(signe, n, x, y) = *lit;
            format!("{}{n}{x}{y}",  if signe == true { " + " } else { " - " })
        };

        let premier_literal = {
            let Literal(signe, n, x, y) = self.0[0];
            format!("{}{n}{x}{y}", if signe == true { "" } else { "-" })
        };

        if self.0.len() == 1 {
            println!("{}", premier_literal)
        } else {
            print!("({}", premier_literal);
            for lit in self.0[1..].iter() {
                print!("{}", format_literal(lit));
            }
            println!(")")
        }
    }
}

impl FromIterator<(bool, u8, u8, u8)> for Clause {
    fn from_iter<I: IntoIterator<Item = (bool, u8, u8, u8)>>(iter: I) -> Self {
        let mut vec = Vec::new();
        for (signe, n, x, y) in iter {
            vec.push(Literal(signe, n, x, y));
        }
        Self(vec)
    }
}

impl<const N: usize> FromIterator<[(bool, u8, u8, u8); N]> for Clause {
    fn from_iter<I: IntoIterator<Item = [(bool, u8, u8, u8); N]>>(iter: I) -> Self {
        let mut vec = Vec::new();
        for litteraux in iter {
            litteraux.iter().for_each(|&(s, n, x, y)| vec.push(Literal(s, n, x, y)));
        }
        Self(vec)
    }
}

impl<const N: usize> From<[(bool, u8, u8, u8); N]> for Clause {
    fn from(litteraux: [(bool, u8, u8, u8); N]) -> Self {
        Clause(Vec::from(litteraux.map(|(s, n, x, y)| Literal(s, n, x, y))))
    }
}

impl std::fmt::Display for Clause {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for lit in self.0.iter() {
            write!(fmt, "{} ", lit.to_dimacs())?;
        }
        write!(fmt, "0\n")
    }
}

/// La grille de sudoku
pub struct Grille([[Option<u8>; 9]; 9]);

impl Grille {
    pub fn new() -> Self { Grille([[None; 9]; 9]) }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<u8> {
        self.0[y - 1][x - 1]
    }

    pub fn set_cell(&mut self, value: u8, x: usize, y: usize) -> () {
        self.0[y - 1][x - 1] = Some(value);
    }

    pub fn get_line(&self, line_number: usize) -> [Option<u8>; 9] {
        self.0[line_number - 1]
    }

    pub fn get_collumn(&self, collumn_number: usize) -> [Option<u8>; 9] {
        self.0.map(|line| line[collumn_number - 1])
    }

    pub fn get_region(&self, mut pos_x: usize, mut pos_y: usize) -> [Option<u8>; 9] {
        pos_x -= 1;
        pos_y -= 1;
        [  // FIXME: C’est vraiment moche là srx
            self.0[pos_y * 3][pos_x * 3],
            self.0[pos_y * 3][pos_x * 3 + 1],
            self.0[pos_y * 3][pos_x * 3 + 2],
            self.0[pos_y * 3 + 1][pos_x * 3],
            self.0[pos_y * 3 + 1][pos_x * 3 + 1],
            self.0[pos_y * 3 + 1][pos_x * 3 + 2],
            self.0[pos_y * 3 + 2][pos_x * 3],
            self.0[pos_y * 3 + 2][pos_x * 3 + 1],
            self.0[pos_y * 3 + 2][pos_x * 3 + 2],
        ]
    }

    pub fn is_valid(&self) -> bool {
        let has_duplicates = |arr: &[u8]| -> bool {
            let mut seen_values = Vec::new();
            for n in arr.iter() {
                if seen_values.contains(n) { return true; }
                seen_values.push(*n);
            }
            false
        };

        let has_nones = |arr: &[Option<u8>]| -> bool {
            for o in arr.iter() {
                if *o == None { return true };
            };
            false
        };

        let valid_group = |group: &[Option<u8>; 9]| -> bool {
            if has_nones(group) { return false; };
            !has_duplicates(&group.map(|o| o.unwrap()))
        };

        for pos in 1..=9 {
            let region_x = (pos - 1) % 3 + 1;
            let region_y = (pos - 1) / 3 + 1;

            if !valid_group(&self.get_line(pos)) { return false }
            if !valid_group(&self.get_collumn(pos)) { return false }
            if !valid_group(&self.get_region(region_x, region_y)) { return false }
        }

        true
    }

    /// Renvoie un vecteur des clauses unitaires décrivant l’état actuel de la grille.
    pub fn get_litteraux(&self) -> Vec<Clause> {
        let coord_to_litteral = |(x, y): (u8, u8)| -> Option<Clause> {
            self.get_cell(x as usize, y as usize)
                .map(|n| Clause::from([(true, n, x, y)]))
        };

        iproduct!(1..=9, 1..=9).filter_map(coord_to_litteral).collect()
    }

    pub fn from_dimacs(clauses: &str) -> Option<Self> {
        let mut rv = Grille::new();
        let mut grille_valide = true;

        let mut applique_litteral = |grille: &mut Grille, lit: Literal| -> () {
            let Literal(signe, n, x, y) = lit;

            if signe == false { return; }
            if let Some(_) = grille.get_cell(x as usize, y as usize) {
                grille_valide = false;
            }

            grille.set_cell(n, x as usize, y as usize);
        };

        clauses.split_whitespace()
            .filter_map(|lit_dimacs_str| lit_dimacs_str.parse::<i32>().ok())
            .filter_map(|lit_dimacs| Literal::from_dimacs(lit_dimacs))
            .for_each(|litteral| applique_litteral(&mut rv, litteral));

        if grille_valide { Some(rv) }
        else { None }
    }
}

impl std::fmt::Display for Grille {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "+-----+-----+-----+\n")?;
        for y in 1..=9 {
            write!(fmt, "|")?;

            for x in 1..=9 {
                let char_a_afficher =
                    self.get_cell(x, y)
                        .and_then(|n| char::from_digit(n as u32, 10))
                        .unwrap_or(' ');

                write!(fmt, "{char_a_afficher}")?;

                write!(fmt, "{}", if x % 3 == 0 { "|" } else { " " })?;
            }

            write!(fmt, "\n")?;
            if y % 3 == 0 {
                write!(fmt, "+-----+-----+-----+\n")?;
            }
        }

        Ok(())
    }
}
pub enum GrilleParseError {
    ImpossibleParserLigne(usize),
    FormatInvalide,
}

impl FromStr for Grille {
    type Err = GrilleParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parse_num = |c: char| -> Option<u8> {
            if '1' <= c && c <= '9' { Some(c as u8 - '0' as u8) }
            else { None }
        };

        let parse_line = |(line_number, line): (usize, &str)|
            -> Result<[Option<u8>; 9], Self::Err>
        {
            line.chars()
                .skip(1)
                .step_by(2)
                .map(parse_num)
                .collect::<Vec<Option<u8>>>()
                .try_into()
                .or_else(|_| Err(GrilleParseError::ImpossibleParserLigne(line_number + 1)))
        };

        Ok(Self(input.lines()
            .filter(|l| l.starts_with('|'))
            .enumerate()
            .map(|line| parse_line(line))
            .collect::<Result<Vec<[Option<u8>; 9]>, Self::Err>>()?
            .try_into()
            .or_else(|_| Err(GrilleParseError::FormatInvalide))?))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn premier_litteral_dimacs() {
        assert_eq!(Literal(true, 1, 1, 1).to_dimacs(), 1);
    }

    #[test]
    fn collision_format_dimacs_literal() {
        use itertools::izip;

        let literaux = iproduct!(1..=9, 1..=9, 1..=9)
                .map(|(n, x, y)| Literal(true, n, x, y));
        let mut literaux_suivant = iproduct!(1..=9, 1..=9, 1..=9)
                .map(|(n, x, y)| Literal(true, n, x, y));
        literaux_suivant.next();

        for (lit1, lit2) in izip!(literaux, literaux_suivant) {
            println!("{}, {}", lit1.to_dimacs(), lit2.to_dimacs());
            assert!(lit1.to_dimacs() + 1 == lit2.to_dimacs());
        }
    }

    #[test]
    fn litteral_from_dimacs() {
        use itertools::izip;

        for (n1, x1, y1) in izip!(1..=9, 1..=9, 1..=9) {
            let Some(Literal(_, n2, x2, y2)) =
                    Literal::from_dimacs(Literal(false, n1, x1, y1).to_dimacs()) else {
                panic!("Literral::from_dimacs is beyond fucked")
            };
            println!("{n1}{x1}{y1} -> {n2}{x2}{y2}");
            assert_eq!((n1, x1, y1), (n2, x2, y2));
        }
    }

    #[test]
    fn clause_nb_fini_elem() {
        let clause1 = Clause::from( [(true, 1, 1, 1), (false, 2, 2, 3)] );
        let clause2 = Clause(vec![
                Literal(true, 1, 1, 1),
                Literal(false, 2, 2, 3),
        ]);
        assert_eq!(clause1, clause2);
    }

    #[test]
    fn clause_from_iter() {
        let clause1 = Clause::from_iter((1..=3).map(|y| (true, 1, 1, y)));
        let clause2 = Clause(vec![
                Literal(true, 1, 1, 1),
                Literal(true, 1, 1, 2),
                Literal(true, 1, 1, 3),
        ]);
        assert_eq!(clause1, clause2);
    }

    #[test]
    fn clause_from_iter_nb_fini() {
        let clause1 = Clause::from_iter((1..=3).map(|y| [
                (true,  1, 1, y),
                (false, 1, 1, y),
        ] ));
        let clause2 = Clause(vec![
                Literal(true,  1, 1, 1),
                Literal(false, 1, 1, 1),
                Literal(true,  1, 1, 2),
                Literal(false, 1, 1, 2),
                Literal(true,  1, 1, 3),
                Literal(false, 1, 1, 3),
        ]);
        assert_eq!(clause1, clause2);
    }

    // Grille valide choisie arbitrairement
    const GRILLE_VALIDE: Grille = Grille([
        [Some(4), Some(1), Some(9), Some(2), Some(7), Some(6), Some(8), Some(5), Some(3)], 
        [Some(6), Some(3), Some(8), Some(1), Some(9), Some(5), Some(2), Some(4), Some(7)], 
        [Some(7), Some(5), Some(2), Some(4), Some(3), Some(8), Some(9), Some(6), Some(1)], 
        [Some(3), Some(2), Some(7), Some(6), Some(1), Some(4), Some(5), Some(8), Some(9)], 
        [Some(8), Some(6), Some(1), Some(9), Some(5), Some(2), Some(7), Some(3), Some(4)], 
        [Some(9), Some(4), Some(5), Some(3), Some(8), Some(7), Some(1), Some(2), Some(6)], 
        [Some(2), Some(7), Some(3), Some(8), Some(4), Some(1), Some(6), Some(9), Some(5)], 
        [Some(5), Some(9), Some(6), Some(7), Some(2), Some(3), Some(4), Some(1), Some(8)], 
        [Some(1), Some(8), Some(4), Some(5), Some(6), Some(9), Some(3), Some(7), Some(2)], 
    ]);

    #[test]
    fn grille_arbitraire_valide() {
        assert!(GRILLE_VALIDE.is_valid());
    }

    #[test]
    fn grille_non_valide_case_vide() {
        let mut grille = GRILLE_VALIDE;
        grille.0[0][0] = None;
        assert!(!grille.is_valid());
    }

    #[test]
    fn grille_non_valide_chiffre_double() {
        let mut grille = GRILLE_VALIDE;
        grille.set_cell(5, 1, 1);
        assert!(!grille.is_valid());
    }
}
