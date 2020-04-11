
pub struct Grid {
    pub slots: usize,
    pub words: Vec<Vec<usize>>,         // word -> ordered list of slots
    pub slot_to_words: Vec<Vec<usize>>, // slot -> list of words
}

impl Grid {
    pub fn num_words(&self) -> usize {
        self.words.len()
    }

    fn generate_slot_to_words(words: &Vec<Vec<usize>>, slots: usize) -> Vec<Vec<usize>> {
        let mut slot_to_words: Vec<Vec<usize>> = (0..slots).map(|_| Vec::new()).collect();

        for (i, word) in words.iter().enumerate() {
            for &c in word {
                slot_to_words[c].push(i)
            }
        }

        slot_to_words
    }

    pub fn new(words: Vec<Vec<usize>>) -> Grid {
        let slots: usize = words
            .iter()
            .map(|w| *w.iter().max().unwrap())
            .max()
            .unwrap()
            + 1;

        let slot_to_words = Grid::generate_slot_to_words(&words, slots);

        Grid {
            words,
            slots,
            slot_to_words,
        }
    }
}

pub fn simple_crossword() -> Grid {
    Grid::new(vec![
        vec![0, 1, 2, 3],
        vec![5, 6, 7],
        vec![8, 9, 10, 11, 12],
        vec![13, 14, 15],
        vec![17, 18, 19, 20],
        vec![0, 4, 8, 13],
        vec![9, 14, 17],
        vec![2, 5, 10, 15, 18],
        vec![3, 6, 11],
        vec![7, 12, 16, 20],
    ])
}

pub fn four_by_four_grid() -> Grid {
    Grid::new(vec![
        vec![0, 1, 2, 3],
        vec![4, 5, 6, 7],
        vec![8, 9, 10, 11],
        vec![12, 13, 14, 15],
        vec![0, 4, 8, 12],
        vec![1, 5, 9, 13],
        vec![2, 6, 10, 14],
        vec![3, 7, 11, 15],
    ])
}
