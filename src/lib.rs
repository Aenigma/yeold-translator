use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use arp_tokenizer::ArpToken;
use rand::seq::IteratorRandom;
use rand::{distributions::Standard, thread_rng, Rng};
use serde::{Deserialize, Serialize};

mod arp_tokenizer;

pub const AUTORP: &str = include_str!("../resources/Autorp.txt");

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
#[serde(rename = "autorp.txt")]
pub struct AutoRP {
    prepended_words: HashMap<String, String>,
    appended_words: HashMap<String, String>,
    word_replacements: HashMap<String, Vec<WordReplacement>>,
}

fn default_one() -> u32 {
    1
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
struct WordReplacement {
    /// Previous word must match
    #[serde(default)]
    prev: HashSet<String>,

    /// Word this must match
    #[serde(default)]
    word: HashSet<String>,

    /// Plural word this must match
    #[serde(default)]
    word_plural: HashSet<String>,

    /// The chance in which this replacement is applied
    #[serde(default = "default_one")]
    chance: u32,

    /// A replacement word
    #[serde(default)]
    replacement: HashSet<String>,

    /// A replacement for the plural
    #[serde(default)]
    replacement_plural: HashSet<String>,

    #[serde(default = "default_one")]
    prepend_count: u32,

    #[serde(default)]
    replacement_prepend: HashSet<String>,
}

#[derive(Debug)]
struct MatchResult<'a> {
    _matcher: &'a WordReplacement,
    kind: MatchKind<'a>,
    replacement: String,
}

/// A buffer which contains the previous and current tokens as well as their
/// translation. Used internally to streamline some of the parser logic.
#[derive(Debug, Default, PartialEq, Eq)]
struct AutoRPParserCtx<'a, 'b> {
    pub prev: &'a str,
    pub prev_translated: Cow<'b, str>,
    pub current: &'a str,
    pub current_translated: Cow<'b, str>,
}

impl<'a, 'b> AutoRPParserCtx<'a, 'b> {
    /// Updates the context by shifting the current to previous and then
    /// setting the new values to `current`.
    fn update(&mut self, next: &'a str, next_translated: Cow<'b, str>) {
        self.prev = self.current;
        self.prev_translated = self.current_translated.clone();
        self.current = next;
        self.current_translated = next_translated;
    }
}

impl AutoRP {
    fn match_on_nodes<'a, R>(
        &'a self,
        prev: &'a str,
        word: &'a str,
        rng: &mut R,
    ) -> Option<MatchResult<'a>>
    where
        R: Rng + Sized,
    {
        self.word_replacements["1"]
            .iter()
            .flat_map(|n| {
                let res: Option<(MatchKind<'_>, String)> = n.translate(prev, word, rng);
                match res {
                    Some((kind, replacement)) => match n.is_chance(rng) {
                        true => Some(MatchResult {
                            _matcher: n,
                            kind,
                            replacement,
                        }),
                        false => None,
                    },
                    None => None,
                }
            })
            .next()
    }

    pub fn translate(&self, input: &str) -> String {
        let mut rng = thread_rng();
        let mut buf = String::with_capacity(1024);
        let (_, tokens) = arp_tokenizer::parse(input).unwrap();
        let mut it = tokens.iter();

        let mut ctx: AutoRPParserCtx = Default::default();
        let mut wsbuf: Vec<&str> = Vec::with_capacity(64);

        while let Some(token) = it.next() {
            let current = match token {
                ArpToken::Ws(s) => {
                    wsbuf.push(s);
                    continue;
                }
                ArpToken::Word(s) | ArpToken::Template(s) => s,
            };

            match self.match_on_nodes(ctx.current, current, &mut rng) {
                None => {
                    ctx.update(&current, Cow::from(*current));

                    buf.push_str(&ctx.prev_translated);
                    buf.push_str(&wsbuf.join(""));
                    wsbuf.clear();
                }
                Some(mr) => {
                    ctx.update(&current, mr.replacement.into());
                    if matches!(mr.kind, MatchKind::Previous(..)) {
                        wsbuf.pop();
                        buf.push_str(&wsbuf.join(""));
                        wsbuf.clear();
                        buf.push_str(&ctx.current_translated);
                        ctx = Default::default();
                        continue;
                    }

                    buf.push_str(&ctx.prev_translated);

                    buf.push_str(&wsbuf.join(""));
                    wsbuf.clear();
                }
            };
        }
        buf.push_str(&wsbuf.join(""));
        buf.push_str(&ctx.current_translated);
        buf
    }
}

impl WordReplacement {
    fn is_chance(&self, rng: &mut impl Rng) -> bool {
        if self.chance == 1 {
            return true;
        }
        let rand: f32 = rng.sample(Standard);
        let chance = 1. / self.chance as f32;

        chance < rand
    }
}

trait SetChoose<T> {
    fn choose<R>(&self, rng: &mut R) -> &T
    where
        R: Rng + Sized;
}

impl<T> SetChoose<T> for HashSet<T> {
    fn choose<R>(&self, rng: &mut R) -> &T
    where
        R: Rng + Sized,
    {
        self.iter().choose(rng).unwrap()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum MatchKind<'a> {
    // advance the pointer additionally
    Previous(&'a str, &'a str),
    // consume normally
    Word(&'a str),
    // plural match
    Plural(&'a str),
}

impl WordReplacement {
    fn matches<'a>(&self, current: &'a str, next: &'a str) -> Option<MatchKind<'a>> {
        if !self.prev.is_empty() {
            if self.prev.contains(current) && self.word.contains(next) {
                return Some(MatchKind::Previous(current, next));
            }
            return None;
        }
        if self.word.contains(next) {
            return Some(MatchKind::Word(next));
        }
        if self.word_plural.contains(next) {
            return Some(MatchKind::Plural(next));
        }
        return None;
    }

    fn translate<'a, R>(
        &self,
        current: &'a str,
        next: &'a str,
        rng: &mut R,
    ) -> Option<(MatchKind<'a>, String)>
    where
        R: Rng + Sized,
    {
        match self.matches(current, next) {
            None => return None,
            Some(kind) => {
                let replacement = match kind {
                    MatchKind::Previous(_, _) | MatchKind::Word(_) => self.replacement.choose(rng),
                    MatchKind::Plural(_) => self.replacement_plural.choose(rng),
                };

                Some((kind, self.prepend(rng) + replacement))
            }
        }
    }

    fn prepend(&self, rng: &mut impl Rng) -> String {
        if self.replacement_prepend.is_empty() {
            return String::default();
        }
        let mut res: Vec<&str> = Vec::with_capacity(self.prepend_count as usize);
        for _ in 0..self.prepend_count {
            res.push(self.replacement_prepend.choose(rng));
        }
        res.join(", ").to_string() + " "
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn previous_works() {
        let wrp = WordReplacement {
            prev: HashSet::from(["foo".to_string()]),
            word: HashSet::from(["bar".to_string()]),
            replacement: HashSet::from(["foobar".to_string()]),
            ..Default::default()
        };

        assert!(matches!(
            wrp.matches("foo", "bar"),
            Some(MatchKind::Previous(_, _))
        ));
        assert!(matches!(wrp.matches("", "bar"), None));
        assert!(matches!(wrp.matches("foo", ""), None));
    }

    #[test]
    fn word_works() {
        let wrp = WordReplacement {
            word: HashSet::from(["foo".to_string()]),
            replacement: HashSet::from(["foobar".to_string()]),
            ..Default::default()
        };

        assert!(matches!(
            wrp.matches("", "foo"),
            Some(MatchKind::Word("foo"))
        ));
        assert!(matches!(wrp.matches("foo", ""), None));
        assert!(matches!(wrp.matches("foobar", ""), None));
        assert!(matches!(wrp.matches("", "foobar"), None));
    }

    #[test]
    fn full_example() {
        let autrp = AutoRP {
            word_replacements: {
                let mut hm: HashMap<_, _> = HashMap::new();
                let lists: Vec<WordReplacement> = vec![WordReplacement {
                    word: HashSet::from(["foo".to_string()]),
                    replacement: HashSet::from(["foobar".to_string()]),
                    chance: 1,
                    ..Default::default()
                }];
                hm.insert("1".to_string(), lists);
                hm
            },
            ..Default::default()
        };

        assert_eq!(autrp.translate("foo"), "foobar");
    }
}
