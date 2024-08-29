//! # Game Server Key Generator
//!
//! This program is a command-line tool that generates keys for game servers
//! for various Call of Duty titles. It interacts with an intermediate API
//! to process server information and generate the appropriate key.
//!
//! ## Supported Games
//!
//! - Call of Duty: World at War
//! - Call of Duty: Black Ops
//! - Call of Duty: Black Ops II
//! - Call of Duty: Modern Warfare 3
//!
//! ## Features
//!
//! - Interactive command-line interface
//! - Support for both Zombie and Multiplayer modes (where applicable)
//! - Integration with an intermediate API for key generation
//!
//! ## Usage
//!
//! The program will prompt the user for:
//! 1. Server name
//! 2. Game selection
//! 3. Game mode (Zombie or Multiplayer, depending on the game)
//!
//! After collecting this information, it sends a request to the intermediate API
//! and displays the generated key.

use reqwest;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct ServerInfo {
    server_name: String,
    mode: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ApiResponse {
    Success { key: String },
    Error { error: String },
}

fn print_logo() {
    // Clear the console
    print!("\x1B[2J\x1B[1;1H");
    println!("\x1b[31m .S_sSSs     .S    S.     sSSSSs        sdSS_SSSSSSbs    sSSs_sSSs      sSSs_sSSs    S.        sSSs ");
    println!(".SS~YS%%b   .SS    SS.   d%%%%SP        YSSS~S%SSSSSP   d%%SP~YS%%b    d%%SP~YS%%b   SS.      d%%SP ");
    println!("S%S   `S%b  S%S    S&S  d%S'                 S%S       d%S'     `S%b  d%S'     `S%b  S%S     d%S'   ");
    println!("S%S    S%S  S%S    d*S  S%S                  S%S       S%S       S%S  S%S       S%S  S%S     S%|    ");
    println!("S%S    d*S  S&S   .S*S  S&S                  S&S       S&S       S&S  S&S       S&S  S&S     S&S    ");
    println!("S&S   .S*S  S&S_sdSSS   S&S                  S&S       S&S       S&S  S&S       S&S  S&S     Y&Ss   ");
    println!("S&S_sdSSS   S&S~YSSY%b  S&S                  S&S       S&S       S&S  S&S       S&S  S&S     `S&&S  ");
    println!("S&S~YSSY    S&S    `S%  S&S sSSs             S&S       S&S       S&S  S&S       S&S  S&S       `S*S ");
    println!("S*S         S*S     S%  S*b `S%%             S*S       S*b       d*S  S*b       d*S  S*b        l*S ");
    println!("S*S         S*S     S&  S*S   S%             S*S       S*S.     .S*S  S*S.     .S*S  S*S.      .S*P ");
    println!("S*S         S*S     S&   SS_sSSS             S*S        SSSbs_sdSSS    SSSbs_sdSSS    SSSbs  sSS*S  ");
    println!("S*S         S*S     SS    Y~YSSY             S*S         YSSP~YSSY      YSSP~YSSY      YSSP  YSS'   ");
    println!("SP          SP                               SP                                                     ");
    println!("Y           Y                                Y                                                      \x1b[0m");

    println!("\n┌─────────────────────────────────────────────────────────────────────────────────────────────┐");
    println!("│                                 \x1b[1mPlutonium Key Generator\x1b[0m                                     │");
    println!("├─────────────────────────────────────────────────────────────────────────────────────────────┤");
    println!("│ \x1b[1;33mVersion\x1b[0m     │ 1.1.0                                                                         │");
    println!("│ \x1b[1;33mAuthor\x1b[0m      │ Sterbweise                                                                    │");
    println!("│ \x1b[1;33mDescription\x1b[0m │ A command-line tool for generating keys for various Plutonium game servers.   │");
    println!("│ \x1b[1;33mRepository\x1b[0m  │ https://github.com/sterbweise/plutonium-key-generator                         │");
    println!("│ \x1b[1;33mLicense\x1b[0m     │ MIT                                                                           │");
    println!("│ \x1b[1;31mWarning\x1b[0m     │ Please don't use this tool if you own the game. Get the key by the classic    │");
    println!("│             │ method on https://platform.plutonium.pw/serverkeys                            │");
    println!("│ \x1b[1;34mInformation\x1b[0m │ The generated key will be valid for only 48 hours                             │");
    println!("└─────────────────────────────────────────────────────────────────────────────────────────────┘\n");
}

fn get_user_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_logo();
    println!("Supported Games:");
    println!("- Call of Duty: World at War \x1b[33m[Not supported yet by the api]\x1b[0m");
    println!("- Call of Duty: Black Ops \x1b[32m[Supported]\x1b[0m");
    println!("- Call of Duty: Black Ops II \x1b[32m[Supported]\x1b[0m");
    println!("- Call of Duty: Modern Warfare 3 \x1b[33m[Not supported yet by the api]\x1b[0m");
    println!("\nPress ENTER to start...");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    // Ask for the server name
    let server_name = loop {
        print_logo();
        let input = get_user_input("\n\x1b[1;36m[Server Name]\x1b[0m Enter the name for your server (max 50 characters): ")?;
        let trimmed = input.trim().to_string();
        if trimmed.len() <= 50 {
            break trimmed;
        } else {
            println!("Server name is too long. Please enter a name with 50 characters or less.");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    };

    // Ask for the game
    let games = [
        ("1", "Black Ops 1 \x1b[32m[Supported]\x1b[0m", "bo1"),
        ("2", "Black Ops 2 \x1b[32m[Supported]\x1b[0m", "bo2"),
        ("3", "Modern Warfare 3 \x1b[31m[Not supported]\x1b[0m", "mw3"),
        ("4", "World at War \x1b[31m[Not supported]\x1b[0m", "waw"),
    ];

    let game = loop {
        print_logo();
        println!("Select the game:");
        for (num, name, _) in &games {
            println!("{}. {}", num, name);
        }

        let game_choice = get_user_input("\n\x1b[1;36m[Game Selection]\x1b[0m Enter your choice (1-4): ")?;
        
        if let Some((_, _, code)) = games.iter().find(|(num, _, _)| num == &game_choice.trim()) {
            break *code;
        } else {
            println!("Unrecognized game choice. Please try again.");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    };

    // Ask for the mode
    let modes = match game {
        "mw3" => vec![("2", "Multiplayer", "iw5mp")],
        _ => vec![("1", "Zombie", "zm"), ("2", "Multiplayer", "mp")],
    };

    let mode = loop {
        print_logo();
        println!("Select the mode:");
        for (num, name, _) in &modes {
            println!("{}. {}", num, name);
        }

        if game == "mw3" {
            println!("Note: Modern Warfare 3 doesn't have a Zombie mode.");
        }

        let mode_choice = get_user_input(&format!("\n\x1b[1;36m[Mode Selection]\x1b[0m Enter your choice ({}):", 
            if modes.len() == 1 { "2" } else { "1-2" }))?;
        
        if let Some((_, _, code)) = modes.iter().find(|(num, _, _)| num == &mode_choice.trim()) {
            break match game {
                "bo1" => format!("t5{}", code),
                "bo2" => format!("t6{}", code),
                "waw" => format!("t4{}", code),
                _ => code.to_string(),
            };
        } else {
            println!("Unrecognized mode choice. Please try again.");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    };

    // Create ServerInfo object
    let server_info = ServerInfo {
        server_name,
        mode: mode.to_string(),
    };

    // Send request to the intermediate API
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://api.sterbweise.dev/plutonium-key-generator")
        .json(&server_info)
        .send()?;

    // Process the response
    match res.json::<ApiResponse>() {
        Ok(api_response) => {
            match api_response {
                ApiResponse::Success { key } => {
                    print_logo();
                    println!("Generated key: {}", key);
                    println!("\x1b[1;31mAttention: The key will only be valid for 48 hours.\x1b[0m");
                    println!("For more information, please check the information section in the Git repository.");
                },
                ApiResponse::Error { error } => {
                    print_logo();
                    println!("\x1b[1;31mError:\x1b[0m {}.\nDo you want to try again? (Y/N)", error);
                    let mut retry = String::new();
                    io::stdin().read_line(&mut retry)?;
                    if retry.trim().to_lowercase() == "y" {
                        return main();  // Restart the program
                    } else {
                        return Ok(());
                    }
                }
            }
        },
        Err(_err) => {
            print_logo();
            println!("Failed to parse the response. Please try again later.");
        }
    }
    println!("\nPress ENTER to exit the program...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}