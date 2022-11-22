mod budget;
pub use budget::*;
mod transaction;
pub use transaction::*;
mod account;
pub use account::*;

use sqlite::{Result, Statement};

use sqlite::Value;

pub trait BindToStatement {
    fn bind(self, s: &mut Statement) -> Result<()>;
}

pub trait ReadFromStatement where Self: Sized {
    fn read(s: &mut Statement) -> Result<Self>;
}

trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for Option<String> {
    fn into_value(self) -> Value {
        match self {
            None => Value::Null,
            Some(s) => Value::String(s),
        }
    }
}

impl IntoValue for bool {
    fn into_value(self) -> Value {
        if self {
            Value::Integer(1)
        } else {
            Value::Integer(0)
        }
    }
}
