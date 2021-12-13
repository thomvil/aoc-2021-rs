use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct SubmarineCoordinate {
    pub(super) depth:      usize,
    pub(super) horizontal: usize,
}

impl SubmarineCoordinate {
    fn move_forward(mut self, amount: usize) -> Self {
        self.horizontal += amount;
        self
    }

    fn move_up(mut self, amount: usize) -> Self {
        self.depth = self.depth.saturating_sub(amount);
        self
    }

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
