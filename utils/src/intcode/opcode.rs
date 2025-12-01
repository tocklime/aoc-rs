use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(isize)]
pub enum OpCode {
    Add = 1,
    Mult = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    MoveRelativeBase = 9,
    Halt = 99,
}
impl TryFrom<isize> for OpCode {
    type Error = isize;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Mult),
            3 => Ok(OpCode::Input),
            4 => Ok(OpCode::Output),
            5 => Ok(OpCode::JumpIfTrue),
            6 => Ok(OpCode::JumpIfFalse),
            7 => Ok(OpCode::LessThan),
            8 => Ok(OpCode::Equals),
            9 => Ok(OpCode::MoveRelativeBase),
            99 => Ok(OpCode::Halt),
            _ => Err(value)
        }
    }
}
impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Add => "Add",
            Self::Mult => "Mult",
            Self::Input => "Input",
            Self::Output => "Output",
            Self::JumpIfTrue => "Jump If True",
            Self::JumpIfFalse => "Jump If False",
            Self::LessThan => "Less Than",
            Self::Equals => "Equals",
            Self::MoveRelativeBase => "Move Relative Base",
            Self::Halt => "Halt",
        };
        write!(f, "{s: <20}")
    }
}

impl OpCode {
    pub fn arg_count(self) -> usize {
        match self {
            Self::Input | Self::Output | Self::MoveRelativeBase => 1,
            Self::JumpIfTrue | Self::JumpIfFalse => 2,
            Self::Add | Self::Mult | Self::LessThan | Self::Equals => 3,
            Self::Halt => 0,
        }
    }
}
