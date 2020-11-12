use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(FromPrimitive, PartialEq, Copy, Clone)]
enum CellState {
    Human = -1,
    Empty = 0,
    AI = 1,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_i32(a: i32);

}
type Board = Vec<i32>;

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveScore {
    pub score: i32,
    pub move_val: Option<i32>,
}

fn is_winner(board: &Board, cell_state: CellState) -> bool {
    let dcs = cell_state as i32;

    // Diagonal
    if board[0] == dcs && board[4] == dcs && board[8] == dcs {
        return true;
    }
    // Anti-Diagonal
    if board[2] == dcs && board[4] == dcs && board[6] == dcs {
        return true;
    }
    // Row 1
    if board[0] == dcs && board[1] == dcs && board[2] == dcs {
        return true;
    }
    // Row 2
    if board[3] == dcs && board[4] == dcs && board[5] == dcs {
        return true;
    }
    // Row 3
    if board[6] == dcs && board[7] == dcs && board[8] == dcs {
        return true;
    }
    // Column 1
    if board[0] == dcs && board[3] == dcs && board[6] == dcs {
        return true;
    }
    // Column 2
    if board[1] == dcs && board[4] == dcs && board[7] == dcs {
        return true;
    }
    // Column 3
    if board[2] == dcs && board[5] == dcs && board[8] == dcs {
        return true;
    }

    return false;
}

fn get_winner(board: &Board) -> CellState {
    if is_winner(board, CellState::Human) {
        return CellState::Human;
    } else if is_winner(board, CellState::AI) {
        return CellState::AI;
    }
    return CellState::Empty;
}

fn get_opponent(player: CellState) -> CellState {
    if player == CellState::AI {
        return CellState::Human;
    } else {
        return CellState::AI;
    }
}

fn minimax(board: &Board, player: CellState) -> MoveScore {
    let winner = get_winner(board);
    if winner != CellState::Empty {
        if winner == player {
            return MoveScore {
                score: 1,
                move_val: None,
            };
        } else {
            return MoveScore {
                score: -1,
                move_val: None,
            };
        }
    }

    let mut move_val: Option<i32> = None;
    let mut score = -2;

    for i in 0..9 {
        if board[i] == CellState::Empty as i32 {
            // log_i32(i as i32);
            let mut new_board = board.clone();
            new_board[i] = player as i32;
            let score_for_move = -minimax(&new_board, get_opponent(player)).score;
            if score_for_move > score {
                score = score_for_move;
                move_val = Some(i as i32);
            }
        }
    }

    if move_val == None {
        return MoveScore { score: 0, move_val };
    }
    return MoveScore { score, move_val };
}

#[wasm_bindgen]
pub fn move_enemy(board: Board) -> JsValue {
    let res = minimax(&board, CellState::AI);
    unsafe {
        log(&format!("Score: {}", res.score));
        log(&format!("MoveVal: {:?}", res.move_val));
    }
    return JsValue::from_str(&serde_json::to_string(&res).unwrap());
    // return JsValue::from_serde(&res).unwrap();
}
