use std::borrow::Cow;
use std::collections::HashMap;

use rand::thread_rng;

use crate::autorp::WordReplacement;

pub trait TemplateMap {
    fn get<'a>(&self, input: &'a str) -> Option<Cow<str>>;
}

impl TemplateMap for Vec<WordReplacement> {
    fn get<'a>(&self, input: &'a str) -> Option<Cow<str>> {
        let mut rng = thread_rng();
        for wr in self {
            if let Some(s) = wr.simple_get(input, &mut rng) {
                return Some(Cow::Borrowed(s));
            }
        }
        None
    }
}

/// Trait representing a map that can be used for template substitution.
impl TemplateMap for HashMap<String, String> {
    /// Retrieves the value corresponding to the given key from the map.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the key to look up.
    ///
    /// # Returns
    ///
    /// A `Cow<str>` that is either borrowed from the input or the value from the map.
    fn get<'a>(&self, input: &'a str) -> Option<Cow<str>> {
        self.get(input).map(|s| Cow::Borrowed(s.as_str()))
    }
}

/// Parses a template string starting with '&' and returns the template and the rest of the string.
///
/// # Arguments
///
/// * `input` - A string slice that holds the template string.
///
/// # Returns
///
/// An `Option` containing a tuple with the template and the rest of the string if parsing is successful, otherwise `None`.
fn parse_template(input: &str) -> Option<(&str, &str)> {
    let mut it = input.char_indices();
    let mut end = 0;

    if it.next()?.1 != '&' {
        return None;
    }

    while let Some((i, c)) = it.next() {
        if !c.is_alphanumeric() {
            break;
        }
        end = i;
    }

    match end {
        0 | 1 => return None,
        _ if end == input.len() - 1 => Some((input, "")),
        _ => Some((&input[0..end + 1], &input[end + 1..])),
    }
}

/// Evaluates a template string by replacing placeholders with values from the provided map.
///
/// # Arguments
///
/// * `input` - A string slice that holds the template string.
/// * `map` - A reference to an object implementing the `TemplateMap` trait.
///
/// # Returns
///
/// A `Cow<str>` that contains the evaluated string with placeholders replaced by corresponding values from the map.
pub fn template_evaluate<'a>(mut input: &'a str, submap: &impl TemplateMap) -> Cow<'a, str> {
    let mut res = match input.chars().position(|c| c == '&') {
        None => return Cow::Borrowed(input),
        Some(pos) => {
            let mut res = String::with_capacity(input.len() * 11 / 10);
            res.push_str(&input[..pos]);
            input = &input[pos..];
            res
        }
    };

    while let Some(pos) = input.chars().position(|c| c == '&') {
        res.push_str(&input[..pos]);
        input = &input[pos..];

        match {
            parse_template(input).and_then(|(template, rest)| {
                return submap.get(&template[1..]).map(|s| (s, rest));
            })
        } {
            Some((s, rest)) => {
                res.push_str(&s);
                input = rest;
            }
            None => {
                res.push_str(&input[..1]);
                input = &input[1..];
            }
        }
    }

    res.push_str(input);

    return Cow::Owned(res);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_template_evaluate_edge_cases() {
        let map = HashMap::new();

        assert_eq!(template_evaluate("", &map), Cow::Borrowed(""));
        assert_eq!(template_evaluate("&", &map), Cow::Borrowed("&"));
        assert_eq!(template_evaluate("&&", &map), Cow::Borrowed("&&"));
        assert_eq!(
            template_evaluate("Hello, &name!", &map),
            Cow::Borrowed("Hello, &name!")
        );
        assert_eq!(
            template_evaluate("Hello, &!", &map),
            Cow::Borrowed("Hello, &!")
        );
    }

    #[test]
    fn test_template_evaluate() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), "Alice".to_string());
        map.insert("lang".to_string(), "Rust".to_string());

        assert_eq!(
            template_evaluate("Hello, &name!", &map),
            Cow::Borrowed("Hello, Alice!")
        );
        assert_eq!(
            template_evaluate("Hello, &name! Welcome to &lang programming.", &map),
            Cow::Borrowed("Hello, Alice! Welcome to Rust programming.")
        );
        assert_eq!(
            template_evaluate("No templates here.", &map),
            Cow::Borrowed("No templates here.")
        );
    }
}
