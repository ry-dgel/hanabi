use colored::{ColoredString, Colorize};
use itertools::Itertools;
use std::cmp::PartialEq;
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;

use rand::seq::SliceRandom;
use rand::thread_rng;

struct Secret<T> {
    value: T,
    known: bool,
}

impl<T: PartialEq> Secret<T> {
    fn new(value: T) -> Self {
        return Secret {
            value,
            known: false,
        };
    }

    fn reveal(&mut self) -> () {
        self.known = true;
    }

    fn get(&self) -> Option<&T> {
        if self.known {
            Some(&self.value)
        } else {
            None
        }
    }

    fn reveal_if(&mut self, value: &T) -> () {
        if &self.value == value {
            self.reveal()
        }
    }

    fn peek(&self) -> &T {
        return &self.value;
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Color {
    Red,
    Blue,
    Green,
    White,
    Yellow,
}

struct Card {
    number: Secret<usize>,
    color: Secret<Color>,
}

impl Card {
    fn new(color: Color, number: usize) -> Self {
        return Card {
            number: Secret::new(number),
            color: Secret::new(color),
        };
    }
    fn hint_color(&mut self, value: &Color) {
        self.color.reveal_if(value);
    }

    fn hint_number(&mut self, value: &usize) {
        self.number.reveal_if(value);
    }

    fn cheat_string(&self) -> String {
        let num_str = self.number.peek().to_string();
        let color = self.color.peek();
        match color {
            Color::Red => format!("{}", num_str.color("red")),
            Color::Blue => format!("{}", num_str.color("blue")),
            Color::Green => format!("{}", num_str.color("green")),
            Color::White => format!("{}", num_str),
            Color::Yellow => format!("{}", num_str.color("yellow")),
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num_str = match self.number.get() {
            None => "?".to_string(),
            Some(x) => x.to_string(),
        };
        let string = match self.color.get() {
            None => format!("{}", num_str.color("cyan")),
            Some(x) => match x {
                Color::Red => format!("{}", num_str.color("red")),
                Color::Blue => format!("{}", num_str.color("blue")),
                Color::Green => format!("{}", num_str.color("green")),
                Color::White => format!("{}", num_str),
                Color::Yellow => format!("{}", num_str.color("yellow")),
            },
        };
        write!(f, "{}", string)
    }
}

pub struct Deck {
    cards: Vec<Card>,
    depleted: bool,
}

impl Deck {
    fn new() -> Self {
        let mut deque = Vec::<Card>::new();
        let colors = vec![
            Color::Red,
            Color::Blue,
            Color::Green,
            Color::White,
            Color::Yellow,
        ];
        let numbers: Vec<usize> = vec![1, 2, 3, 4, 5];
        for color in colors {
            for number in &numbers {
                let copies = match number {
                    1 => 3,
                    2 => 2,
                    3 => 2,
                    4 => 2,
                    5 => 1,
                    _ => 0,
                };
                for _ in 0..copies {
                    deque.push(Card::new(color.clone(), number.clone()));
                }
            }
        }
        deque.shuffle(&mut thread_rng());
        Deck {
            cards: deque,
            depleted: false,
        }
    }
    fn get_card(&mut self) -> Card {
        if self.cards.len() == 1 {
            self.depleted = true
        }
        self.cards.pop().unwrap()
    }

    fn is_empty(&self) -> &bool {
        &self.depleted
    }

    pub fn print_deck(&self) {
        println!(
            "{}",
            self.cards
                .iter()
                .map(|card| card.cheat_string())
                .intersperse(" ".to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }
}

pub struct Game {
    pub deck: Deck,
    pub hints: usize,
    max_hints: usize,
    lives: usize,
    played: HashMap<Color, usize>,
    discarded: HashMap<Color, HashMap<usize, usize>>,
    discard_limits: HashMap<Color, HashMap<usize, usize>>,
    perfection: bool,
    pub ended: bool,
}

fn init_discard_limits() -> HashMap<Color, HashMap<usize, usize>> {
    let mut out = HashMap::<Color, HashMap<usize, usize>>::new();
    let default = HashMap::from([(1, 3), (2, 2), (3, 2), (4, 2), (5, 1)]);
    let colors = vec![
        Color::Red,
        Color::Blue,
        Color::Green,
        Color::White,
        Color::Yellow,
    ];

    for color in colors {
        out.insert(color, default.clone());
    }

    out
}

impl Game {
    pub fn new(perfection: bool) -> Self {
        let deck = Deck::new();
        let hints = 7 as usize;
        let max_hints = 7 as usize;
        let lives = 3 as usize;
        let played = HashMap::<Color, usize>::new();
        let discarded = HashMap::<Color, HashMap<usize, usize>>::new();
        let discard_limits = init_discard_limits();
        let ended = false;

        Game {
            deck,
            hints,
            max_hints,
            lives,
            played,
            discarded,
            discard_limits,
            perfection,
            ended,
        }
    }
    fn play(&mut self, card: Card) -> bool {
        let color: Color = card.color.peek().clone();
        if self.is_valid_play(&card) {
            self.played
                .entry(color)
                .and_modify(|number| *number += 1)
                .or_insert(1);
            if self.played.values().all(|x| *x == 5) {
                self.end_game("All colors completed");
            };
            true
        } else {
            self.lives -= 1;
            if self.lives == 0 {
                self.end_game("Ran out of lives");
            };
            self.drop_card(card);
            false
        }
    }

    pub fn token_string(&self) -> String {
        let mut output = String::new();
        for _ in 0..self.hints {
            output = format!("{} {}", output, "●".blue());
        }
        for _ in self.hints..self.max_hints {
            output = format!("{} {}", output, "●".white());
        }
        output = format!("{} {}", output, " ");

        for _ in 0..self.lives {
            output = format!("{} {}", output, "●".red());
        }
        for _ in self.lives..3 {
            output = format!("{} {}", output, "●".white());
        }

        output
    }

    pub fn played_string(&self) -> String {
        let mut output = String::new();
        let colors = vec![
            (Color::Green, "green"),
            (Color::Blue, "blue"),
            (Color::Yellow, "yellow"),
            (Color::White, "white"),
            (Color::Red, "red"),
        ];
        for (color, fmt_str) in colors {
            let played = self.played.get(&color);
            let next = match played {
                Some(x) => x.to_string().color(fmt_str),
                None => "X".color(fmt_str),
            };
            output = format!("{} {}", output, next);
        }

        output
    }

    pub fn discarded_strings(&self) -> Vec<String> {
        let mut output = Vec::<String>::new();
        let colors = vec![
            (Color::Green, "green"),
            (Color::Blue, "blue"),
            (Color::Yellow, "yellow"),
            (Color::White, "white"),
            (Color::Red, "red"),
        ];
        for (color, fmt_str) in colors {
            let mut string = String::new();
            let discards = self.discarded.get(&color);
            if discards.is_none() {
                continue;
            }
            let mut values = discards.unwrap().iter().collect::<Vec<(&usize, &usize)>>();
            values.sort_by_key(|x| x.0);
            for (num, cnt) in values {
                let danger = self.check_dangerous(&color, num, cnt);
                let mut addition = num.to_string().repeat(*cnt).color(fmt_str);
                if danger {
                    addition = addition.underline();
                }
                string = format!("{} {}", string, addition);
            }
            output.push(string)
        }
        output
    }

    pub fn check_dangerous(&self, color: &Color, number: &usize, count: &usize) -> bool {
        if self.played.get(color).unwrap_or(&0) > number {
            false
        } else if count + 1 < self.discard_limits[color][number] {
            false
        } else {
            true
        }
    }

    fn drop_card(&mut self, card: Card) {
        let color: Color = card.color.peek().clone();
        let number = card.number.peek().clone();
        self.discarded
            .entry(color.clone())
            .or_default()
            .entry(number)
            .and_modify(|number| *number += 1)
            .or_insert(1);

        if self.perfection
            && self.discarded[&color][&number] == self.discard_limits[&color][&number]
        {
            self.end_game("Hit discard limit with perfection enabled.")
        }
    }
    fn discard(&mut self, card: Card) {
        if self.hints < self.max_hints {
            self.hints += 1;
        }
        self.drop_card(card)
    }

    fn is_valid_play(&self, card: &Card) -> bool {
        let color = card.color.peek();
        let number = card.number.peek();
        match self.played.get(color) {
            None => number.clone() == 1 as usize,
            Some(x) => number.clone() == x + 1,
        }
    }

    pub fn end_game(&mut self, reason: &str) {
        self.ended = true;
        println!("Game Over: {}", reason);
    }
}

pub struct Player {
    hand: VecDeque<Card>,
    pub hand_size: usize,
}

impl Player {
    pub fn new(game: &mut Game, hand_size: usize) -> Self {
        let hand = VecDeque::<Card>::new();
        let mut player = Player { hand, hand_size };
        for _ in 0..hand_size {
            player.draw(game);
        }
        player
    }
    fn draw(&mut self, game: &mut Game) {
        self.hand.push_back(game.deck.get_card());
    }

    pub fn play(&mut self, card_index: usize, game: &mut Game) {
        game.play(self.hand.remove(card_index).unwrap());
        self.draw(game);
    }

    pub fn discard(&mut self, card_index: usize, game: &mut Game) {
        game.discard(self.hand.remove(card_index).unwrap());
        self.draw(game);
    }

    pub fn get_hand_string(&self) -> String {
        self.hand
            .iter()
            .map(|card| card.to_string())
            .intersperse(" ".to_string())
            .collect()
    }

    pub fn peak_hand_string(&self) -> String {
        self.hand
            .iter()
            .map(|card| card.cheat_string())
            .intersperse(" ".to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn get_color_hint(&mut self, color: Color) {
        self.hand
            .iter_mut()
            .for_each(|card| card.hint_color(&color));
    }

    pub fn get_number_hint(&mut self, number: usize) {
        self.hand
            .iter_mut()
            .for_each(|card| card.hint_number(&number));
    }
}
