use std::fs::{ self, File };
use std::io::{ self, Write, BufWriter, Seek, SeekFrom };
use std::str::FromStr;
use itertools::iproduct;

pub mod clauses;
use clauses::*;

/// La grille de sudoku
struct Grille([[Option<u8>; 9]; 9]);

impl Grille {
    fn get_cell(&self, x: usize, y: usize) -> Option<u8> {
        self.0[y - 1][x - 1]
    }

    /// Renvoie un vecteur des clauses unitaires décrivant l’état actuel de la grille.
    fn get_litteraux(&self) -> Vec<Clause> {
        let coord_to_litteral = |(x, y): (u8, u8)| -> Option<Clause> {
            self.get_cell(x as usize, y as usize)
                .map(|n| Clause::from([(true, n, x, y)]))
        };

        iproduct!(1..=9, 1..=9).filter_map(coord_to_litteral).collect()
    }
}

enum GrilleParseError {
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

enum Output {
    OutFile(BufWriter<File>),
    StdOut,
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::OutFile(buff_writer) => buff_writer.write(buf),
            Self::StdOut => io::stdout().write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::OutFile(buff_writer) => buff_writer.flush(),
            Self::StdOut => io::stdout().flush(),
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Faut y donner un nom de fichier en entrée.");
    }

    use Output::*;
    let mut output = match args.get(2) {
        Some(out_file_name) => OutFile(BufWriter::new(File::create(out_file_name)?)),
        None => StdOut,
    };

    let sudoku = match Grille::from_str(fs::read_to_string(&args[1])?.as_str()) {
        Ok(grille) => grille,
        Err(e) => match e {
            GrilleParseError::ImpossibleParserLigne(n) => {
                panic!("Impossible de parser la ligne {n} dans le sudoku.");
            },
            GrilleParseError::FormatInvalide => {
                panic!("Le sudoku n’est pas au bon format");
            },
        },
    };

    let mut nb_clauses = 0;

    // Le nombre de clauses est inscrit à la fin du programme,
    // les espaces sont là pour 'réserver' la place nécessaire.
    write!(output, "p cnf {} x                 \n", Literal(true, 9, 9, 9).to_dimacs())?;

    write!(output, "c Clauses Grille\n")?;
    for litteral in sudoku.get_litteraux().iter() {
        nb_clauses += 1;
        write!(output, "{}", litteral)?;
    }

    // RJ (gauche)
    write!(output, "c RJ (gauche)\n")?;
    for (x, y) in iproduct!(1..=9, 1..=9) {
        nb_clauses += 1;
        write!(output, "{}", Clause::from_iter((1..=9).map(|n| (true, n, x, y))))?;
    }

    // RJ (droit)
    write!(output, "c RJ (droite)\n")?;
    for (n1, x, y) in iproduct!(1..=9, 1..=9, 1..=9) {
        nb_clauses += 1;
        write!(output, "{}", Clause::from_iter((1..=9)
                    .filter(|&n2| n2 != n1)
                    .map(|n2| [ (false, n1, x, y), (false, n2, x, y), ])
            ))?;
    }

    // L (gauche)
    write!(output, "c L (gauche)\n")?;
    for (x, n) in iproduct!(1..=9, 1..=9) {
        nb_clauses += 1;
        write!(output, "{}", Clause::from_iter((1..=9).map(|y| (true, n, x, y))))?;
    }

    // L (droit)
    write!(output, "c L (droite)\n")?;
    for (x, n, y0, y) in iproduct!(1..=9, 1..=9, 1..=9, 1..=9) {
        if y == y0 { continue; }
        nb_clauses += 1;
        write!(output, "{}", Clause::from( [(false, n, x, y0), (false, n, x, y)] ))?;
    }

    // C (gauche)
    write!(output, "c C (gauche)\n")?;
    for (y, n) in iproduct!(1..=9, 1..=9) {
        nb_clauses += 1;
        write!(output, "{}", Clause::from_iter((1..=9).map(|x| (true, n, x, y))))?;
    }

    // C (droit)
    write!(output, "c C (droite)\n")?;
    for (y, n, x0, x) in iproduct!(1..=9, 1..=9, 1..=9, 1..=9) {
        if x == x0 { continue; }
        nb_clauses += 1;
        write!(output, "{}", Clause::from( [(false, n, x0, y), (false, n, x, y)] ))?;
    }

    /* Fait la même chose que RJ(gauche), donc on en a pas besoin
    // R1 (gauche)
    write!(output, "c R1 (gauche)\n")?;
    for (i, j, x, y) in iproduct!(0..=2, 0..=2, 1..=3, 1..=3) {
        nb_clauses += 1;
        write!(output, "{}", Clause::from_iter((1..=9)
                    .map(|n| (true, n, 3*i + x, 3*j + y))))?;
    }
    */

    // R1 (droit)
    write!(output, "c R1 (droite)\n")?;
    for (i, j, x0, y0, n) in iproduct!(0..=2, 0..=2, 1..=3, 1..=3, 1..=9) {
        nb_clauses += 1;
        write!(output, "{}", Clause::from_iter(iproduct!(1..=3, 1..=3)
                    .filter(|&(x, y)| (x, y) != (x0, y0))
                    .map(|(x, y)| [
                         (false, n, x0 + 3*i, y0 + 3*j),
                         (false, n, x + 3*i, y + 3*j)
                    ]) ))?;
    }

    // R2 (gauche)
    write!(output, "c R2 (gauche)\n")?;
    for (n, i, j) in iproduct!(1..=9, 0..=2, 0..=2) {
        nb_clauses += 1;
        write!(output, "{}", Clause::from_iter(iproduct!(1..=3, 1..=3)
                    .map(|(x, y)| (true, n, 3 * i + x, 3 * j + y))))?;
    }

    // R2 (droit)
    write!(output, "c R2 (droite)\n")?;
    for (p, i, j, x, y) in iproduct!(1..=9, 0..=2, 0..=2, 1..=3, 1..=3) {
        nb_clauses += 1;
        write!(output, "{}", Clause::from_iter((1..=9)
                    .filter(|&n| n != p)
                    .map(|n| [
                         (false, p, x + 3*i, y + 3*j),
                         (false, n, x + 3*i, y + 3*j)
                    ]) ))?;
    }

    if let Output::OutFile(out_file) = &mut output {
        out_file.seek(SeekFrom::Start(10))?;
    }
    write!(output, "{nb_clauses}")?;

    output.flush().unwrap();
    Ok(())
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
}
