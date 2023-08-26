use std::fmt;

use crate::style::*;

pub struct Svg <'a>{
    style: &'a StyleList<'a>
}

impl<'a> fmt::Display for Svg<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><svg>
               <defs id=\"defs2\">
               <marker
                   style=\"overflow:visible\"
                   id=\"ConcaveTriangle\"
                   inkscape:stockid=\"Concave triangle arrow\"
                   markerWidth=\"1\"
                   markerHeight=\"1\"
                   viewBox=\"0 0 1 1\"
                   preserveAspectRatio=\"xMidYMid\">
                   <path
                       transform=\"scale(0.7)\"
                       d=\"M -2,-4 9,0 -2,4 c 2,-2.33 2,-5.66 0,-8 z\"
                       style=\"fill:context-stroke;fill-rule:evenodd;stroke:none\"
                   id=\"path7\" />
               </marker>
               </defs>
               <g></g><inkscape:clipboard style=\"{}\" /></svg>",
               self.style)
    }
}

impl <'a> Svg <'a>{
    pub fn new(style: &'a StyleList) -> Self {
        Self { style }
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
}
