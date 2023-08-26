use std::convert::TryFrom;
use std::fmt;
use serde::Serialize;
// use serde::Deserialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(try_from = "String")]
pub enum Modifier{
    Shift,
    Control,
    Alt
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Modifier::Shift => "shift",
            Modifier::Control => "ctrl",
            Modifier::Alt => "alt",
        };
        f.write_str(text)
    }
}
pub struct UnknownModifier;

impl fmt::Display for UnknownModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Unknown Modifier")
    }
}

impl TryFrom<String> for Modifier {
    type Error = UnknownModifier;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "shift" => Ok(Self::Shift),
            "ctrl" => Ok(Self::Control),
            "alt" => Ok(Self::Alt),
            _ => Err(UnknownModifier)
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(try_from = "String")]
pub struct Rebind <'a> {
    rebind_to: &'a str,
    #[serde(flatten)]
    modifiers: Option<&'a [Modifier]>
}

impl<'a> fmt::Display for Rebind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bind = self.modifiers
            .map(|f| f.iter()
                 .map(|c| c.to_string())
                 .collect::<Vec<String>>()
                 .join("+") + "+");
        write!(f, "{}{}", bind.unwrap_or("".to_string()), self.rebind_to)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Modifier::{Shift, Control, Alt};
    
    #[test]
    fn serialises() {
        let rebind = Rebind {
            rebind_to: "a",
            modifiers: Some(&[Shift, Control]),
        };
        println!("{}", rebind);
        let yaml = serde_yaml::to_string(&rebind);
        println!("{:#?}", yaml);
    }
    
}
