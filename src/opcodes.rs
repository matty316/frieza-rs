pub(crate) enum OpCode {
    Return,
    Int,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Float,
}

pub(crate) type Value = i32;

impl TryInto<OpCode> for u8 {
    type Error = ();

    fn try_into(self) -> Result<OpCode, Self::Error> {
        match self {
            0 => Ok(OpCode::Return),
            1 => Ok(OpCode::Int),
            2 => Ok(OpCode::Negate),
            3 => Ok(OpCode::Add),
            4 => Ok(OpCode::Subtract),
            5 => Ok(OpCode::Multiply),
            6 => Ok(OpCode::Divide),
            7 => Ok(OpCode::Float),
            _ => Err(())
        }
    }
}