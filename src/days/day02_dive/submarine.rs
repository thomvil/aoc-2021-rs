use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct Submarine {
    location: SubmarineCoordinate,
}

impl Submarine {
    pub(crate) fn location_code(&self) -> usize {
        self.location.depth * self.location.horizontal
    }

    pub(crate) fn make_movements(
        &mut self,
        movements: impl Iterator<Item = SubmarineDiveMovement>,
    ) {
        for movement in movements {
            self.location.make_move(movement);
        }
    }

    pub(crate) fn make_movement(&mut self, movement: SubmarineDiveMovement) {
        self.location = self.location.make_move(movement);
    }

    pub(crate) fn read_instructions(mut self, movements_str: &str) -> Aoc2021Result<Self> {
        for movement_str in movements_str.lines() {
            self.make_movement(SubmarineDiveMovement::try_from(movement_str)?);
        }
        Ok(self)
    }
}
