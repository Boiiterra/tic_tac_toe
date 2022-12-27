#[derive(PartialEq)]
enum GameType {
    Choose,
    PvP,
    PvE,
}

#[derive(PartialEq)] //* working on that soon
enum Difficulty {
    Easy,
    Medium,
}

#[derive(PartialEq)]
struct Player {
    mark: char,
}

#[derive(PartialEq)]
struct Bot {
    mark: char,
    difficulty: Difficulty,
    field: [u8; 9],
}

#[derive(PartialEq)]
enum Participants {
    Player(Player),
    Bot(Bot),
}

impl Participants {
    fn get
}

impl Bot {
    fn new(mark: char) -> Self {
        Self {
            mark,
            difficulty: Difficulty::Easy,
            field: [0; 9]
        }
    }

    fn change_difficulty(&mut self, to: Difficulty) {
        self.difficulty = to;
    }
}

pub struct TicTacToe {
    game_table: [Option<char>; 9],
    game_type: GameType,
    x_wins: u32,
    o_wins: u32,
}

impl TicTacToe {
    fn new() -> Self {
        Self { game_table: [None; 9], game_type: GameType::Choose, x_wins: 0, o_wins: 0 }
    }

    fn change_type(&mut self, to: GameType) {
        self.game_type = to;
    }

    fn make_move(&mut self, index: usize, player_char: char) {
        self.game_table[index] = Some(player_char);
    }

    fn x_won(&mut self) {
        self.x_wins += 1;
    }

    fn o_won(&mut self) {
        self.o_wins += 1;
    }
}

use std::{fmt::Display, io::Write};
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
            self.game_table[0].unpack(), // row one start
            self.game_table[1].unpack(),
            self.game_table[2].unpack(), // row one end

            self.game_table[3].unpack(), // row two start
            self.game_table[4].unpack(),
            self.game_table[5].unpack(), // row two end

            self.game_table[6].unpack(), // row three start
            self.game_table[7].unpack(),
            self.game_table[8].unpack()  // row three end
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

fn choose_game_type(game: &mut TicTacToe) {
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
            game.change_type(GameType::PvP);
            break;
        } else if index == 2 {
            game.change_type(GameType::PvE);
            break;
        }
    }
}

fn setup_players(game: &TicTacToe) -> [Participants; 2] {
        if game.game_type == GameType::PvP {
            print!("Decide who plays with 'X' and who plays with 'O'.");
            [Participants::Player(Player {mark: 'X',}), Participants::Player(Player {mark: 'O',})]
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

                if number == 1 {
                    return [Participants::Player(Player {mark: 'X'}), Participants::Bot(Bot::new('O'))];
                } else {
                    return [Participants::Bot(Bot::new('X')), Participants::Player(Player {mark: 'O'})];
                }
            }
        }
}

fn change_table(game: &mut TicTacToe, player_char: char) {
    loop {
        print!("Where do you want to place '{}'", player_char);
        std::io::stdout().flush().unwrap();

        let raw_input = input();

        let index: usize = match raw_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number. Please try again.");
                continue;
            }
        };

        if game
            .game_table
            .get(index)
            .expect("Please don't write index out of table")
            .is_none()
        {
            println!("'{}'", game.game_table[index].unpack());
            println!(
                "Your move is {}. \"{}\"",
                index,
                game.game_table[index].unpack()
            );
            game.make_move(index, player_char);
            break;
        } else {
            println!(
                "Unfortunately you can't place '{}' here.",
                player_char
            );
        }
    }
}

fn choose_difficulty(game: &mut TicTacToe) { // TODO: finish this one
    print!("Which difficulty do you want to choose? 1. Easy or 2. Medium");
    let raw_input = input();
}

pub fn play() {
    let mut game = TicTacToe::new();
    // choose_game_type(&mut game);
    game.change_type(GameType::PvP);
    let participants = setup_players(&game);
    let mut index: usize;

    if game.game_type == GameType::PvE {
        choose_difficulty(&mut game)
    }

    println!("\nSome hint for positions:\n\
              +---+---+---+\n\
              | 0 | 1 | 2 |\n\
              +---+---+---+\n\
              | 3 | 4 | 5 |\n\
              +---+---+---+\n\
              | 6 | 7 | 8 |\n\
              +---+---+---+\n");

    println!("{game}");

    for i in 0..9 {
        if i % 2 == 0 {
            index = 0;
        } else {
            index = 1;
        }
        change_table(&mut game, participants[index].)
    }

}