use std::collections::HashSet;

use crate::{encodings::Encoding, error::Error};
use aho_corasick::{AhoCorasick, MatchKind};

pub struct Converter {
    encoding: Encoding,
    forward_automaton: AhoCorasick,
    forward_replacements: Vec<&'static str>,

    reverse_automaton: AhoCorasick,
    reverse_replacements: Vec<&'static str>,
}

impl Converter {
    pub fn new(encoding: Encoding) -> Result<Self, Error> {
        let table = encoding.mapping_table();
        Self::from_table(encoding, table)
    }

    pub fn from_table(
        encoding: Encoding,
        table: &'static [(&'static str, &'static str)],
    ) -> Result<Self, Error> {
        let forward_patterns: Vec<&str> = table.iter().map(|(src, _)| *src).collect();
        let forward_replacements: Vec<&str> = table.iter().map(|(_, dst)| *dst).collect();

        let forward_automaton = AhoCorasick::builder()
            .match_kind(MatchKind::LeftmostLongest)
            .build(&forward_patterns)?;

        let mut seen_dst = HashSet::with_capacity(table.len());
        let mut reverse_patterns = Vec::with_capacity(table.len());
        let mut reverse_replacements = Vec::with_capacity(table.len());

        for &(src, dst) in table {
            if !dst.is_empty() && seen_dst.insert(dst) {
                reverse_patterns.push(dst);
                reverse_replacements.push(src);
            }
        }

        let reverse_automaton = AhoCorasick::builder()
            .match_kind(MatchKind::LeftmostLongest)
            .build(&reverse_patterns)?;

        Ok(Self {
            encoding,
            forward_automaton,
            forward_replacements,
            reverse_automaton,
            reverse_replacements,
        })
    }

    #[inline]
    pub fn encoding(&self) -> Encoding {
        self.encoding
    }

    pub fn to_unicode(&self, input: &str) -> String {
        let mut result = String::with_capacity(input.len() * 3);
        self.forward_automaton
            .replace_all_with(input, &mut result, |mat, _, dst| {
                dst.push_str(self.forward_replacements[mat.pattern().as_usize()]);
                true
            });
        result
    }

    pub fn to_legacy(&self, input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        self.reverse_automaton
            .replace_all_with(input, &mut result, |mat, _, dst| {
                dst.push_str(self.reverse_replacements[mat.pattern().as_usize()]);
                true
            });
        result
    }

    #[cfg(feature = "parallel")]
    pub fn convert_batch_parallel<T: AsRef<str> + Sync>(&self, texts: &[T]) -> Vec<String> {
        use rayon::prelude::*;
        texts
            .par_iter()
            .map(|t| self.to_unicode(t.as_ref()))
            .collect()
    }
}
