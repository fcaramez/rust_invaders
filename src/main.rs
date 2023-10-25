use crossterm::{
    cursor::{Hide, Show},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use std::{error::Error, fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    // Load all initial audio Files into the program

    let mut audio = Audio::new();
    for item in &["explode", "lose", "move", "pew", "startup", "win"] {
        audio.add(item, &format!("sounds/{}.wav", item));
    }
    audio.play("startup");

    // Access the Terminal in raw mode to extract all keyboard inputs

    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;

    // VIM and EMACS work like this, this will open the second available screen on the terminal so when you leave the game, you stay on the same place
    stdout.execute(EnterAlternateScreen)?;

    // Hide the cursor
    stdout.execute(Hide)?;

    // Cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
