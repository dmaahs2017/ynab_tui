pub mod account;
pub mod budget;
pub mod category_group;
pub mod enums;
pub mod transaction;
pub mod category;

pub use category::*;
pub use account::*;
pub use budget::*;
pub use category_group::*;
pub use enums::*;
pub use transaction::*;

use sqlite::{Result, Statement};

use sqlite::Value;

pub trait Insertable {
    fn query(&self) -> &'static str;
}

pub trait Updateable {
    fn query(&self) -> &'static str;
}

pub trait IdSelectable {
    fn query() -> &'static str;
}

pub trait AllSelectable {
    fn query() -> &'static str;
}

#[macro_export]
macro_rules! impl_all_selectable {
    ($a:ty,$query_file:literal) => {
        impl crate::data_layer::models::AllSelectable for $a {
            fn query() -> &'static str {
                include_str!($query_file)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_id_selectable {
    ($a:ty,$query_file:literal) => {
        impl crate::data_layer::models::IdSelectable for $a {
            fn query() -> &'static str {
                include_str!($query_file)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_insertable {
    ($a:ty,$query_file:literal) => {
        impl crate::data_layer::models::Insertable for $a {
            fn query(&self) -> &'static str {
                include_str!($query_file)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_updateable {
    ($a:ty,$query_file:literal) => {
        impl crate::data_layer::models::Updateable for $a {
            fn query(&self) -> &'static str {
                include_str!($query_file)
            }
        }
    };
}

pub trait BindToStatement {
    fn bind(self, s: &mut Statement) -> Result<()>;
}

pub trait ReadFromStatement
where
    Self: Sized,
{
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

impl IntoValue for AccountTypeString {
    fn into_value(self) -> Value {
        // TODO: This is hacky as shit lmao
        Value::String(
            serde_json::ser::to_string(&self)
                .unwrap()
                .trim_matches('"')
                .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_into_value_works_with_serde() {
        let x = AccountTypeString::Checking;
        assert_eq!(x.into_value(), Value::String("checking".to_string()));
    }
}
