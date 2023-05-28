/// This module contains functions that are used in operations within main.rs and other binaries.

/// This function prompts the user for input and returns the input as a string.
pub fn prompt(msg: &str) -> String {
    println!("{}\n", msg);
    let mut input = String::new();
    print!(">> ");

    use std::io::{self, Write};
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

/// This function parses a string to an integer.
pub fn parse_to_int(input: &str, default: i32) -> i32 {
    match input.parse::<i32>() {
        Ok(n) => n,
        Err(_) => default,
    }
}

/// This function parses a string to a float.
pub fn parse_to_float(input: &str, default: f32) -> f32 {
    match input.parse::<f32>() {
        Ok(n) => n,
        Err(_) => default,
    }
}

/// This function prompts the user to choose from a menu and returns the choice as a string.
pub fn choose_from_menu(choices: Vec<String>, msg: &str) -> String {
    println!("Please choose an option:\n");

    let choice_len = choices.len().try_into().expect("Too many choices.");

    for (i, choice) in choices.iter().enumerate() {
        println!("{}. {}", i + 1, choice);
    }

    println!();

    let selection = loop {
        let choice = prompt(msg);
        let choice = parse_to_int(&choice, 0);

        if choice > 0 && choice <= choice_len {
            break (choice - 1) as usize;
        } else {
            println!("Invalid choice.");
        }
    };

    choices.get(selection).unwrap().to_string()
}

use crate::{Abnormalities, Curvature, FromString, GetVariants, InnerUser, Size, SizeType, ID};

/// This function prompts the user to choose from a menu consisting of the variants of the type `T` and returns the `T` variant chosen.
pub fn input<T: GetVariants + FromString>(message: &str) -> T {
    let choice = choose_from_menu(T::get_variants(), message);
    let variant = T::from_string(&choice);
    clear_screen();
    variant
}

/// This function is responsible for getting the user's [ID] parameters.
pub fn get_user() -> ID {
    let user_type = choose_from_menu(ID::get_variants(), "ID type:");
    let user = match user_type.as_str() {
        "Anonymous" => ID::Anonymous,
        "User" => {
            clear_screen();
            let name = prompt("User Name:");
            clear_screen();
            let discord_name = prompt("Discord Name:");
            ID::User(InnerUser { name, discord_name })
        }
        _ => panic!("Invalid user type"),
    };
    clear_screen();
    user
}

/// This function is responsible for getting the cock [Size] parameters.
pub fn get_size() -> Size {
    let size_type = choose_from_menu(SizeType::get_variants(), "Inches or Centimeters?");
    let length = parse_to_float(prompt("Cock length:").as_str(), 0.0);
    let girth = parse_to_float(prompt("Cock girth:").as_str(), 0.0);

    let size = match size_type.as_str() {
        "Inches" => Size::from_in(length, girth),
        "Centimeters" => Size::from_cm(length, girth),
        _ => panic!("Invalid size type"),
    };
    clear_screen();
    size
}

/// Fn to clear terminal and reset cursor position
pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

use crate::{CockHandler, CockStruct, Shape};

/// Used to prompt a user for each necessary cock attribute and construct a CockStruct from them.
pub fn cock_handler_build() -> CockHandler {
    let id = get_user();
    let size = get_size();
    let aesthetic = input("Choose aesthetic:");
    let balls = input("Choose ball size:");
    let shape = {
        let in_shape = input("Choose cock shape:");
        match in_shape {
            Shape::Other(_) => {
                let val = prompt("Define the shape:");
                clear_screen();
                Shape::Other(val)
            }
            _ => in_shape,
        }
    };
    let curvature = {
        let in_curv = input("Choose cock curvature:");
        match in_curv {
            Curvature::Other(_) => {
                let val = prompt("Define the curvature:");
                clear_screen();
                Curvature::Other(val)
            }
            _ => in_curv,
        }
    };
    let circumcision = input("Choose cirumcision status:");
    let veininess = input("Choose veininess level:");
    let abnormalities = {
        let in_abnor = input("Choose cock abnormalities:");
        match in_abnor {
            Abnormalities::Major(_) => {
                let val = prompt("Define the major abnormalities:");
                clear_screen();
                Abnormalities::Major(val)
            }
            Abnormalities::Minor(_) => {
                let val = prompt("Define the minor abnormalities:");
                clear_screen();
                Abnormalities::Minor(val)
            }
            _ => in_abnor,
        }
    };

    let cock = CockStruct {
        size,
        aesthetic,
        balls,
        shape,
        curvature,
        circumcision,
        veininess,
        abnormalities,
    };

    CockHandler { id, cock }
}
