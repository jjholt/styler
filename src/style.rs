use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(from = "&str")]
pub struct Style<'a> {
    pub style: Option<&'a str>, 
    pub value: Option<&'a str>,
}

impl<'a> From<&'a str> for Style<'a> {
    fn from(value: &'a str) -> Self {
        let mut keys = value.split(':');
        let mut style = keys.next();
        if let Some("") = style { style = None };
        Style {
            style,
            value: keys.next()
        } 
    }
}

impl<'a> From<Style<'a>> for String {
    fn from(value: Style<'a>) -> Self {
        value.to_string()
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
#[serde(from = "&str")]
pub struct StyleList<'a> (
    #[serde(borrow)]
    pub Vec<Style<'a>> 
);

impl<'a> StyleList<'a> {
    pub fn missing_param(&self) -> Option<&Style<'_>> {
        self.iter().find(|&f| f.missing_param())
    }
}

impl<'a> From<StyleList<'a>> for String {
    fn from(value: StyleList<'a>) -> Self {
        value.to_string()
    }
}


impl<'a> std::ops::Deref for StyleList<'a> {
    type Target = [Style<'a>];

    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}

impl<'a> Style<'a> {
    pub fn missing_param(&self) -> bool {
        self.style.is_none() || self.value.is_none()
    }
    pub fn set(&self, param: &'a Style) -> Style<'a> {
        Style {
            style: self.style.or(param.style),
            value: self.value.or(param.value),
        }
    }
}

impl<'a> From<&'a str> for StyleList<'a> {
    fn from(value: &'a str) -> Self {
        let styles: Vec<&str> = value
            .split(';')
            .filter(|&c| !c.is_empty())
            .collect();
        let styles: Vec<Style> = styles
            .iter()
            .map(|&f| f.into())
            .collect();
        StyleList(styles)
    }
}

impl<'a> fmt::Display for StyleList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let my_str: String = self.0.iter().map(|f| f.to_string()).collect();
        f.write_str(&my_str)
    }
}

impl <'a> fmt::Display for Style <'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{};", self.style.unwrap_or(""), self.value.unwrap_or(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const STYLE: Style = Style { style: Some("fill"), value: Some("000") };

    #[test]
    fn single_svg_compliant_string() {
        assert_eq!(STYLE.to_string(), "fill:000;");
    }

    #[test]
    fn multiple_svg_strings() {
        let styles = vec![ STYLE, STYLE ];
        let stylelist = StyleList(styles);
        assert_eq!(stylelist.to_string(), "fill:000;fill:000;");
        assert_eq!(stylelist, "fill:000;fill:000;".into());
    }
    #[test]
    fn serialises() {
        let result = serde_yaml::to_string(&STYLE).unwrap();
        println!("{}", result);
    }
    

    #[test]
    fn set() {
        let style = STYLE.clone();
        assert_eq!(
            style.set(&"fill".into()),
            Style { style: Some("fill"), value: Some("000") }
        );

        assert_eq!(
            style.set(&"fill".into()).set(&"paint".into()),
            Style { style: Some("fill"), value: Some("000") }
        );
    }
}
