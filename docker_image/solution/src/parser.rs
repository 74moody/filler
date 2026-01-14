use std::io::{self, BufRead};

use crate::types::{Anfield, Piece, Player, Pos, Round};
use crate::utils::{parse_dims, strip_row_prefix};

fn read_line_opt<R: BufRead>(r: &mut R, buf: &mut String) -> io::Result<bool> {
    buf.clear();
    Ok(r.read_line(buf)? != 0)
}

fn build_piece(w: usize, h: usize, raw: Vec<Vec<char>>) -> Piece {
    let mut filled: Vec<Pos> = Vec::new();
    let mut sum_dx: i64 = 0;
    let mut sum_dy: i64 = 0;

    for (y, row) in raw.iter().enumerate().take(h) {
        for (x, &c) in row.iter().enumerate().take(w) {
            if c == 'O' || c == '#' {
                filled.push(Pos {
                    x: x as i32,
                    y: y as i32,
                });
                sum_dx += x as i64;
                sum_dy += y as i64;
            }
        }
    }

    Piece {
        filled,
        sum_dx,
        sum_dy,
    }
    
}

/// Reads one full round (Anfield + Piece).
/// Returns Ok(None) on clean EOF.
pub fn read_round<R: BufRead>(r: &mut R, me: Player) -> io::Result<Option<Round>> {
    let mut line = String::new();

    // resync to "Anfield"
    loop {
        if !read_line_opt(r, &mut line)? {
            return Ok(None);
        }
        if line.starts_with("Anfield ") {
            break;
        }
        // ignore everything else (handshake already read in main)
    }

    let (w, h) = match parse_dims(line.trim_end()) {
        Some(v) => v,
        None => return Ok(None),
    };

    // skip header "    0123..."
    if !read_line_opt(r, &mut line)? {
        return Ok(None);
    }

    let mut cells: Vec<Vec<char>> = Vec::with_capacity(h);

    for _ in 0..h {
        if !read_line_opt(r, &mut line)? {
            return Ok(None);
        }
        let row_src = strip_row_prefix(line.trim_end());
        let mut row: Vec<char> = row_src.chars().take(w).collect();
        while row.len() < w {
            row.push('.');
        }
        cells.push(row);
    }

    // find "Piece"
    loop {
        if !read_line_opt(r, &mut line)? {
            return Ok(None);
        }
        if line.starts_with("Piece ") {
            break;
        }
    }

    let (pw, ph) = match parse_dims(line.trim_end()) {
        Some(v) => v,
        None => return Ok(None),
    };

    let mut raw_piece: Vec<Vec<char>> = Vec::with_capacity(ph);
    for _ in 0..ph {
        if !read_line_opt(r, &mut line)? {
            return Ok(None);
        }
        let mut row: Vec<char> = line.trim_end().chars().take(pw).collect();
        while row.len() < pw {
            row.push('.');
        }
        raw_piece.push(row);
    }

    let board = Anfield { w, h, cells };
    let piece = build_piece(pw, ph, raw_piece);

    Ok(Some(Round { me, board, piece }))
}
