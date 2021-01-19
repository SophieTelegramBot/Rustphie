use std::str::FromStr;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
/// Type used in optional command arg
///
/// # Description
/// Deref / DerefMut allows you to access methods of wrapped `Option`. learn more [Deref]
///
/// [Deref]: std::ops::Deref
pub struct OptionArg<Type: FromStr>(Option<Type>);

impl<Type: FromStr> Deref for OptionArg<Type> {
    type Target = Option<Type>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Type: FromStr> DerefMut for OptionArg<Type> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<Type> FromStr for OptionArg<Type>
where
    Type: FromStr
{
    type Err = Type::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(OptionArg(None))
        } else {
            let converted = Type::from_str(s)?;
            Ok(OptionArg(Some(converted)))
        }
    }
}
