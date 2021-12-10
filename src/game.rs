use std::fmt::{Display, Error, Formatter};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::game::State::{Finish, Progress};
use crate::get_input;



/// Enum Player
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player { X, O }


impl Player {
    /// Other change the other player and change the value to him
    /// Thanks to that it's easy to handle which player must play
    pub fn other(mut self) -> Self {
        self = match self {
            Player::X => Player::O,
            Player::O => Player::X
        };
        self
    }
}
/// # Display to show on the terminal
/// Put a color special for the each player
impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let c;
        match self {
            Player::X => {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
                    .expect("Error when changing color ");
                c = 'X';
            }
            Player::O => {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
                    .expect("Error when changing color ");
                c = 'O';
            }
        }
        let val = write!(f, "{}", c);
        stdout.reset().expect("Error when reset stdout");
        val
    }
}

/// ## Hut of the game
/// Simple tupple of Option<Player>
/// Where :
/// `None` nobody is on this hut
/// `Player` mean someone on it
#[derive(Copy, Clone, PartialEq, Debug)]
struct Hut (Option<Player>);


/// Implementation de Hut
impl Hut {

    /// Allow do add a player to the hut
    /// If someone already there nothing happen
    pub fn change_to(&mut self, player: Player) {
        if let None = self.0 {
            self.0 = Some(player);
        }
    }

    pub fn assert_player(&self) -> Result<Player, Error> {
        match self.0 {
            Some(p) => Ok(p),
            _ => { Err(Error {})}
        }
    }
}

/// Will use display on Player if existe else will be a char for None  = "\u{25A2}" -> ▢
impl Display for Hut {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => {write!(f, "\u{25A2}")}
            Some(p) => {write!(f, "{}", p)}
        }


    }
}

/// State of the game
enum State {
    /// `Progress` game not finish and contains the player to who is the turn
    Progress(Player),
    /// `Finish` contains option of Player mean the game is finish and if None is a draw else Player is the winner
    Finish(Option<Player>)
}

impl State {
    /// Function new return a State but will always return a Progress State because a game can start by Finish don't make sense
    pub fn new(player: Player) -> State {
        State::Progress(player)
    }

    /// Handle the call to the player.other() on himself by match but only on progress because we don't want to change the Winner like that
    /// This functionnality make the call to other at the end of the game not a danger
    pub fn other(&mut self) {
        match self {
            Progress(p) => {
                *self = Progress(p.other());
            }
            _ => {}
        };
    }
}

/// Const BOARD_LEN is the value usize of the board
/// Should always be = 3
const BOARD_LEN: usize = 3;

/// # struct of the game tic_tac_toe
/// Can be use easily by juste call the new function and everything is handler to make the main the more empty possible
pub struct TicTacToe {

    /// Attributes which is the board of BOARD_LEN x BOARD_LEN dimension
    /// each position is a Hut wich can only be None or Player
    /// `None` significate that the hut is empty and `Player` is the player on it
    board: [[Hut; BOARD_LEN]; BOARD_LEN],

    /// State of the game
    state: State
}

/// Show the board on 2D on the terminal with color isn't that wonderful ?!
impl Display for TicTacToe {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "  A B C").unwrap();

        for (index, val ) in self.board.iter().enumerate() {
            write!(f, "\n{} ", index+1).unwrap();
            for val2 in val {
                write!(f, "{} ", val2).unwrap();
            }
        }
        write!(f, "")
    }

}

impl TicTacToe {

    /// # Call to launch the game
    /// This function centralised the call of TicTacToe elsewhere because it's the only public function
    pub fn new()  {
        // we create the game
        let mut game = TicTacToe {
            state: State::new(Player::X),
            board: [[ Hut(None); BOARD_LEN]; BOARD_LEN]
        };

        // loop who contains the logic
        println!("Welcom to the game Tic-Tac-Toe ! ");
        loop {
            game.make_move();
            game.state.other();


            if game.calcul_if_over() {
                println!("Game finish !");
                if let Finish(w) = game.state {
                    match w {
                        None => { println!("Sadly it's was a draw ! ");}
                        Some(p) => { println!("Player : {} won fair and square", p)}
                    }
                    println!("{}", game);
                    return;
                }
            }

        }
    }

    /// Convert the tuple get by the stdin_handler::get_input into a tuple of `(usize,usize)`
    fn convert(tuple: (char, usize)) -> (usize, usize) {
        match tuple.0 {
            'A' => { (0, tuple.1)}
            'B' => { (1, tuple.1)}
            'C' => { (2, tuple.1)}
            _ => { panic!("Should never happen ! ") }
        }
    }

    /// # Danger must be call if we sure that the board is not full else this will be a infinite loop
    /// ### Recursif function
    /// while force the player to make a move
    fn make_move(&mut self) {
        println!("{}", self);
        match self.state {

            State::Progress(player) => {
                let tup = TicTacToe::convert(get_input(player));
                if !self.do_move(tup, player) {
                    let mut stdout = StandardStream::stdout(ColorChoice::Always);
                    stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                        .expect("Error when changing color ");
                    println!("Hut already used ! retry");
                    stdout.reset().expect("Error when reset stdout");
                    self.make_move()
                }

            }
            _ => { }
        }
    }
    /// Method who do the move in the tupple for the player if it's None
    fn do_move(&mut self, tupple: (usize, usize), player : Player) -> bool {
        let (col, line) = tupple;
        if let None = self.board[line][col].0  {
            self.board[line][col].change_to(player);
            true
        } else { false }
    }

    /// Test if the game is_over or not
    fn calcul_if_over(&mut self) -> bool {
        // test in diagonal
        if self.board[0][0].0 == self.board[2][2].0 && self.board[1][1].0 == self.board[2][2].0 && self.board[2][2].0 != None ||
                    self.board[2][0].0 == self.board[0][2].0 && self.board[1][1].0 == self.board[0][2].0 && self.board[0][2].0 != None {

            self.state = Finish(Some(self.board[1][1].assert_player().unwrap()));
            return true;
        }

        for i in 0..BOARD_LEN {
            // test line
            if self.board[i][0].0 == self.board[i][2].0 && self.board[i][1].0 == self.board[i][2].0 && self.board[i][1].0 != None {
                self.state = Finish(Some(self.board[i][1].assert_player().unwrap()));
                return true;
            }
            //test column
            if self.board[0][i].0 == self.board[2][i].0 && self.board[1][i].0 == self.board[2][i].0 && self.board[0][i].0 != None {
                self.state = Finish(Some(self.board[1][i].assert_player().unwrap()));
                return true;
            }
        }
        false
    }
}