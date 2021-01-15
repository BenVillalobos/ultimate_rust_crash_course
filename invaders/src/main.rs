use invaders::frame::Drawable;
use std::error::Error;
use crossterm::cursor::Show;
use rusty_audio::Audio;
use std::io;
use std::time::{Duration, Instant};
use invaders::{frame, render, player::Player, enemy::Enemy};
use crossterm::{terminal, ExecutableCommand};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide};
use crossterm::event::{Event, KeyCode};
use crossterm::{event};
use std::sync::mpsc;
use std::thread;

// Big lesson learned. Couldn't use ? before because I didn't mark main returning a result. ? only applies to functions that return
// either result or option.
// Then the compiler threw an error on main, but it was because main wasn't returning Ok(()) at the end.

// Another big lesson learned:
// main.rs can't `mod lib` WHILE ALSO doing: invaders::{frame, render}
// this is because it would essentially be importing the types twice.

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "sound/explode.wav");
    audio.add("lose", "sound/lose.wav");
    audio.add("move", "sound/move.wav");
    audio.add("pew", "sound/pew.wav");
    audio.add("startup", "sound/startup.wav");
    audio.add("win", "sound/win.wav");

    audio.play("startup");

    let mut stdout = io::stdout();
    let _ = terminal::enable_raw_mode();
    
    // The game exists in an alternate screen
    let _ = stdout.execute(EnterAlternateScreen);
    let _ = stdout.execute(Hide);

    // RENDER LOOP (separate thread)
    // crossbeam channels are higher perf and more features
    let (rtx, rrx) = mpsc::channel();
    let r_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();

        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let curr_frame = match rrx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };

            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut instant = Instant::now();
    let mut curr_frame;
    let mut player = Player::new();
    let enemy = Enemy::new();
    

    // GAMEPLAY LOOP
    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        curr_frame = frame::new_frame();
        
        //input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('Q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => continue 'gameloop
                }
            }
        }

        // UPDATE
        player.update(delta);

        // DRAW & RENDER
        player.draw(&mut curr_frame);
        enemy.draw(&mut curr_frame);
        let _ = rtx.send(curr_frame);
        thread::sleep(Duration::from_millis(1)); // main loop runs faster than render loop

    }

    // CLEANUP

    drop(rtx); // may not be necessary in newer rust
    r_handle.join().unwrap();

    audio.wait();
    let _ = stdout.execute(Show);
    let _ = stdout.execute(LeaveAlternateScreen);
    let _ = terminal::disable_raw_mode();

    Ok(())
}
