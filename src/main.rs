use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rust_invaders::{
    frame::{self, new_frame, Drawable},
    player::Player,
    render,
};
use rusty_audio::Audio;
use std::{error::Error, io, sync::mpsc, thread, time::Duration};

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

    // Render loop in separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };

            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut player = Player::new();

    // Game Loop
    'gapeloop: loop {
        // Per-frame init

        let mut curr_frame = new_frame();
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_self(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gapeloop;
                    }
                    _ => {}
                }
            }
        }

        // Draw and render
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
