mod hanabi;
use hanabi::{Color, Game, Player};
use std::io::Write;
use std::{collections::HashMap, io};

fn main() {
    println!("Welcome to Hanabi!");
    let num_player = get_player_count();
    let perfection = get_perfection();
    print!("\x1B[2J\x1B[1;1H");
    println!(
        "Initializing game with {} players and perfection: {}",
        num_player, perfection
    );

    let mut game = Game::new(perfection);
    let mut players = HashMap::<usize, Player>::new();
    let hand_size = if num_player < 5 { 5 } else { 4 };
    for i in 0..num_player {
        players.insert(i, Player::new(&mut game, hand_size));
    }

    println!("Enter ? for help");
    while !&game.ended {
        for i in 0..num_player {
            println!("{}", &game.token_string());
            println!("Played: {}", &game.played_string());
            print_hands(&players, num_player, &i);
            print_discards(&game);
            parse_command(&mut players, &mut game, &i);
            print!("\x1B[2J\x1B[1;1H");
        }
    }
}

fn get_player_count() -> usize {
    let mut number: Option<usize> = None;
    while number.is_none() {
        let mut input = String::new();
        print!("How many players? (2-5): ");
        std::io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        number = input.trim().parse::<usize>().ok();
        match number {
            Some(x) => {
                if x > 5 || x < 2 {
                    number = None
                } else {
                    break;
                }
            }
            None => {}
        }
        println!("Invalid input: {}", input);
    }
    number.unwrap()
}

fn get_perfection() -> bool {
    print!("Play with perfection? [y/N]: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    if input == "y" {
        true
    } else if input == "n" || input == "" {
        false
    } else {
        println!("Invalid input: {}", input);
        get_perfection()
    }
}

fn print_hands(players: &HashMap<usize, Player>, num_players: usize, cur_player: &usize) {
    let index_vec = (0..num_players).collect::<Vec<usize>>();
    println!("Your hand (Player {}):", cur_player);
    println!("\t{}", players[&cur_player].get_hand_string());
    println!("");

    for sec in index_vec.split(|x| x == cur_player).rev() {
        for i in sec {
            println! {"Player {}:", i};
            println!("\t{}", players[&i].peak_hand_string());
            println!("\t{}", players[&i].get_hand_string());
            println!("");
        }
    }
}

fn parse_command(players: &mut HashMap<usize, Player>, game: &mut Game, current_player: &usize) {
    let mut res: Option<usize> = None;
    while res.is_none() {
        print!("> ");
        let mut input = String::new();
        std::io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        let parts = input.split(" ").collect::<Vec<&str>>();

        res = match parts[0] {
            "?" => print_help(),
            "h" => hint(&parts, players, game),
            "p" => play(&parts, players, game, current_player),
            "d" => discard(&parts, players, game, current_player),
            "q" => {
                game.end_game("Quit by player");
                Some(0)
            }
            _ => None,
        };

        match res {
            Some(x) => break,
            None => (),
        }
        if parts[0] != "?" {
            println!("Invalid command: {}", input);
            println!("Enter ? for help");
        }
    }
}

fn print_help() -> Option<usize> {
    println!("");
    println!("Available commands: ");
    println!("'h <i> <n/c>' : Hint player <i> about <n>umber or <c>olor");
    println!("\t Each color should be the first letter, lower case (g,b,y,w,r)");
    println!("'p <i>' : Play card at index <i> in hand");
    println!("'d <i>' : Discard card at index <i> in hand");
    println!("'q' : End game.");
    println!("");
    println!("Discards will be displayed below hands, underlines mean that card is at risk");
    None
}

fn print_discards(game: &Game) {
    let lines = game.discarded_strings();
    if lines.len() > 0 {
        println!("Discards:");
    }
    for line in lines {
        println!("\t{}", line);
    }
}

fn hint(parts: &Vec<&str>, players: &mut HashMap<usize, Player>, game: &mut Game) -> Option<usize> {
    if game.hints == 0 {
        println!("Out of hint tokens, must play or discard.");
        return None;
    }

    let player = parts.get(1)?.parse::<usize>().ok()?;
    let value = parts.get(2)?;
    let number = value.parse::<usize>().ok();
    match number {
        Some(x) => players.get_mut(&player)?.get_number_hint(x),
        None => match *value {
            "r" => players.get_mut(&player)?.get_color_hint(Color::Red),
            "w" => players.get_mut(&player)?.get_color_hint(Color::White),
            "b" => players.get_mut(&player)?.get_color_hint(Color::Blue),
            "y" => players.get_mut(&player)?.get_color_hint(Color::Yellow),
            "g" => players.get_mut(&player)?.get_color_hint(Color::Green),
            _ => return None,
        },
    }
    game.hints -= 1;
    Some(0)
}

fn play(
    parts: &Vec<&str>,
    players: &mut HashMap<usize, Player>,
    game: &mut Game,
    current_player: &usize,
) -> Option<usize> {
    let index = parts.get(1)?.parse::<usize>().ok()?;
    if index > players.get(current_player)?.hand_size {
        return None;
    }
    players.get_mut(current_player)?.play(index, game);
    return Some(0);
}

fn discard(
    parts: &Vec<&str>,
    players: &mut HashMap<usize, Player>,
    game: &mut Game,
    current_player: &usize,
) -> Option<usize> {
    let index = parts.get(1)?.parse::<usize>().ok()?;
    if index > players.get(current_player)?.hand_size {
        return None;
    }
    players.get_mut(current_player)?.discard(index, game);
    return Some(0);
}
