#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComputerState {
    Running,
    Halted,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(isize)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl TryFrom<isize> for ParameterMode {
    type Error = isize;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            2 => Ok(ParameterMode::Relative),
            _ => Err(value),
        }
    }
}
