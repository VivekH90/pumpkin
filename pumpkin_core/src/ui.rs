use std::io::{self, Write};
use std::process::Command;

use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
};


pub fn start() {

    enable_raw_mode().unwrap();

    let mut cursor_num: usize = 0;


    // Generate logo
    let output = Command::new("bash")
        .arg("-c")
        .arg("figlet PUMPKIN | lolcat -f")
        .output()
        .expect("Failed to generate logo");


    // Fix newline behaviour in raw mode
    let logo = String::from_utf8_lossy(&output.stdout)
        .replace("\n", "\r\n");


    let actions = vec![
        "Watch Anime",
        "Open Workspace",
        "AI Browsing",
        "Add More Actions",
    ];


    loop {

        // Clear previous frame and return cursor to top-left
        execute!(
            io::stdout(),
            Clear(ClearType::All),
            MoveTo(0, 0)
        )
        .unwrap();


        // Draw logo
        print!("{}", logo);


        print!("\r\n");
        print!("What do you want to do now?\r\n");
        print!("\r\n");


        // Draw actions
        for (index, action) in actions.iter().enumerate() {

            let cursor = if index == cursor_num {
                ">"
            } else {
                " "
            };

            print!("{} {}\r\n", cursor, action);
        }


        io::stdout().flush().unwrap();


        // Wait for keyboard event
        let event = read().unwrap();


        if let Event::Key(key) = event {

            match key.code {

                // Down arrow
                KeyCode::Down => {

                    cursor_num += 1;
                    cursor_num %= actions.len();

                }


                // Up arrow
                KeyCode::Up => {

                    if cursor_num > 0 {
                        cursor_num -= 1;
                    }

                }


                // Exit
                KeyCode::Esc => {
                    break;
                }


                _ => {}

            }
        }
    }


    // Restore terminal
    disable_raw_mode().unwrap();

}