use std::collections::HashSet;

use crate::game::{build_opp_frontier, collect_my_cells, enemy_centroid_fp};
use crate::scoring::{better, score_move, Score};
use crate::types::{Anfield, Piece, Player, Pos};

fn is_valid(board: &Anfield, piece: &Piece, top_left: Pos, me: Player) -> bool {
    let mut overlap = 0;

    for off in &piece.filled {
        let x = top_left.x + off.x;
        let y = top_left.y + off.y;

        if x < 0 || y < 0 || (x as usize) >= board.w || (y as usize) >= board.h {
            return false;
        }

        let c = board.cells[y as usize][x as usize];
        if c == '.' {
            continue;
        }
        if me.is_me(c) {
            overlap += 1;
            if overlap > 1 {
                return false;
            }
        } else {
            // any other non-empty is enemy/blocked
            return false;
        }
    }

    overlap == 1
}

/// Generate candidates anchored on my existing cells (fast & avoids scanning full grid).
fn candidates(_board: &Anfield, piece: &Piece, my_cells: &[Pos]) -> Vec<Pos> {
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut out: Vec<Pos> = Vec::new();

    for &m in my_cells {
        for off in &piece.filled {
            let tl = Pos { x: m.x - off.x, y: m.y - off.y };
            if seen.insert((tl.x, tl.y)) {
                out.push(tl);
            }
        }
    }

    out
}

pub fn solve(board: &Anfield, piece: &Piece, me: Player) -> Option<Pos> {
    if piece.filled.is_empty() {
        return None;
    }

    let my_cells = collect_my_cells(board, me);
    if my_cells.is_empty() {
        return None;
    }

    let frontier = build_opp_frontier(board, me);
    let target = enemy_centroid_fp(board, me);

    let mut best_pos: Option<Pos> = None;
    let mut best_score: Score = Score { hits: -1, dist2: i64::MAX / 4 };

    for tl in candidates(board, piece, &my_cells) {
        if !is_valid(board, piece, tl, me) {
            continue;
        }
        let s = score_move(piece, tl, &frontier, target);
        if best_pos.is_none() || better(s, best_score) {
            best_score = s;
            best_pos = Some(tl);
        }
    }

    best_pos
}
