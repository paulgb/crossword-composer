use crate::dictionary::Dictionary;
use std::collections::HashMap;

static EMPTY_VEC: Vec<(usize, Vec<char>)> = Vec::new();
static EMPTY_VEC2: Vec<Vec<char>> = Vec::new();

#[derive(Clone)]
pub struct Index {
    pub known_letters: Vec<usize>, // Indices of letters known at the step when the index is used.
    pub unknown_letters: Vec<usize>, // Indices of letters *not* known at the step when the index is used.
    known_to_unknown: HashMap<Vec<char>, Vec<(usize, Vec<char>)>>, // Map of unknown to known letters and word indices.
}

impl Index {
    pub fn new(known_letters: Vec<usize>, word_length: usize, dict: &Dictionary) -> Index {
        let unknown_letters: Vec<usize> = (0..word_length)
            .filter(|j| !known_letters.contains(j))
            .collect();
        let mut known_to_unknown = HashMap::new();
        // TODO: unwrap may fail if dict is empty for this word length.
        let word_list = dict.words.get(&word_length).unwrap_or(&EMPTY_VEC2);

        for (i, w) in word_list.iter().enumerate() {
            let key: Vec<char> = known_letters.iter().map(|j| w[*j]).collect();
            let val: Vec<char> = unknown_letters.iter().map(|j| w[*j]).collect();

            known_to_unknown
                .entry(key)
                .or_insert_with(Vec::new)
                .push((i, val));
        }

        Index {
            known_letters,
            known_to_unknown,
            unknown_letters,
        }
    }

    pub fn get(&self, known: &Vec<char>) -> &Vec<(usize, Vec<char>)> {
        self.known_to_unknown.get(known).unwrap_or(&EMPTY_VEC)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dictionary::tests::simple_dict;

    #[test]
    fn test_no_known_letter_index() {
        let dict = simple_dict();
        let index = Index::new(vec![], 3, &dict);

        assert_eq!(
            vec![
                (0, vec!['f', 'o', 'o']),
                (1, vec!['b', 'a', 'r']),
                (2, vec!['b', 'a', 'z']),
            ],
            *index.get(&vec![])
        );
    }

    #[test]
    fn test_one_known_letter_index() {
        let dict = simple_dict();
        let index = Index::new(vec![1], 3, &dict);

        assert_eq!(vec![(0, vec!['f', 'o'])], *index.get(&vec!['o']));

        assert_eq!(
            vec![(1, vec!['b', 'r']), (2, vec!['b', 'z'])],
            *index.get(&vec!['a'])
        );
    }

    #[test]
    fn test_two_known_letter_index() {
        let dict = simple_dict();
        let index = Index::new(vec![0, 1], 4, &dict);

        assert_eq!(
            vec![(1, vec!['e', 'x']), (2, vec!['a', 'n'])],
            *index.get(&vec!['p', 'l'])
        );

        assert_eq!(vec![(0, vec!['a', 'd'])], *index.get(&vec!['q', 'u']));
    }
}
