
/// Tuple-Struct compris des champs (signe, n, x, y), où :
/// - signe vaut `false` si on considère la négation du literal, `true` sinon
/// - n est le chiffre inscrit à la case donnée (n ∈ { 1, …, 9 })
/// - x est la position de la case sur l’axe des abscices (x ∈ { 1, …, 9 })
/// - y est la position de la case sur l’axe des ordonnées (y ∈ { 1, …, 9 })
/// Exemple : Literal(true, 2, 1, 1) signifie que le chiffre 2 est présent
/// dans le coin suppérieur gauche de la grille.
pub struct Literal(pub bool, pub u8, pub u8, pub u8);

impl Literal {
    /// Renvoie le litéral inscrit dans le format dimacs
    pub fn to_dimacs(&self) -> i32 {
        let Literal(signe, n, x, y) = *self;
        let litteral_formate = (n as i32 - 1) * 81 + (x as i32 - 1) * 9 + y as i32;
        if signe == true {
            litteral_formate
        }
        else {
            -litteral_formate
        }
    }
}

/// Un ensemble de littéraux.
/// note: Une clause peut être instancié à partir de tuples (bool, u8, u8, u8)
/// avec les implémentation de `From` et `FromIterator`, pour un peu de sucre syntaxique.
pub struct Clause(Vec<Literal>);

impl Clause {
    /// Affiche des clauses lisibles par un humain
    #[allow(dead_code)]
    fn print_debug(&self) -> () {
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

