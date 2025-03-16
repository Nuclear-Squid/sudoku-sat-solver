use std::fs::{ self, File };
use std::io::{ self, Write, BufWriter, Seek, SeekFrom };
use std::str::FromStr;
use itertools::iproduct;

// import the contents of `lib.rs`
use sudoku_dimacs::*;

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

    // U (gauche)
    write!(output, "c U (gauche)\n")?;
    for (x, y) in iproduct!(1..=9, 1..=9) {
        nb_clauses += 1;
        write!(output, "{}", Clause::from_iter((1..=9).map(|n| (true, n, x, y))))?;
    }

    // U (droit)
    write!(output, "c U (droite)\n")?;
    for (n1, x, y) in iproduct!(1..=9, 1..=9, 1..=9) {
        nb_clauses += 1;
        write!(output, "{}", Clause::from_iter((1..=9)
                    .filter(|n2| *n2 != n1)
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

    // R1 (gauche)
    write!(output, "c R1 (gauche)\n")?;
    for (i, j, x, y) in iproduct!(0..=2, 0..=2, 1..=3, 1..=3) {
        nb_clauses += 1;
        write!(output, "{}", Clause::from_iter((1..=9)
                    .map(|n| (true, n, 3*i + x, 3*j + y))))?;
    }

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
