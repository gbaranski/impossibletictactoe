use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use wasm_bindgen::prelude::*;

#[derive(FromPrimitive, PartialEq, Copy, Clone)]
enum CellState {
    X = -1,
    Empty = 0,
    O = 1,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

type Board = Vec<i32>;

fn is_winner(board: &Board, cell_state: CellState) -> bool {
    let dcs = cell_state as i32;
    if board[0] == dcs && board[4] == dcs && board[8] == dcs {
        return true;
    // Cross top right to bottom left
    } else if board[2] == dcs && board[4] == dcs && board[6] == dcs {
        return true;
    // Vertical top left to bottom left
    } else if board[0] == dcs && board[3] == dcs && board[6] == dcs {
        return true;
    // Vertical middle top  to middle bottom
    } else if board[1] == dcs && board[4] == dcs && board[7] == dcs {
        return true;
    // Vertical top right to bottom right
    } else if board[2] == dcs && board[5] == dcs && board[8] == dcs {
        return true;
    // Horizontal top left to top right
    } else if board[0] == dcs && board[1] == dcs && board[2] == dcs {
        return true;
    // Horizontal middle left to middle right
    } else if board[3] == dcs && board[4] == dcs && board[5] == dcs {
        return true;
    // Horizontal bottom left to bottom right
    } else if board[6] == dcs && board[7] == dcs && board[8] == dcs {
        return true;
    }
    return false;
}

fn get_winner(board: &Board) -> CellState {
    if is_winner(board, CellState::X) {
        return CellState::X;
    } else if is_winner(board, CellState::O) {
        return CellState::O;
    }
    return CellState::Empty;
}

fn get_opponent(player: CellState) -> CellState {
    if player == CellState::X {
        return CellState::O;
    } else if player == CellState::O {
        return CellState::X;
    }

    return CellState::Empty;
}

fn are_moves_left(board: &Board) -> bool {
    for i in 0..board.len() {
        let cell_state: CellState =
            FromPrimitive::from_i32(board[i]).expect("Couln't convert primitite to Cell State");
        if cell_state == CellState::Empty {
            return true;
        }
    }
    return false;
}

fn minimax(board: &Board, depth: i32, is_maximizing_player: bool) -> i32 {
    if get_winner(board) ==

}

#[wasm_bindgen]
pub fn move_enemy(board: Board) -> Option<i32> {
    let new_move: i32 = minimax(&board, CellState::X as i32);
    log(&format!("Move: {}", new_move));

    let mut i = 0;
    for cell in board {
        let cell_state: CellState =
            FromPrimitive::from_i32(cell).expect("Could not convert primitive to CellState");

        if cell_state == CellState::Empty {
            return Some(i);
        }

        i = i + 1;
    }
    return None;
}
