use std::io::Write;

fn main() {
    let mut game_state = TicTacToe::new();
    let mut winner: Option<bool> = None;

    for i in 0..=8 {
        println!("{}", i);
        println!("{game_state}");

        let current_player_char: char;

        if i % 2 == 0 {
            current_player_char = 'X';
        } else {
            current_player_char = 'O';
        }

        game_state.change_table(current_player_char);
        winner = get_winner(game_state.game_table.map(|x| x.unwrap_or(' ')));
    }

    match winner {
        None => println!("No one won this game"),
        Some(true) => println!("Player 'X' won this game"),
        Some(false) => println!("Player 'O' won this game"),
    }
}

fn get_winner(table: [char; 9]) -> Option<bool> {
    if table[0] == table[1] && table[1] == table[2] {
        Some(table[0] == 'X')
    } else if table[3] == table[4] && table[4] == table[5] {
        Some(table[0] == 'X')
    } else if table[6] == table[7] && table[7] == table[8] {
        Some(table[0] == 'X')
    } else if table[0] == table[3] && table[3] == table[6] {
        Some(table[0] == 'X')
    } else if table[1] == table[4] && table[4] == table[7] {
        Some(table[0] == 'X')
    } else if table[2] == table[5] && table[5] == table[8] {
        Some(table[0] == 'X')
    } else if table[0] == table[4] && table[4] == table[8] {
        Some(table[0] == 'X')
    } else if table[2] == table[4] && table[4] == table[6] {
        Some(table[0] == 'X')
    } else {
        None
    }
}

struct TicTacToe {
    game_table: [Option<char>; 9],
}

impl TicTacToe {
    fn new() -> Self {
        Self {
            game_table: [None; 9],
        }
    }
    fn change_table(&mut self, current_player_char: char) {
        loop {
            print!(
                "Where do you want to place your {} (position, ex.: 0 or 8): ",
                current_player_char
            );
            std::io::stdout().flush().unwrap();

            let raw_user_choice = input();

            let user_choice: usize = match raw_user_choice.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Not a number. Please try again.");
                    continue;
                }
            };

            if self
                .game_table
                .get(user_choice)
                .expect("Please don't write index out of table")
                .is_none()
            {
                println!("'{}'", self.game_table[user_choice].p());
                println!(
                    "Your move is {}. \"{}\"",
                    user_choice,
                    self.game_table[user_choice].p()
                );
                self.game_table[user_choice] = Some(current_player_char);
                break;
            } else {
                println!(
                    "Unfortunately you can't place '{}' here.",
                    current_player_char
                );
            }
        }
    }
}

use std::fmt::Display;
impl Display for TicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "+---+---+---+\n\
             | {} | {} | {} |\n\
             +---+---+---+\n\
             | {} | {} | {} |\n\
             +---+---+---+\n\
             | {} | {} | {} |\n\
             +---+---+---+\n",
            self.game_table[0].p(),
            self.game_table[1].p(),
            self.game_table[2].p(),
            self.game_table[3].p(),
            self.game_table[4].p(),
            self.game_table[5].p(),
            self.game_table[6].p(),
            self.game_table[7].p(),
            self.game_table[8].p()
        )
    }
}

fn input() -> String {
    let mut output = String::new();
    std::io::stdin()
        .read_line(&mut output)
        .expect("Invalid Unicode character");
    output
}

// The code below and the code uses it must be refactored in some way
trait Shit {
    fn p(&self) -> char;
}

impl Shit for Option<char> {
    fn p(&self) -> char {
        self.unwrap_or(' ')
    }
}
