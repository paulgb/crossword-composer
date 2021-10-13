use crate::{
    dictionary::Dictionary,
    grid::Grid,
    solver::{generate_solver_steps, solve_step, SolverState},
};

pub fn solve(
    grid: &Grid,
    dict: &Dictionary,
    thread_num: usize,
    num_threads: usize,
) -> Option<Vec<char>> {
    let steps = generate_solver_steps(&grid, &dict);

    let first_word_possibilities = dict.words.get(&steps[0].output_slots.len()).unwrap();

    let words_per_thread =
        (first_word_possibilities.len() as f64 / num_threads as f64).ceil() as usize;
    let starting_index = words_per_thread * thread_num;
    let ending_index = std::cmp::min(
        first_word_possibilities.len(),
        starting_index + words_per_thread,
    );

    for i in starting_index..ending_index {
        let mut state: SolverState = SolverState::new(&grid);

        // Add the initial word to state
        state.words.push(i);

        // Add the word's letters to result
        for (out_slot, letter) in steps[0]
            .output_slots
            .iter()
            .zip(&first_word_possibilities[i])
        {
            state.result[*out_slot] = *letter;
        }

        // Check if it is possible to solve with these words. If so, end program.
        if solve_step(&mut state, &steps, 1) {
            return Some(state.result);
        }
    }

    None
}
