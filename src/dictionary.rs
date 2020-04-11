use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub struct Dictionary {
    pub words: HashMap<usize, Vec<Vec<char>>>,
}

impl Dictionary {
    pub fn from_file(filename: String) -> io::Result<Dictionary> {
        let file = File::open(filename)?;
        let words: Vec<String> = io::BufReader::new(file)
            .lines()
            .map(|w| w.unwrap())
            .collect();

        Ok(Dictionary::from_vec(words))
    }

    pub fn from_vec(v: Vec<String>) -> Dictionary {
        let mut words = HashMap::new();
        for raw_word in v.iter() {
            let word: Vec<char> = raw_word.to_ascii_lowercase().chars().collect();

            words.entry(word.len()).or_insert_with(Vec::new).push(word);
        }

        Dictionary { words }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn simple_dict() -> Dictionary {
        Dictionary::from_vec(vec![
            // 3:
            String::from("foo"),    // 0
            String::from("BAR"),    // 1
            String::from("baz"),    // 2
            // 4:
            String::from("quad"),   // 0
            String::from("plex"),   // 1
            String::from("plan"),   // 2
        ])
    }

    #[test]
    fn test_dict() {
        let d = simple_dict();

        assert_eq!(vec![
            vec!['f', 'o', 'o'],
            vec!['b', 'a', 'r'],
            vec!['b', 'a', 'z'],
        ], *d.words.get(&3).unwrap());

        assert_eq!(vec![
            vec!['q', 'u', 'a', 'd'],
            vec!['p', 'l', 'e', 'x'],
            vec!['p', 'l', 'a', 'n'],
        ], *d.words.get(&4).unwrap());
    }
}