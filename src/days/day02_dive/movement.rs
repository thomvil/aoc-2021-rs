use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct SubmarineDiveMovement {
    pub(super) direction: SubmarineDiveDirection,
    pub(super) amount:    u8,
}

// TODO: collapse with macro
impl TryFrom<&str> for SubmarineDiveMovement {
    type Error = Aoc2021Error;

    fn try_from(s: &str) -> Aoc2021Result<Self> {
        let (direction_str, amount_str) = s
            .split_once(' ')
            .ok_or_else(|| Aoc2021Error::input_parsing(2))?;
        let direction = SubmarineDiveDirection::try_from(direction_str)?;
        let amount = amount_str
            .parse::<u8>()
            .map_err(|_| Aoc2021Error::input_parsing(2))?;
        Ok(Self { direction, amount })
    }
}

impl TryFrom<String> for SubmarineDiveMovement {
    type Error = Aoc2021Error;

    fn try_from(s: String) -> Aoc2021Result<Self> {
        Self::try_from(s.as_str())
    }
}

impl FromStr for SubmarineDiveMovement {
    type Err = Aoc2021Error;

    fn from_str(s: &str) -> Aoc2021Result<Self> {
        Self::try_from(s)
    }
}
