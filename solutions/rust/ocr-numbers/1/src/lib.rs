// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::str::Lines;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

/// A single row
type Row<'a> = &'a str;

/// A logical line, made up of 4 rows
type Line<'a> = [Row<'a>; 4];

/// Reads logical lines from a `&str` input.
struct LineReader<'a> {
    rows: Lines<'a>,
    read_rows: usize,
}

impl<'a> LineReader<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            rows: input.lines(),
            read_rows: 0,
        }
    }

    /// Reads a row, incrementing the row count if successful.
    fn read_row(&mut self) -> Option<Row<'a>> {
        self.rows.next().inspect(|_| self.read_rows += 1)
    }

    /// Reads the line (four rows). Returns `None` if there are no
    /// more lines.
    fn read_line(&mut self) -> Option<Result<Line<'a>, Error>> {
        // The first row might be `None`, then we return `None`
        let next = self.read_row()?;
        let mut out = [next; 4];
        for val in out.iter_mut().skip(1) {
            // If any of the following rows are `None`, this is an error
            let row = self.read_row();
            match row {
                Some(row) => *val = row,
                None => return Some(Err(Error::InvalidRowCount(self.read_rows))),
            }
        }
        Some(Ok(out))
    }
}

impl<'a> Iterator for LineReader<'a> {
    type Item = Result<CharReader<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_line().map(|x| x.map(CharReader::try_new)?)
    }
}

/// Reads logical chars from a logical line.
struct CharReader<'a> {
    line: Line<'a>,
    read_columns: usize,
}

impl<'a> CharReader<'a> {
    /// Tries to create a new reader, fails if not all rows of the `line` are of the
    /// same length or not divisible by 3.
    fn try_new(line: Line<'a>) -> Result<Self, Error> {
        // Line length must be divisible by 3
        if line[0].len() % 3 != 0 {
            return Err(Error::InvalidColumnCount(line[0].len()));
        }
        // All lines of a row must be of same length
        for row in &line[1..] {
            if row.len() != line[0].len() {
                return Err(Error::InvalidColumnCount(row.len()));
            }
        }
        Ok(Self {
            line,
            read_columns: 0,
        })
    }
}

impl<'a> Iterator for CharReader<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.read_columns >= self.line[0].len() {
            return None;
        }
        let window = self.read_columns..self.read_columns + 3;
        let out = [
            &self.line[0][window.clone()],
            &self.line[1][window.clone()],
            &self.line[2][window.clone()],
            &self.line[3][window],
        ];
        self.read_columns += 3;
        Some(out)
    }
}

/// Given a field of 4 rows of length 3, returns the recognized number if any.
fn decode(lines: &[&str]) -> Option<u8> {
    match lines {
        [" _ ", "| |", "|_|", "   "] => Some(0),
        ["   ", "  |", "  |", "   "] => Some(1),
        [" _ ", " _|", "|_ ", "   "] => Some(2),
        [" _ ", " _|", " _|", "   "] => Some(3),
        ["   ", "|_|", "  |", "   "] => Some(4),
        [" _ ", "|_ ", " _|", "   "] => Some(5),
        [" _ ", "|_ ", "|_|", "   "] => Some(6),
        [" _ ", "  |", "  |", "   "] => Some(7),
        [" _ ", "|_|", "|_|", "   "] => Some(8),
        [" _ ", "|_|", " _|", "   "] => Some(9),
        _ => None,
    }
}

pub fn convert(input: &str) -> Result<String, Error> {
    let mut out = String::new();
    for (i, line) in LineReader::new(input).into_iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        for character in line? {
            let value = decode(&character);
            let value = value.map(|v| v.to_string());
            if let Some(value) = value {
                out.push_str(&value);
            } else {
                out.push('?');
            }
        }
    }
    Ok(out)
}
