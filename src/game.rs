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
            _ => panic!("This char is not supported, please revert your changes!"),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)] //* working on that soon
enum Difficulty {
    ChaoticRandom,
    Easy,
    Medium,
}

#[derive(Debug, Clone, Copy)]
struct Player {
    mark: char,
}

impl Player {
    fn new_x() -> Self {
        Self {
            mark: 'X',
        }
    }

    fn new_o() -> Self {
        Self {
            mark: 'O',
        }
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
        if enemy {
            self.field[at] = -1;
        } else {
            self.field[at] = -2;
        }
        println!("{:?}", self.field);

        if self.difficulty == Difficulty::Medium {
            ();
        }
    }

    fn make_move(&self) -> usize {
        if self.difficulty == Difficulty::Medium {
            1
        } else if self.difficulty == Difficulty::Easy {
            let mut allowed: Vec<usize> = Vec::new();
            let mut pos: usize = 0;
            for i in self.field.iter() {
                if *i == 0 {
                    allowed.push(pos);
                    pos += 1;
                    println!("Available at {}", pos);
                }
            }
            println!("{:?}", self.field);
            *allowed.choose(&mut rand::thread_rng()).unwrap() // Should not panic
        } else {
            let mut allowed: Vec<usize> = Vec::new();
            for i in 0..9 {
                if self.field[i] == 0 {
                    allowed.push(i);
                }
            }
            *allowed.choose(&mut rand::thread_rng()).unwrap() // Should not panic
        }
    }
}

#[derive(Debug)]
enum Participants {
    Player(Player),
    Bot(Bot),
}

pub struct TicTacToe {
    game_table: [Option<char>; 9],
    game_type: GameType,
    x_score: u32,
    o_score: u32,
}

impl TicTacToe {
    fn new() -> Self {
        Self {
            game_table: [None; 9],
            game_type: GameType::Choose,
            x_score: 0,
            o_score: 0
        }
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
                    }},
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
    }

    fn get_winner(&self) -> Winner {
        if self.game_table[0] == self.game_table[1] && self.game_table[1] == self.game_table[2] {
            Winner::check(self.game_table[0].unpack())
        } else if self.game_table[3] == self.game_table[4] && self.game_table[4] == self.game_table[5] {
            Winner::check(self.game_table[3].unpack())
        } else if self.game_table[6] == self.game_table[7] && self.game_table[7] == self.game_table[8] {
            Winner::check(self.game_table[6].unpack())
        } else if self.game_table[0] == self.game_table[3] && self.game_table[3] == self.game_table[6] {
            Winner::check(self.game_table[0].unpack())
        } else if self.game_table[1] == self.game_table[4] && self.game_table[4] == self.game_table[7] {
            Winner::check(self.game_table[1].unpack())
        } else if self.game_table[2] == self.game_table[5] && self.game_table[5] == self.game_table[8] {
            Winner::check(self.game_table[2].unpack())
        } else if self.game_table[0] == self.game_table[4] && self.game_table[4] == self.game_table[8] {
            Winner::check(self.game_table[0].unpack())
        } else if self.game_table[2] == self.game_table[4] && self.game_table[4] == self.game_table[6] {
            Winner::check(self.game_table[2].unpack())
        } else {
            Winner::None
        }
    }

    fn score(&self) {
        println!("+-Game score--+\n\
                  |>   X : O   <|\n\
                  |>   {} : {}   <|\n\
                  +-------------+\n",
                self.x_score, self.o_score);
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
            if self.game_table[0].unpack() == ' ' {'1'} else {self.game_table[0].unpack()}, // row one start
            if self.game_table[1].unpack() == ' ' {'2'} else {self.game_table[1].unpack()},
            if self.game_table[2].unpack() == ' ' {'3'} else {self.game_table[2].unpack()}, // row one end

            if self.game_table[3].unpack() == ' ' {'4'} else {self.game_table[3].unpack()}, // row two start
            if self.game_table[4].unpack() == ' ' {'5'} else {self.game_table[4].unpack()},
            if self.game_table[5].unpack() == ' ' {'6'} else {self.game_table[5].unpack()}, // row two end

            if self.game_table[6].unpack() == ' ' {'7'} else {self.game_table[6].unpack()}, // row three start
            if self.game_table[7].unpack() == ' ' {'8'} else {self.game_table[7].unpack()},
            if self.game_table[8].unpack() == ' ' {'9'} else {self.game_table[8].unpack()}  // row three end
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
            [Participants::Player(Player::new_x()), Participants::Player(Player::new_o())]
        } else {
            loop {
                print!("Who do you want to play as 1. 'X' or 2. 'O': ");
                std::io::stdout().flush().unwrap();

                let raw_input = input();

                let number: u8 = match raw_input.trim().parse() {
                    Ok(num) => {
                        if num == 1 || num == 2 { num }
                        else {
                            println!("Please choose 1 or 2 next time not '{}'", num);
                            continue;
                        }
                    },
                    Err(_) => {
                        println!("Unfortunately this is not a valid number.");
                        continue;
                    }
                };

                let difficulty  = change_difficulty();

                if number == 1 {
                    return [Participants::Player(Player::new_x()), Participants::Bot(Bot::construct('O', difficulty))];
                } else {
                    return [Participants::Bot(Bot::construct('X', difficulty)), Participants::Player(Player::new_o())];
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
                if num == 1 || num == 2 || num == 3 { num }
                else {
                    println!("Try entering 1, 2 or 3.");
                    continue;
                }
            },
            Err(_) => {
                println!("This is not a number. Please try again.");
                continue;
            },
        };

        match choice {
            1 => return Difficulty::ChaoticRandom,
            2 => return Difficulty::Easy,
            3 => return Difficulty::Medium,
            _ => panic!("This was unexpected!"),
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
                    println!("Please try integer between 0 and 10");
                    continue;
                }
            },
            Err(_) => {
                println!("Not a number. Please try again.");
                continue;
            }
        };

        index -= 1;

        if game
            .game_table
            .get(index)
            .expect("Please don't write index out of table") // This won't panic too - IF panic -> something went completely wrong
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

pub fn play() {
    let mut game = TicTacToe::new();
    game.change_type();

    let participants: [Participants; 2] = setup_players(&game);
    let mut winner: Winner;
    let mut index: usize;
    let mut move_ = -1;

    println!("{:?}", participants);

    println!("{game}");

    while game.game_table.iter().any(|&i| i == None) {
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
                game.update_table(at, bot.mark)
            },
            Participants::Player(player) => {
                let at = change_table(&mut game, player.mark);
                if game.game_type == GameType::PvE {
                    if index == 1 {
                        match participants[0] {
                            Participants::Bot(mut bot) => {
                                bot.update_field(at, true)
                            },
                            _ => (),
                        };
                    } else {
                        match participants[1] {
                            Participants::Bot(mut bot) => {
                                bot.update_field(at, true)
                            },
                            _ => (),
                        };
                    }
                }
                game.update_table(at, player.mark)
            },
        }
        println!("{game}");

        winner = game.get_winner();

        match winner {
            Winner::X => {
                game.x_won();
                break;
            },
            Winner::O => {
                game.o_won();
                break;
            },
            Winner::None => (),
        }
    }
    println!("{game}");
    game.score();
}