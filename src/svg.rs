use std::fmt;

use crate::style::*;

pub struct Svg <'a>{
    style: StyleList<'a>
}

impl<'a> fmt::Display for Svg<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><svg>\n<defs/><g></g><inkscape:clipboard style=\"{}\" />\n</svg>",
               self.style)
    }
}

impl <'a> Svg <'a>{
    pub fn new(style: Vec<Style<'a>>) -> Self {
        Self { style: StyleList(style) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_style() {
        let my_style = Style { style: Some("fill"), value: Some("#000") };
        assert_eq!(my_style.to_string(), "fill:#000;");
        let styles: StyleList<'_> = "fill;".into();
        assert_eq!(styles[0], Style { style: Some("fill"), value: None })
    }
    #[test]
    fn iterates() {
        let my_styles = StyleList(vec![
            Style { style: Some("fill"), value: Some("#000") },
            Style { style: Some("stroke"), value: Some("#000") },
            Style { style: Some("stroke-width"), value: Some("0.7") },
        ]);
        assert_eq!(
            my_styles.to_string(),
           "fill:#000;stroke:#000;stroke-width:0.7;"
       );
        assert_eq!(
            my_styles,
           "fill:#000;stroke:#000;stroke-width:0.7;".into()
       );
    }
    #[test]
    fn correct_output() {
        let my_svg = Svg {style: "fill:#000".into()};

        assert_eq!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><svg>\n<defs/><g></g><inkscape:clipboard style=\"fill:#000;\" />\n</svg>",
            my_svg.to_string()
        );
    }
}
