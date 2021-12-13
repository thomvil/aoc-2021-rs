use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct SubmarineCoordinate {
    pub(super) depth:      usize,
    pub(super) horizontal: usize,
}

impl SubmarineCoordinate {
    #[inline]
    fn move_forward(mut self, amount: usize) -> Self {
        self.horizontal += amount;
        self
    }

    #[inline]
    fn move_up(mut self, amount: usize) -> Self {
        self.depth = self.depth.saturating_sub(amount);
        self
    }

    #[inline]
    fn move_down(mut self, amount: usize) -> Self {
        self.depth += amount;
        self
    }

    pub(crate) fn make_move(self, movement: SubmarineDiveMovement) -> Self {
        use SubmarineDiveDirection::*;
        let amount = movement.amount as usize;
        match movement.direction {
            Up => self.move_up(amount),
            Down => self.move_down(amount),
            Forward => self.move_forward(amount),
        }
    }
}

// TODO: rename
pub(crate) trait CoordinateTrait {
    fn make_move(&mut self, movement: SubmarineDiveMovement);
    fn location_code(&self) -> usize;
}

impl Coordinate2 {
    #[inline]
    fn move_forward(mut self, amount: usize) -> Self {
        self.horizontal += amount;
        self.depth += self.aim * amount;
        self
    }

    #[inline]
    fn move_up(mut self, amount: usize) -> Self {
        self.aim = self.aim.saturating_sub(amount);
        self
    }

    #[inline]
    fn move_down(mut self, amount: usize) -> Self {
        self.aim += amount;
        self
    }
}

pub(crate) struct Coordinate2 {
    pub(super) depth:      usize,
    pub(super) horizontal: usize,
    pub(super) aim:        usize,
}

impl CoordinateTrait for Coordinate2 {
    fn make_move(&mut self, movement: SubmarineDiveMovement) {
        use SubmarineDiveDirection::*;
        let amount = movement.amount as usize;
        match movement.direction {
            Up => self.move_up(amount),
            Down => self.move_down(amount),
            Forward => self.move_forward(amount),
        }
    }

    fn location_code(&self) -> usize {
        self.depth * self.horizontal
    }
}
