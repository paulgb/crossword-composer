mod grid;
mod solver;
mod index;
mod dictionary;

fn main() {
    let dict = dictionary::Dictionary::from_file(
        String::from("words.txt")).unwrap();
        //String::from("/usr/share/dict/words")).unwrap();
    let grid = grid::simple_crossword();
    let result = solver::solve(&grid, &dict);
    println!("{:?}", result);
}
