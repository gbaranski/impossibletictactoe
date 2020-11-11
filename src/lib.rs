use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

// thats weird, i need to add PartialEq to compare anything with it?
#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    O,
    X,
    Empty,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CellState::O => write!(f, "O"),
            CellState::X => write!(f, "X"),
            CellState::Empty => write!(f, "Empty"),
        }
    }
}

static mut CELLS: [CellState; 9] = [CellState::Empty; 9];

pub fn doc() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    return window.document().expect("should have a document on window");
}

pub fn body() -> HtmlElement {
    return doc().body().expect("document should have a body");
}

pub enum GameStatus {
    WON,
    LOST,
    PENDING,
}

impl fmt::Display for GameStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameStatus::WON => write!(f, "WON"),
            GameStatus::LOST => write!(f, "LOST"),
            GameStatus::PENDING => write!(f, "PENDING"),
        }
    }
}

#[wasm_bindgen]
pub fn make_move() -> String {
    unsafe {
        let mut i = 0;
        for cell in &CELLS {
            if *cell == CellState::Empty {
                CELLS[i] = CellState::O;
                return i.to_string().into();
            }
            i = i + 1;
        }
        return "EOG".into();
    }
}

#[wasm_bindgen]
pub fn on_cell_click(string_i: &str) -> String {
    log(&format!("I: {}", string_i));
    let i = string_i.parse::<i32>().unwrap() as usize;
    unsafe {
        if CELLS[i] == CellState::Empty {
            CELLS[i] = CellState::X;
            log(&format!("new Cells[i]: {}", CELLS[i]));
            return "OK".into();
        } else if CELLS[i] == CellState::O {
            return "Cannot place X on O".into();
        } else if CELLS[i] == CellState::X {
            return "Cannot place X on X".into();
        } else {
            return "Cannot handle this move".into();
        }
    }
}
