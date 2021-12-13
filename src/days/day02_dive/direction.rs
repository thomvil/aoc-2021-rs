use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum SubmarineDiveDirection {
    Forward,
    Down,
    Up,
}

// TODO: collapse with macro
impl TryFrom<&str> for SubmarineDiveDirection {
    type Error = Aoc2021Error;

    fn try_from(s: &str) -> Aoc2021Result<Self> {
        match s {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(Aoc2021Error::input_parsing(2)),
        }
    }
}

impl TryFrom<String> for SubmarineDiveDirection {
    type Error = Aoc2021Error;

    fn try_from(s: String) -> Aoc2021Result<Self> {
        Self::try_from(s.as_str())
    }
}

impl FromStr for SubmarineDiveDirection {
    type Err = Aoc2021Error;

    fn from_str(s: &str) -> Aoc2021Result<Self> {
        Self::try_from(s)
    }
}
