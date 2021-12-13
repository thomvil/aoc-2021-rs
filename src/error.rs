#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Aoc2021Error {
    Input(Aoc2021InputError),
}

impl Aoc2021Error {
    pub(crate) fn input_file_missing(day_nb: u8) -> Self {
        Self::Input(Aoc2021InputError::FileMissing(day_nb))
    }

    pub(crate) fn input_parsing(day_nb: u8) -> Self {
        Self::Input(Aoc2021InputError::Parsing(day_nb))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Aoc2021InputError {
    Parsing(u8),
    NotSufficient(u8),
    FileMissing(u8),
}

pub(crate) type Aoc2021Result<T> = Result<T, Aoc2021Error>;
