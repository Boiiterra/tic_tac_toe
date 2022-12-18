#![allow(unused)]

use std::io;
use std::io::Write;

fn main() {
    let mut players: [(char, bool); 2] = [('X', true), ('O', true)];
    let mut winner: Option<bool> = None;
    let mut table = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    let mut allowed_i: Vec<u8> = Vec::from([11, 12, 13, 21, 22, 23, 31, 32, 33]);

    let something: bool = game_type();
    players = who(something, players);

    for i in (0..=1) {
        println!("{} {}", players[i].0, players[i].1);
    }

    for i in (0..10) {

        println!("{}", i);
        draw_table(&table);

        let index: usize;

        if i % 2 == 0 {
            index = 0;
        } else {
            index = 1;
        }

        table = change_table(players[index], table, &mut allowed_i);
    }

    match winner {
        None => println!("No one won this game"),
        Some(true) => println!("Player 'X' won this game"),
        Some(false) => println!("Player 'O' won this game"),
    }
}

fn game_type() -> bool {
    let mut g_type: u32;

    loop {
        print!("Choose game mode: 1. PvP, 2. PvE (1/2): ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        g_type = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number. Please try again.");
                continue
            },
        };

        if (g_type == 1) || (g_type == 2) {
            break
        } else {
            println!("Please enter either 1 or 2 not \"{}\".", g_type);
        }
    }

    if g_type == 2 {
        println!("Your game mode is PvE.");
        true
    } else {
        println!("Your game mode is PvP.");
        false
    }
}

fn who(is_pve: bool, mut players: [(char, bool); 2]) -> [(char, bool); 2] {
    if is_pve {
        let mut position: u32;
        
        loop {
            print!("Who do you want to play as: 1. 'X', 2. 'O' (1/2): ");
            io::stdout().flush().unwrap();
            
            let mut input: String = String::new();

            io::stdin().read_line(&mut input).expect("Failed to read line.");
            position = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Not a number. Please try again.");
                    continue
                },
            };

            if (position == 1) || (position == 2) {
                break
            } else {
                println!("Please enter either 1 or 2. Not \"{}\"", position);
                continue
            }
        }

        if position == 1 {
            println!("You are playing as 'X'.");
            players[1].1 = false
        } else {
            println!("You are playing as 'O'.");
            players[0].1 = false
        }
    } else {
        println!("You have another player to play with. Now decide who is player one -> 'X' and who is two -> 'O'.");
    }
    println!("Now lets start. The game.");
    players
}

fn change_table(player: (char, bool), mut table: [char; 9], allowed_i: &mut Vec<u8>) -> [char; 9] {
    loop {
        print!("Where do you want to place your {} (row column, ex.: 21 or 33): ", player.0);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let number:u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number. Please try again.");
                continue
            },
        };

        if allowed_i.contains(&number) {
            let index: usize = ((number / 10 - 1) * 3 + (number % 10 - 1)) as usize;
            println!("Your move is row {} and column {}. index -> {} -> \"{}\"", number / 10, number % 10, index, table[index]);
            let index = allowed_i.iter().position(|x| *x == number).unwrap();
            allowed_i.remove(index);
            table[index] = player.0;
        } else {
            println!("Unfortunately you can't place '{}' here.", player.0)
        }
    }
    table
}

fn draw_table(table: &[char; 9]) {
    println!("    1   2   3  ");
    println!("  +---+---+---+");
    println!("1 | {} | {} | {} |", &table[0], &table[1], &table[2]);
    println!("  +---+---+---+");
    println!("2 | {} | {} | {} |", &table[3], &table[4], &table[5]);
    println!("  +---+---+---+");
    println!("3 | {} | {} | {} |", &table[6], &table[7], &table[8]);
    println!("  +---+---+---+");
}