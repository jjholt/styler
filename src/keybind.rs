use std::fmt;
use serde::{Deserialize, Serialize};

use crate::style::StyleList;

#[derive(Deserialize, Debug, PartialEq, Serialize)]
pub struct Keybind<'a> {
    pub key: &'a str, 
    #[serde(flatten)]
    pub action: Action<'a>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Action<'a> {
    Style{style: StyleList<'a>},
    Rebind{rebind_to: &'a str},
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Keybinds<'a> (
    #[serde(borrow)]
    Vec<Keybind<'a>>
);

impl<'a> Keybind<'a> {
    pub fn action(&self) -> &Action<'_> {
        &self.action
    } 
    pub fn key(&self) -> &str {
        self.key
    }
}

impl<'a> fmt::Display for Action<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Style {style} => write!(f, "{}", style),
            Action::Rebind {rebind_to: key} => write!(f, "{}", key),
        }
    }
}


impl<'a> Keybinds<'a> {
    pub fn get_bind_for(&self, letter: &str) -> Option<&Keybind<'_>> {
        self.0.iter().find(|c| c.key == letter)
    }
}

impl <'a> fmt::Display for Keybinds<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|k| {
            write!(f, "{} => {}\n", k.key, k.action) 
        })
    }
}


