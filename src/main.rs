use crate::game::TicTacToe;
use crate::stdin_handler::get_input;

mod game;
mod stdin_handler;

fn main() {
    TicTacToe::launch();
}
