use std::fmt;

use crate::style::*;

pub struct Svg <'a>{
    styles: Vec<Style<'a>>
}

impl<'a> fmt::Display for Svg<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><svg>\n<defs/><g></g><inkscape:clipboard style=\"{}\" />\n</svg>",
               self.styles.display())
    }
}

impl <'a> Svg <'a>{
    pub fn new(styles: Vec<Style<'a>>) -> Self {
        Self { styles }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_style() {
        let my_style = Style { style: "fill", value: "#000" };
        assert_eq!(my_style.to_string(), "fill:#000;");
    }
    #[test]
    fn iterates() {
        let my_styles: Vec<Style<'_>> = vec![
            Style { style: "fill", value: "#000" },
            Style { style: "stroke", value: "#000" },
            Style { style: "stroke-width", value: "0.7" },
        ];
        assert_eq!(
            my_styles.display(),
           "fill:#000;stroke:#000;stroke-width:0.7;"
       );
    }
    #[test]
    fn correct_output() {
        let styles = vec![ Style { style: "fill", value: "#000" } ];
        let my_svg = Svg {styles};

        assert_eq!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><svg>\n<defs/><g></g><inkscape:clipboard style=\"fill:#000;\" />\n</svg>",
            my_svg.to_string()
        );
    }
}
