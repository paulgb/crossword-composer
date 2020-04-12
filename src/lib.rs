extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;

mod grid;
mod solver;
mod index;
mod dictionary;

use self::js_sys::Array;
use crate::dictionary::Dictionary;
use web_sys::console;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Solver {
    dict: Dictionary,
}

#[wasm_bindgen]
impl Solver {
    pub fn new(words_arr: Array) -> Solver {
        let words: Vec<String> = words_arr.iter().map(|d| d.as_string().unwrap()).collect();

        let dict = Dictionary::from_vec(words);

        Solver {
            dict
        }
    }

    pub fn solve(&self, spec_arr: Array) -> JsValue {
        let spec: Vec<Vec<usize>> = spec_arr.iter().map(|a| {
            let ar: Array = a.into();

            ar.iter().map(|v| v.as_f64().unwrap() as usize).collect()
        }).collect();

        let grid = grid::Grid::new(spec);

        let result = solver::solve(&grid, &self.dict);
        console::log_1(&format!("{:?}", result).into());

        if let Some(r) = result {
            let v: Vec<JsValue> = r.iter().map(|c| c.to_string().into()).collect();
            let a: Array = v.iter().collect();
            
            a.into()
        } else {
            JsValue::NULL
        }
    }
}