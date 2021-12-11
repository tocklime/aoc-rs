use num_enum::TryFromPrimitive;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComputerState {
    RUNNING,
    HALTED,
}

#[derive(Clone, Copy, TryFromPrimitive, PartialEq, Eq, Debug)]
#[repr(isize)]
pub enum ParameterMode {
    POSITION = 0,
    IMMEDIATE = 1,
    RELATIVE = 2,
}
