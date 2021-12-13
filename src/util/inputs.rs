use crate::prelude::*;

pub(crate) fn input_str(day_nb: u8) -> Aoc2021Result<String> {
    read_to_string(format!("./inputs/day{:02}.txt", day_nb))
        .map_err(|_| Aoc2021Error::input_file_missing(day_nb))
}
