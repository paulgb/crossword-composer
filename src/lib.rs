extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

mod dictionary;
mod grid;
mod index;
mod solver;

use self::js_sys::Array;
use crate::dictionary::Dictionary;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solver {
    dict: Dictionary,
}

#[wasm_bindgen]
impl Solver {
    pub fn new(words_arr: Array) -> Solver {
        console_error_panic_hook::set_once();

        let words: Vec<String> = words_arr.iter().map(|d| d.as_string().unwrap()).collect();
        let dict = Dictionary::from_vec(words);

        Solver { dict }
    }

    pub fn solve(&self, spec_arr: Array) -> JsValue {
        let spec: Vec<Vec<usize>> = spec_arr
            .iter()
            .map(|a| {
                let ar: Array = a.into();

                ar.iter().map(|v| v.as_f64().unwrap() as usize).collect()
            })
            .collect();

        let grid = grid::Grid::new(spec);

        let result = solver::solve(&grid, &self.dict);

        if let Some(r) = result {
            let v: Vec<JsValue> = r.iter().map(|c| c.to_string().into()).collect();
            let a: Array = v.iter().collect();

            a.into()
        } else {
            JsValue::NULL
        }
    }
}

mod par_solver;

#[wasm_bindgen(js_name = parSolve)]
pub fn par_solve(
    dict_words: Array,
    grid_words: Array,
    thread_num: usize,
    num_threads: usize,
) -> JsValue {
    console_error_panic_hook::set_once();

    let dict_words: Vec<String> = dict_words.iter().map(|d| d.as_string().unwrap()).collect();
    let dict = Dictionary::from_vec(dict_words);

    let grid_words: Vec<Vec<usize>> = grid_words
        .iter()
        .map(|a| {
            let ar: Array = a.into();
            ar.iter().map(|v| v.as_f64().unwrap() as usize).collect()
        })
        .collect();
    let grid = grid::Grid::new(grid_words);

    let result = par_solver::solve(&grid, &dict, thread_num, num_threads);

    if let Some(r) = result {
        let v: Vec<JsValue> = r.iter().map(|c| c.to_string().into()).collect();
        let a: Array = v.iter().collect();

        a.into()
    } else {
        JsValue::NULL
    }
}

#[test]
fn crossword_7secs_on_browser() {
    let slot_words = vec![
        vec![0, 10, 21, 30, 40, 51, 61, 71, 82, 92, 101],
        vec![0, 1],
        vec![1, 11, 22, 31, 41],
        vec![2, 13, 23, 33, 43, 53, 64, 74],
        vec![2, 3, 4, 5, 6, 7, 8, 9],
        vec![3, 14],
        vec![4, 15, 24, 34, 45],
        vec![5, 16, 25, 35],
        vec![6, 17, 26, 36, 46, 56, 67, 77],
        vec![7, 18, 27],
        vec![8, 19],
        vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
        vec![20, 29, 39, 50, 60, 70, 81, 91, 100, 111, 121],
        vec![21, 22],
        vec![24, 25, 26, 27],
        vec![28, 38, 49, 59],
        vec![28, 29],
        vec![30, 31, 32, 33],
        vec![32, 42, 52, 63, 73, 84],
        vec![34, 35, 36],
        vec![37, 48, 58, 69, 79, 89],
        vec![37, 38, 39],
        vec![40, 41, 42, 43, 44, 45],
        vec![44, 54, 65, 75, 85, 95, 104, 115],
        vec![46, 47, 48, 49, 50],
        vec![47, 57, 68, 78, 88, 98, 108, 119],
        vec![52, 53, 54],
        vec![55, 56, 57, 58, 59, 60],
        vec![61, 62, 63, 64, 65, 66],
        vec![62, 72, 83, 93],
        vec![67, 68, 69],
        vec![71, 72, 73, 74, 75],
        vec![76, 87, 97, 106, 117],
        vec![76, 77, 78, 79, 80, 81],
        vec![80, 90, 99, 110, 120],
        vec![82, 83, 84],
        vec![85, 86, 87],
        vec![86, 96, 105, 116],
        vec![88, 89, 90, 91],
        vec![92, 93],
        vec![94, 103, 114],
        vec![94, 95, 96, 97],
        vec![99, 100],
        vec![102, 113],
        vec![102, 103, 104, 105, 106, 107, 108, 109, 110, 111],
        vec![107, 118],
        vec![112, 113, 114, 115, 116, 117, 118, 119],
        vec![120, 121],
    ];
    let grid = grid::Grid::new(slot_words.clone());

    let words: Vec<String> = std::fs::read_to_string("./ui/public/words.txt")
        .unwrap()
        .split('\n')
        .map(|str| str.to_string())
        .collect();

    let result = solver::solve(&grid, &Dictionary::from_vec(words));
    assert!(result.is_some());
    let result = result.unwrap();

    println!(
        "Result: {:?}",
        slot_words
            .into_iter()
            .map(|v| v.into_iter().map(|n| result[n]).collect())
            .collect::<Vec<Vec<char>>>()
    );
}

#[test]
fn crossword_8x8() {
    let slot_words = vec![
        vec![0, 8, 15, 21, 27],
        vec![0, 1, 2, 3, 4, 5, 6, 7],
        vec![1, 9, 16, 22, 28, 33, 39, 47],
        vec![2, 10],
        vec![3, 11, 17, 23, 29, 35, 41, 49],
        vec![4, 12, 18, 24, 30, 36, 42, 50],
        vec![5, 13, 19],
        vec![6, 14, 20, 25, 31, 37, 44, 52],
        vec![8, 9, 10, 11, 12, 13, 14],
        vec![15, 16],
        vec![17, 18, 19, 20],
        vec![21, 22],
        vec![23, 24],
        vec![25, 26],
        vec![26, 32, 38, 45, 53],
        vec![27, 28],
        vec![29, 30],
        vec![31, 32],
        vec![33, 34, 35, 36],
        vec![34, 40, 48],
        vec![37, 38],
        vec![39, 40, 41, 42, 43, 44, 45],
        vec![43, 51],
        vec![46, 47, 48, 49, 50, 51, 52, 53],
    ];
    let grid = grid::Grid::new(slot_words.clone());

    let words: Vec<String> = std::fs::read_to_string("./ui/public/words.txt")
        .unwrap()
        .split('\n')
        .map(|str| str.to_string())
        .collect();

    let result = solver::solve(&grid, &Dictionary::from_vec(words));
    assert!(result.is_some());
    let result = result.unwrap();

    println!(
        "Result: {:?}",
        slot_words
            .into_iter()
            .map(|v| v.into_iter().map(|n| result[n]).collect())
            .collect::<Vec<Vec<char>>>()
    );
}
