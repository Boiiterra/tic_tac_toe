use rand::seq::SliceRandom;
use std::io::Write;

#[derive(PartialEq)]
enum GameType {
    Choose,
    PvP,
    PvE,
}

#[derive(PartialEq)]
enum Winner {
    None,
    O,
    X,
}

impl Winner {
    fn check(cell: char) -> Winner {
        match cell {
            'O' => Winner::O,
            'X' => Winner::X,
            ' ' => Winner::None,
            _ => panic!(
                "This char is not supported, please revert your changes, or make it supported!"
            ),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)] //* working on that soon
enum Difficulty {
    ChaoticRandom, // Pure random with overwriting!
    Easy,          // Random
    Defender,      // My own thingy
    _Medium,       // Minimax alg
}

#[derive(Debug, Clone, Copy)]
struct Player {
    mark: char,
}

impl Player {
    fn new_x() -> Self {
        Self { mark: 'X' }
    }

    fn new_o() -> Self {
        Self { mark: 'O' }
    }
}

#[derive(Debug, Clone, Copy)]
struct Bot {
    mark: char,
    field: [i8; 9],
    difficulty: Difficulty,
}

impl Bot {
    fn construct(mark: char, difficulty: Difficulty) -> Self {
        Self {
            mark,
            field: [0; 9],
            difficulty,
        }
    }

    fn update_field(&mut self, at: usize, enemy: bool) {
        self.field[at] = -1 - (enemy as i8);

        if self.difficulty == Difficulty::Defender {
            if enemy {
                match at {
                    0 => {
                        if self.field[2] == -2 && self.field[1] >= 0 {
                            self.field[1] += 2;
                        } else if self.field[1] == -2 && self.field[2] >= 0 {
                            self.field[2] += 2;
                        } else if self.field[1] >= 0 {
                            self.field[1] += 1;
                        }

                        if self.field[6] == -2 && self.field[3] >= 0 {
                            self.field[3] += 2;
                        } else if self.field[3] == -2 && self.field[6] >= 0 {
                            self.field[6] += 2;
                        } else if self.field[3] >= 0 {
                            self.field[3] += 1;
                        }

                        if self.field[8] == -2 && self.field[4] >= 0 {
                            self.field[4] += 2;
                        } else if self.field[4] == -2 && self.field[8] >= 0 {
                            self.field[8] += 2;
                        } else if self.field[4] >= 0 {
                            self.field[4] += 1;
                        }
                    }
                    1 => {
                        if self.field[2] == -2 && self.field[0] >= 0 {
                            self.field[0] += 2;
                        } else if self.field[0] == -2 && self.field[2] >= 0 {
                            self.field[2] += 2;
                        }
                        if self.field[0] >= 0 {
                            self.field[0] += 1;
                        }
                        if self.field[2] >= 0 {
                            self.field[2] += 1;
                        }

                        if self.field[7] == -2 && self.field[4] >= 0 {
                            self.field[4] += 2;
                        } else if self.field[4] == -2 && self.field[7] >= 0 {
                            self.field[7] += 2;
                        } else if self.field[4] >= 0 {
                            self.field[4] += 1;
                        }

                        if self.field[3] >= 0 {
                            self.field[3] += 1;
                        }
                        if self.field[5] >= 0 {
                            self.field[5] += 1;
                        }
                    }
                    2 => {
                        if self.field[0] == -2 && self.field[1] >= 0 {
                            self.field[1] += 2;
                        } else if self.field[1] == -2 && self.field[0] >= 0 {
                            self.field[0] += 2;
                        } else if self.field[1] >= 0 {
                            self.field[1] += 1;
                        }

                        if self.field[8] == -2 && self.field[5] >= 0 {
                            self.field[5] += 2;
                        } else if self.field[5] == -2 && self.field[8] >= 0 {
                            self.field[8] += 2;
                        } else if self.field[5] >= 0 {
                            self.field[5] += 1;
                        }

                        if self.field[6] == -2 && self.field[4] >= 0 {
                            self.field[4] += 2;
                        } else if self.field[4] == -2 && self.field[6] >= 0 {
                            self.field[6] += 2;
                        } else if self.field[4] >= 0 {
                            self.field[4] += 1;
                        }
                    }
                    3 => {
                        if self.field[5] == -2 && self.field[4] >= 0 {
                            self.field[4] += 2;
                        } else if self.field[4] == -2 && self.field[5] >= 0 {
                            self.field[5] += 2;
                        }
                        if self.field[6] == -2 && self.field[0] >= 0 {
                            self.field[0] += 2;
                        } else if self.field[0] == -2 && self.field[6] >= 0 {
                            self.field[0] += 2;
                        }

                        if self.field[4] >= 0 {
                            self.field[4] += 1;
                        }
                        if self.field[6] >= 0 {
                            self.field[6] += 1;
                        }
                        if self.field[0] >= 0 {
                            self.field[0] += 1;
                        }
                        if self.field[1] >= 0 {
                            self.field[1] += 1;
                        }
                        if self.field[7] >= 0 {
                            self.field[7] += 1;
                        }
                    }
                    4 => {
                        if self.field[1] == -2 && self.field[7] >= 0 {
                            self.field[7] += 2;
                        } else if self.field[7] == -2 && self.field[1] >= 0 {
                            self.field[1] += 2;
                        }
                        if self.field[3] == -2 && self.field[5] >= 0 {
                            self.field[5] += 2;
                        } else if self.field[5] == -2 && self.field[3] >= 0 {
                            self.field[3] += 2;
                        }
                        if self.field[0] == -2 && self.field[8] >= 0 {
                            self.field[8] += 2;
                        } else if self.field[8] == -2 && self.field[0] >= 0 {
                            self.field[0] += 2;
                        }
                        if self.field[2] == -2 && self.field[6] >= 0 {
                            self.field[6] += 2;
                        } else if self.field[6] == -2 && self.field[2] >= 0 {
                            self.field[2] += 2;
                        }

                        if self.field[0] >= 0 {
                            self.field[0] += 1;
                        }
                        if self.field[1] >= 0 {
                            self.field[1] += 1;
                        }
                        if self.field[2] >= 0 {
                            self.field[2] += 1;
                        }
                        if self.field[3] >= 0 {
                            self.field[3] += 1;
                        }
                        if self.field[5] >= 0 {
                            self.field[5] += 1;
                        }
                        if self.field[6] >= 0 {
                            self.field[6] += 1;
                        }
                        if self.field[7] >= 0 {
                            self.field[7] += 1;
                        }
                        if self.field[8] >= 0 {
                            self.field[8] += 1;
                        }
                    }
                    5 => {
                        if self.field[2] == -2 && self.field[8] >= 0 {
                            self.field[8] += 2;
                        } else if self.field[8] == -2 && self.field[2] >= 0 {
                            self.field[2] += 2;
                        }
                        if self.field[3] == -2 && self.field[4] >= 0 {
                            self.field[4] += 2;
                        } else if self.field[4] == -2 && self.field[3] >= 0 {
                            self.field[3] += 2;
                        }

                        if self.field[1] >= 0 {
                            self.field[1] += 1;
                        }
                        if self.field[1] >= 0 {
                            self.field[1] += 1;
                        }
                        if self.field[2] >= 0 {
                            self.field[2] += 1;
                        }
                        if self.field[4] >= 0 {
                            self.field[4] += 1;
                        }
                        if self.field[7] >= 0 {
                            self.field[7] += 1;
                        }
                        if self.field[8] >= 0 {
                            self.field[8] += 1;
                        }
                    }
                    6 => {
                        if self.field[0] == -2 && self.field[3] >= 0 {
                            self.field[3] += 2;
                        } else if self.field[3] == -2 && self.field[0] >= 0 {
                            self.field[0] += 2;
                        }
                        if self.field[7] == -2 && self.field[8] >= 0 {
                            self.field[8] += 2;
                        } else if self.field[8] == -2 && self.field[7] >= 0 {
                            self.field[7] += 2;
                        }
                        if self.field[2] == -2 && self.field[4] >= 0 {
                            self.field[4] += 2;
                        } else if self.field[4] == -2 && self.field[2] >= 0 {
                            self.field[2] += 2;
                        }

                        if self.field[3] >= 0 {
                            self.field[3] += 1;
                        }
                        if self.field[4] >= 0 {
                            self.field[4] += 1;
                        }
                        if self.field[7] >= 0 {
                            self.field[7] += 1;
                        }
                    }
                    7 => {
                        if self.field[1] == -2 && self.field[4] >= 0 {
                            self.field[4] += 2;
                        } else if self.field[4] == -2 && self.field[1] >= 0 {
                            self.field[1] += 2;
                        }
                        if self.field[6] == -2 && self.field[8] >= 0 {
                            self.field[8] += 2;
                        } else if self.field[8] == -2 && self.field[6] >= 0 {
                            self.field[6] += 2;
                        }

                        if self.field[3] >= 0 {
                            self.field[3] += 1;
                        }
                        if self.field[4] >= 0 {
                            self.field[4] += 1;
                        }
                        if self.field[5] >= 0 {
                            self.field[5] += 1;
                        }
                        if self.field[6] >= 0 {
                            self.field[6] += 1;
                        }
                        if self.field[8] >= 0 {
                            self.field[8] += 1;
                        }
                    }
                    8 => {
                        if self.field[0] == -2 && self.field[4] >= 0 {
                            self.field[4] += 2;
                        } else if self.field[4] == -2 && self.field[0] >= 0 {
                            self.field[0] += 2;
                        }
                        if self.field[2] == -2 && self.field[5] >= 0 {
                            self.field[5] += 2;
                        } else if self.field[5] == -2 && self.field[2] >= 0 {
                            self.field[2] += 2;
                        }
                        if self.field[6] == -2 && self.field[7] >= 0 {
                            self.field[7] += 2;
                        } else if self.field[7] == -2 && self.field[6] >= 0 {
                            self.field[6] += 2;
                        }

                        if self.field[4] >= 0 {
                            self.field[4] += 1;
                        }
                        if self.field[5] >= 0 {
                            self.field[5] += 1;
                        }
                        if self.field[7] >= 0 {
                            self.field[7] += 1;
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    fn make_move(&self) -> usize {
        if self.difficulty == Difficulty::Defender {
            let mut allowed: Vec<usize> = Vec::new();
            let mut prev: i8 = -1;
            let mut pos: usize = 0;
            for i in self.field.iter() {
                if *i > prev {
                    // Found more valueble place.
                    prev = *i;
                    allowed = Vec::new();
                }
                if *i == prev {
                    allowed.push(pos);
                }
                pos += 1;
            }
            *allowed.choose(&mut rand::thread_rng()).unwrap() // Should not panic
        } else if self.difficulty == Difficulty::Easy {
            let mut allowed: Vec<usize> = Vec::new();
            let mut pos: usize = 0;
            for i in self.field.iter() {
                if *i == 0 {
                    allowed.push(pos);
                }
                pos += 1;
            }
            *allowed.choose(&mut rand::thread_rng()).unwrap() // Should not panic
        } else {
            *[0usize, 1, 2, 3, 4, 5, 6, 7, 8]
                .choose(&mut rand::thread_rng())
                .unwrap() // Should not panic
        }
    }
}

#[derive(Debug)]
enum Participants {
    Player(Player),
    Bot(Bot),
}

struct TicTacToe {
    game_table: [Option<char>; 9],
    game_table_int: [i8; 9],
    game_type: GameType,
    x_score: u32,
    o_score: u32,
}

impl TicTacToe {
    fn new() -> Self {
        Self {
            game_table: [None; 9],
            game_type: GameType::Choose,
            game_table_int: [0; 9],
            x_score: 0,
            o_score: 0,
        }
    }

    fn new_table(&mut self) {
        self.game_table = [None; 9];
        self.game_table_int = [0i8; 9];
    }

    fn change_type(&mut self) {
        loop {
            print!("Which game type do you wand to choose: 1. PvP or 2. PvE: ");
            std::io::stdout().flush().unwrap();

            let raw_input = input();

            let index: u8 = match raw_input.trim().parse() {
                Ok(num) => {
                    if num == 1 || num == 2 {
                        num
                    } else {
                        println!("Please try to enter 1 or 2 not '{num}'.");
                        continue;
                    }
                }
                Err(_) => {
                    println!("Something wrong happened. Please try again.");
                    continue;
                }
            };

            if index == 1 {
                self.game_type = GameType::PvP;
                break;
            } else if index == 2 {
                self.game_type = GameType::PvE;
                break;
            }
        }
    }

    fn update_table(&mut self, at: usize, player_char: char) {
        self.game_table[at] = Some(player_char);

        if player_char == 'X' {
            self.game_table_int[at] = 1
        } else {
            self.game_table_int[at] = -1
        }
    }

    fn get_winner(&self) -> Winner {
        if (self.game_table_int[0] + self.game_table_int[1] + self.game_table_int[2]).abs() == 3 {
            return Winner::check(self.game_table[0].unpack());
        }
        if (self.game_table_int[3] + self.game_table_int[4] + self.game_table_int[5]).abs() == 3 {
            return Winner::check(self.game_table[3].unpack());
        }
        if (self.game_table_int[6] + self.game_table_int[7] + self.game_table_int[8]).abs() == 3 {
            return Winner::check(self.game_table[6].unpack());
        }

        if (self.game_table_int[0] + self.game_table_int[3] + self.game_table_int[6]).abs() == 3 {
            return Winner::check(self.game_table[0].unpack());
        }
        if (self.game_table_int[1] + self.game_table_int[4] + self.game_table_int[7]).abs() == 3 {
            return Winner::check(self.game_table[1].unpack());
        }
        if (self.game_table_int[2] + self.game_table_int[5] + self.game_table_int[8]).abs() == 3 {
            return Winner::check(self.game_table[2].unpack());
        }

        if (self.game_table_int[0] + self.game_table_int[4] + self.game_table_int[8]).abs() == 3 {
            return Winner::check(self.game_table[0].unpack());
        }
        if (self.game_table_int[2] + self.game_table_int[4] + self.game_table_int[6]).abs() == 3 {
            return Winner::check(self.game_table[2].unpack());
        }

        // no one is a winner :(
        Winner::None
    }

    fn score(&self) {
        println!(
            "+-Game score--+\n\
             |>   X : O   <|\n\
             |>   {} : {}   <|\n\
             +-------------+\n",
            self.x_score, self.o_score
        );
    }

    fn x_won(&mut self) {
        self.x_score += 1;
    }

    fn o_won(&mut self) {
        self.o_score += 1;
    }
}

use std::fmt::Display;
impl Display for TicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "+---+---+---+\n\
             | {} | {} | {} |\n\
             +---+---+---+\n\
             | {} | {} | {} |\n\
             +---+---+---+\n\
             | {} | {} | {} |\n\
             +---+---+---+\n",
            if self.game_table[0].unpack() == ' ' {
                '1'
            } else {
                self.game_table[0].unpack()
            }, // row one start
            if self.game_table[1].unpack() == ' ' {
                '2'
            } else {
                self.game_table[1].unpack()
            },
            if self.game_table[2].unpack() == ' ' {
                '3'
            } else {
                self.game_table[2].unpack()
            }, // row one end
            if self.game_table[3].unpack() == ' ' {
                '4'
            } else {
                self.game_table[3].unpack()
            }, // row two start
            if self.game_table[4].unpack() == ' ' {
                '5'
            } else {
                self.game_table[4].unpack()
            },
            if self.game_table[5].unpack() == ' ' {
                '6'
            } else {
                self.game_table[5].unpack()
            }, // row two end
            if self.game_table[6].unpack() == ' ' {
                '7'
            } else {
                self.game_table[6].unpack()
            }, // row three start
            if self.game_table[7].unpack() == ' ' {
                '8'
            } else {
                self.game_table[7].unpack()
            },
            if self.game_table[8].unpack() == ' ' {
                '9'
            } else {
                self.game_table[8].unpack()
            } // row three end
        )
    }
}

trait Unpack {
    fn unpack(&self) -> char;
}

impl Unpack for Option<char> {
    fn unpack(&self) -> char {
        self.unwrap_or(' ')
    }
}

fn input() -> String {
    let mut output = String::new();
    std::io::stdin()
        .read_line(&mut output)
        .expect("Invalid Unicode character");
    output
}

fn setup_players(game: &TicTacToe) -> [Participants; 2] {
    if game.game_type == GameType::PvP {
        println!("Decide who plays with 'X' and who plays with 'O'.");
        [
            Participants::Player(Player::new_x()),
            Participants::Player(Player::new_o()),
        ]
    } else {
        loop {
            print!("Who do you want to play as 1. 'X' or 2. 'O': ");
            std::io::stdout().flush().unwrap();

            let raw_input = input();

            let number: u8 = match raw_input.trim().parse() {
                Ok(num) => {
                    if num == 1 || num == 2 {
                        num
                    } else {
                        println!("Please choose 1 or 2 next time not '{}'", num);
                        continue;
                    }
                }
                Err(_) => {
                    println!("Unfortunately this is not a valid number.");
                    continue;
                }
            };

            let difficulty = change_difficulty();

            if number == 1 {
                return [
                    Participants::Player(Player::new_x()),
                    Participants::Bot(Bot::construct('O', difficulty)),
                ];
            } else {
                return [
                    Participants::Bot(Bot::construct('X', difficulty)),
                    Participants::Player(Player::new_o()),
                ];
            }
        }
    }
}

fn change_difficulty() -> Difficulty {
    loop {
        print!("Which difficulty do you want to choose? 1. Chaotic Random, 2. Easy or 3. Medium: ");
        std::io::stdout().flush().unwrap();

        let raw_input = input();

        let choice: u8 = match raw_input.trim().parse() {
            Ok(num) => {
                if num == 1 || num == 2 || num == 3 {
                    num
                } else {
                    println!("Try entering 1, 2 or 3.");
                    continue;
                }
            }
            Err(_) => {
                println!("This is not a number. Please try again.");
                continue;
            }
        };

        match choice {
            1 => return Difficulty::ChaoticRandom,
            2 => return Difficulty::Easy,
            3 => return Difficulty::Defender,
            _ => panic!("This was unexpected! BUG!!!"),
        }
    }
}

fn change_table(game: &mut TicTacToe, player_char: char) -> usize {
    loop {
        print!("Where do you want to place '{}': ", player_char);
        std::io::stdout().flush().unwrap();

        let raw_input = input();

        let mut index: usize = match raw_input.trim().parse() {
            Ok(num) => {
                if num > 0 && num < 10 {
                    num
                } else {
                    println!("Please try integer between 0 and 10.");
                    continue;
                }
            }
            Err(_) => {
                println!("Not a number. Please try again.");
                continue;
            }
        };

        index -= 1;

        if game
            .game_table
            .get(index)
            .expect("Please don't write index out of table.") // This won't panic too - IF panic -> something went completely wrong.
            .is_none()
        {
            return index;
        } else {
            println!(
                "Unfortunately you can't place '{}' here. Please try another position.",
                player_char
            );
        }
    }
}

fn replay() -> bool {
    loop {
        print!("Do you want to continue? Y/n: ");
        std::io::stdout().flush().unwrap();

        let mut r_in = input();

        r_in = r_in.trim().to_owned();

        if r_in == "n" || r_in == "N" || r_in == "No" || r_in == "no" {
            return false;
        }

        if r_in == "y" || r_in == "Y" || r_in == "Yes" || r_in == "yes" || r_in == "" {
            return true;
        }

        println!("Not a valid input");
    }
}

fn main() {
    let mut game = TicTacToe::new();
    loop {
        game.change_type(); // PvP or PvE.

        let mut participants: [Participants; 2] = setup_players(&game);
        let mut winner: Winner;
        let mut index: usize;
        let mut move_ = -1;

        println!("{game}");

        while game.game_table.iter().any(|&i| i == None) {
            // While there are free cells game is in progress.
            move_ += 1;
            if move_ % 2 == 0 {
                index = 0;
            } else {
                index = 1;
            }

            match participants[index] {
                Participants::Bot(mut bot) => {
                    let at = bot.make_move();
                    bot.update_field(at, false);
                    if bot.difficulty != Difficulty::ChaoticRandom {
                        participants[index] = Participants::Bot(bot);
                    }
                    game.update_table(at, bot.mark)
                }
                Participants::Player(player) => {
                    let at = change_table(&mut game, player.mark);
                    if game.game_type == GameType::PvE {
                        match participants[(index + 1) % 2] {
                            Participants::Bot(mut bot) => {
                                bot.update_field(at, true);
                                if bot.difficulty != Difficulty::ChaoticRandom {
                                    participants[(index + 1) % 2] = Participants::Bot(bot);
                                }
                            }
                            _ => (),
                        };
                    }
                    game.update_table(at, player.mark)
                }
            }
            println!("{game}");

            winner = game.get_winner(); // Is there a winner?

            match winner {
                Winner::X => {
                    game.x_won();
                    break;
                }
                Winner::O => {
                    game.o_won();
                    break;
                }
                Winner::None => (), // No one is a winner.
            }
        }
        println!("{game}"); // Draw final table.
        game.score(); // Draw game scores.

        // Replay the game?
        if replay() {
            // Reset the game table.
            game.new_table();
        } else {
            break;
        } // End the program.
    }
}
