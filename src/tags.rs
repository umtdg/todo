use regex::{Captures, Regex};
use std::collections::HashSet;
use std::ops::Index;
use colored::Colorize;
use crate::utils::remove_first;

pub trait Tags {
    fn extract_tags(self, extract_exact: Option<bool>) -> HashSet<String>;

    fn highlight_tags(self) -> String;
}

impl<T: Into<String>> Tags for T {
    fn extract_tags(self, extract_exact: Option<bool>) -> HashSet<String> {
        if !extract_exact.unwrap_or(false) {
            self.into().split_whitespace()
                .filter(|&word| word.starts_with("#"))
                .map(remove_first)
                .map(str::to_lowercase)
                .collect()
        } else {
            self.into().split_whitespace()
                .filter(|&word| word.starts_with("#"))
                .map(&str::to_owned).collect()
        }
    }

    fn highlight_tags(self) -> String {
        let s: String = self.into();
        let text: &str = s.as_str();
        let re: Regex = Regex::new(
            &(format!(
                "({})",
                itertools::join(
                    text.extract_tags(Some(true)),
                    "|"
                )
            ))[..]
        ).unwrap();

        re.replace_all(text, |cap: &Captures| {
            cap.index(0).yellow().bold().to_string()
        }).into_owned()
    }
}
