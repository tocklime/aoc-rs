use std::fmt;

use super::compmem::CompMem;
use super::computer::Computer;
use super::enums::*;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Arg<MemType>(MemType, ParameterMode);

impl<MemType> Arg<MemType>
where
    MemType: CompMem,
{
    pub fn new(value: MemType, mode: ParameterMode) -> Self {
        Self(value, mode)
    }
    pub fn get(self, c: &Computer<MemType>) -> MemType {
        match self {
            Self(i, ParameterMode::Immediate) => i,
            Self(i, ParameterMode::Position) => c.abs_load(i.as_isize()),
            Self(i, ParameterMode::Relative) => c.rel_load(i.as_isize()),
        }
    }
    pub fn ptr(self, c: &Computer<MemType>) -> isize {
        match self {
            Self(_, ParameterMode::Immediate) => panic!("Write instruction in immediate mode."),
            Self(i, ParameterMode::Position) => i.as_isize(),
            Self(i, ParameterMode::Relative) => c.rel_offset(i.as_isize()),
        }
    }
}
impl<MemType> fmt::Display for Arg<MemType>
where
    MemType: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.1 {
            ParameterMode::Immediate => write!(f, " {: <4}", self.0),
            ParameterMode::Position => write!(f, "@{: <4}", self.0),
            ParameterMode::Relative => write!(f, "R{: <4}", self.0),
        }
    }
}
