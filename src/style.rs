use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Style<'a> {
    pub style: &'a str, 
    pub value: &'a str,
}

pub trait Display {
    fn display(&self) -> String;
}

impl <'a> Display for Vec<Style<'a>> {
    fn display(&self) -> String {
        let concat_strings: String = self.iter().map(|style| style.to_string()).collect();
        concat_strings
    }
}


impl <'a> fmt::Display for Style <'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{};", self.style, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_svg_compliant_string() {
        let style = Style { style: "fill", value: "000" };
        assert_eq!(style.to_string(), "fill:000;");
    }

    #[test]
    fn multiple_svg_strings() {
        let stylelists = vec![
            Style { style: "fill", value: "000" },
            Style { style: "fill", value: "000" },
        ];
        assert_eq!(stylelists.display(), "fill:000;fill:000;");
    }
}
