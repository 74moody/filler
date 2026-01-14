use std::io::{self, BufRead, Write};
use std::thread::sleep;
use std::time::Duration;

mod game;
mod parser;
mod scoring;
mod solver;
mod types;
mod utils;

use types::{Player, Pos};

fn main() {
    let stdin = io::stdin();
    let mut r = stdin.lock();

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    // Read handshake
    let mut first = String::new();
    let _ = r.read_line(&mut first);
    let me = Player::from_handshake(&first).unwrap_or(Player::P1);

    loop {
        match parser::read_round(&mut r, me) {
            Ok(Some(round)) => {
                let mv = solver::solve(&round.board, &round.piece, round.me)
                    .unwrap_or(Pos { x: 0, y: 0 });
                let _ = writeln!(out, "{} {}", mv.x, mv.y);
                let _ = out.flush();
            }

            Ok(None) => {
                // DO NOT EXIT — engine may still send more data
                sleep(Duration::from_millis(1));
            }

            Err(_) => {
                // Still must answer to avoid timeout
                let _ = writeln!(out, "0 0");
                let _ = out.flush();
                sleep(Duration::from_millis(1));
            }
        }
    }
}
