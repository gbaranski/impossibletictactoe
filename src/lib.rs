use num_derive::FromPrimitive;
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

fn minimax(board: &Board, player: CellState) -> i32 {
    let winner = get_winner(board);
    if winner != CellState::Empty {
        if winner == player {
            return 1;
        } else {
            return -1;
        }
    }

    let mut move_val: i32 = -1;
    let mut score = -2;

    for i in 0..9 {
        if board[i] == CellState::Empty as i32 {
            // log_i32(i as i32);
            let mut new_board = board.clone();
            new_board[i] = player as i32;
            let score_for_move = -minimax(&new_board, get_opponent(player));
            if score_for_move > score {
                score = score_for_move;
                move_val = i as i32;
            }
        }
    }

    if move_val == -1 {
        return 0;
    }
    return move_val;
}

#[wasm_bindgen]
pub fn move_enemy(board: Board) -> i32 {
    let new_move: i32 = minimax(&board, CellState::AI);
    return new_move;
}
