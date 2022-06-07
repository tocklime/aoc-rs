pub trait AnswerType {
    type Output;
    fn to_option(self) -> Option<Self::Output>;
}

impl<T> AnswerType for Option<T> {
    type Output = T;

    fn to_option(self) -> Option<Self::Output> {
        self
    }
}
impl<T1, T2> AnswerType for (T1, T2) {
    type Output = Self;
    fn to_option(self) -> Option<Self::Output> {
        Some(self)
    }
}

macro_rules! impl_answer_type {
    ($($t:ident) *) => {
        $(
            impl AnswerType for $t {
                type Output = Self;
                fn to_option(self) -> Option<Self::Output> {
                    Some(self)
                }
            }
        )*
    };
}

impl_answer_type!(usize isize u128 i128 u64 i64 u32 i32 u16 i16 u8 i8 String);
