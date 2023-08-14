mod svg;
pub mod style;
pub mod keybind;

pub use crate::svg::Svg;

#[cfg(test)]
mod tests {
    use super::style::*;
    use super::keybind::{
        Keybind,
        Action::{ApplyStyle, Rebind}
    };

    const TEST_STYLE: &str = "key: a\nstyle: fill\nvalue: 000\n";
    const TEST_REBIND: &str = "key: b\nrebind_to: q\n";
    const TEST_COMBINED: &str = "- key: a\n  style: fill\n  value: 000\n- key: b\n  rebind_to: q";

    #[test]
    fn apply_style_from_yaml() {
        let keybind = Keybind {
            key: "a",
            action: ApplyStyle( Style { style: "fill", value: "000"} )
        };
        let keybind_from_yaml: Keybind = serde_yaml::from_str(TEST_STYLE).unwrap();
        assert_eq!(keybind, keybind_from_yaml);
    }
    #[test]
    fn rebind_from_yaml() {
        let keybind = Keybind {key: "b", action: Rebind {rebind_to: "q"}};
        let keybind_from_yaml: Keybind = serde_yaml::from_str(TEST_REBIND).unwrap();
        assert_eq!(keybind, keybind_from_yaml);
    }
    #[test]
    fn combined_from_yaml() {
        let keybinds: Vec<Keybind> = vec![
            Keybind {key: "a", action: ApplyStyle( Style { style: "fill", value: "000"} )},
            Keybind {key: "b", action: Rebind {rebind_to: "q"}},
        ];
        // println!("{}", serde_yaml::to_string(&keybinds).unwrap());
        let keybind_from_yaml: Vec<Keybind> = serde_yaml::from_str(TEST_COMBINED).unwrap();
        assert_eq!(keybinds, keybind_from_yaml);
    }
}
