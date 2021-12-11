use num_enum::TryFromPrimitive;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComputerState {
    Running,
    Halted,
}

#[derive(Clone, Copy, TryFromPrimitive, PartialEq, Eq, Debug)]
#[repr(isize)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}
