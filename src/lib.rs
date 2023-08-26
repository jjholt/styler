mod svg;
pub mod style;
pub mod keybind;

pub use crate::svg::Svg;

#[cfg(test)]
mod tests {
    use super::keybind::{
        Keybind,
        Keybinds,
        Action::{Style, Rebind}
    };

    const TEST_STYLE: &str = "key: a\nstyle: fill:000";
    const TEST_REBIND: &str = "key: b\nrebind_to: q\n";
    const TEST_COMBINED: &str = "- key: a\n  style: fill:000\n- key: b\n  rebind_to: q";

    #[test]
    fn apply_style_from_yaml() {
        let keybind = Keybind {
            key: "a",
            action: Style{style: "fill:000".into()}
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
            Keybind {key: "a", action: Style {style: "fill:000".into()}},
            Keybind {key: "b", action: Rebind {rebind_to: "q"}},
        ];
        let keybind_from_yaml: Vec<Keybind> = serde_yaml::from_str(TEST_COMBINED).unwrap();
        assert_eq!(keybinds, keybind_from_yaml);
    }
    #[test]
    fn reads_config_file() {
        let s = &std::fs::read_to_string("config.yaml").unwrap();
        println!("{}", s);
        let keybinds: Keybinds = serde_yaml::from_str(s).unwrap();
        println!("{:#?}", keybinds);
    }
}
