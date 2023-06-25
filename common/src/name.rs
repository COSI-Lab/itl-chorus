//! User names are color + animal pairs like "green giraffe" or "blue elephant.
//! They are built from an enum of colors and an enum of animals.
//!
//! Names are randomly generated and will be unique.

use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[cfg(feature = "server")]
use rand::{distributions::Standard, prelude::*};

/// A color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Brown,
    Black,
    White,
}

impl Color {
    pub fn iter() -> impl Iterator<Item = Color> {
        use Color::*;

        [
            Red, Orange, Yellow, Green, Blue, Purple, Pink, Brown, Black, White,
        ]
        .iter()
        .copied()
    }
}

// Only implement distributions if the "rand" feature is enabled
#[cfg(feature = "server")]
impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0..=9) {
            0 => Color::Red,
            1 => Color::Orange,
            2 => Color::Yellow,
            3 => Color::Green,
            4 => Color::Blue,
            5 => Color::Purple,
            6 => Color::Pink,
            7 => Color::Brown,
            8 => Color::Black,
            9 => Color::White,
            _ => unreachable!(),
        }
    }
}

/// Animal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Animal {
    Ant,
    Bear,
    Cat,
    Dog,
    Elephant,
    Fox,
    Giraffe,
    Horse,
    Iguana,
    Jaguar,
    Kangaroo,
    Lion,
    Monkey,
    Newt,
    Octopus,
    Penguin,
    Quail,
    Rabbit,
    Snake,
    Tiger,
    Unicorn,
    Vulture,
    Whale,
    Xray,
    Yak,
    Zebra,
}

impl Animal {
    pub fn iter() -> impl Iterator<Item = Animal> {
        use Animal::*;

        [
            Ant, Bear, Cat, Dog, Elephant, Fox, Giraffe, Horse, Iguana, Jaguar, Kangaroo, Lion,
            Monkey, Newt, Octopus, Penguin, Quail, Rabbit, Snake, Tiger, Unicorn, Vulture, Whale,
            Xray, Yak, Zebra,
        ]
        .iter()
        .copied()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Color::*;

        match self {
            Red => write!(f, "Red"),
            Orange => write!(f, "Orange"),
            Yellow => write!(f, "Yellow"),
            Green => write!(f, "Green"),
            Blue => write!(f, "Blue"),
            Purple => write!(f, "Purple"),
            Pink => write!(f, "Pink"),
            Brown => write!(f, "Brown"),
            Black => write!(f, "Black"),
            White => write!(f, "White"),
        }
    }
}

#[cfg(feature = "server")]
impl Distribution<Animal> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Animal {
        match rng.gen_range(0..=25) {
            0 => Animal::Ant,
            1 => Animal::Bear,
            2 => Animal::Cat,
            3 => Animal::Dog,
            4 => Animal::Elephant,
            5 => Animal::Fox,
            6 => Animal::Giraffe,
            7 => Animal::Horse,
            8 => Animal::Iguana,
            9 => Animal::Jaguar,
            10 => Animal::Kangaroo,
            11 => Animal::Lion,
            12 => Animal::Monkey,
            13 => Animal::Newt,
            14 => Animal::Octopus,
            15 => Animal::Penguin,
            16 => Animal::Quail,
            17 => Animal::Rabbit,
            18 => Animal::Snake,
            19 => Animal::Tiger,
            20 => Animal::Unicorn,
            21 => Animal::Vulture,
            22 => Animal::Whale,
            23 => Animal::Xray,
            24 => Animal::Yak,
            25 => Animal::Zebra,
            _ => unreachable!(),
        }
    }
}

impl Display for Animal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Animal::*;

        match self {
            Ant => write!(f, "Ant"),
            Bear => write!(f, "Bear"),
            Cat => write!(f, "Cat"),
            Dog => write!(f, "Dog"),
            Elephant => write!(f, "Elephant"),
            Fox => write!(f, "Fox"),
            Giraffe => write!(f, "Giraffe"),
            Horse => write!(f, "Horse"),
            Iguana => write!(f, "Iguana"),
            Jaguar => write!(f, "Jaguar"),
            Kangaroo => write!(f, "Kangaroo"),
            Lion => write!(f, "Lion"),
            Monkey => write!(f, "Monkey"),
            Newt => write!(f, "Newt"),
            Octopus => write!(f, "Octopus"),
            Penguin => write!(f, "Penguin"),
            Quail => write!(f, "Quail"),
            Rabbit => write!(f, "Rabbit"),
            Snake => write!(f, "Snake"),
            Tiger => write!(f, "Tiger"),
            Unicorn => write!(f, "Unicorn"),
            Vulture => write!(f, "Vulture"),
            Whale => write!(f, "Whale"),
            Xray => write!(f, "Xray"),
            Yak => write!(f, "Yak"),
            Zebra => write!(f, "Zebra"),
        }
    }
}

/// A user name
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Name {
    pub color: Color,
    pub animal: Animal,
}

#[cfg(feature = "server")]
impl Distribution<Name> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Name {
        Name {
            color: rng.gen(),
            animal: rng.gen(),
        }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.color, self.animal)
    }
}
