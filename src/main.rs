use std::{io::{stdin, stdout, Write}, thread, time::{Duration, Instant}};

use rifegame::{entities::{rules::StandardRule, world::World}, values::grid::{Grid, WORLD_HEIGHT}};
use termion::{event::{Event, Key}, input::TermRead, raw::IntoRawMode};

const FPS: u64 = 5;
const INTERVAL_MILLIS: u64 = 1000 / FPS;
const INITIAL_GRID: [&str; WORLD_HEIGHT] = [
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "            oo oooooo           ",
    "            oo oooooo           ",
    "            oo                  ",
    "            oo     oo           ",
    "            oo     oo           ",
    "            oo     oo           ",
    "                   oo           ",
    "            oooooo oo           ",
    "            oooooo oo           ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
    "                                ",
];

fn main() {
    let grid = Grid::try_from(INITIAL_GRID).unwrap();
    let mut world = World::new(grid, StandardRule);

    let stdout = stdout();
    let mut screen = stdout.lock().into_raw_mode().unwrap();

    let stdin = stdin();

    write!(screen, "{}{}", termion::cursor::Hide, termion::clear::All).unwrap();
    screen.flush().unwrap();

    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn(move || {
        for event_result in stdin.events() {
            if let Ok(event) = event_result {
                tx.send(event).unwrap();
            }
        }
    });

    let mut prev_draw = Instant::now();
    loop {
        if Instant::now() - prev_draw > Duration::from_millis(INTERVAL_MILLIS) {
            world.update();
            world.draw(&mut screen);
            screen.flush().unwrap();
            prev_draw = Instant::now();
        }
        if let Ok(event) = rx.recv_timeout(Duration::from_millis(16)) {
            match event {
                Event::Key(Key::Char('q')) => break,
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    write!(screen, "{}", termion::cursor::Show).unwrap();
}
