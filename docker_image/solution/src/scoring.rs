use crate::types::{Piece, Pos};

/// Score tuple: (frontier_hits, dist2) where:
/// - higher frontier_hits is better
/// - lower dist2 is better
#[derive(Clone, Copy, Debug)]
pub struct Score {
    pub hits: i32,
    pub dist2: i64,
}

/// Integer distance^2 between placed piece centroid and target centroid (both fixed-point).
///
/// placed centroid:
///   px_num = top_left.x * n + piece.sum_dx
///   py_num = top_left.y * n + piece.sum_dy
/// target centroid:
///   tx_num / tden, ty_num / tden
///
/// Compare squared distance without floating:
///   dx_num = px_num * tden - tx_num * n
pub fn score_move(
    piece: &Piece,
    top_left: Pos,
    frontier: &[Vec<bool>],
    target_fp: Option<(i64, i64, i64)>,
) -> Score {
    let n = piece.filled.len() as i64;
    if n == 0 {
        return Score { hits: -1, dist2: i64::MAX / 4 };
    }

    // frontier hits
    let mut hits: i32 = 0;
    for off in &piece.filled {
        let x = (top_left.x + off.x) as usize;
        let y = (top_left.y + off.y) as usize;
        if frontier[y][x] {
            hits += 1;
        }
    }

    // distance^2 to target
    let (tx, ty, tden) = match target_fp {
        Some(v) => v,
        None => {
            // no enemy -> use board center-ish default (0,0) with denom 1;
            // solver will still behave well because hits dominates early game
            (0, 0, 1)
        }
    };

    let px_num = (top_left.x as i64) * n + piece.sum_dx;
    let py_num = (top_left.y as i64) * n + piece.sum_dy;

    let dx_num = px_num * tden - tx * n;
    let dy_num = py_num * tden - ty * n;

    let dist2 = dx_num * dx_num + dy_num * dy_num;

    Score { hits, dist2 }
}

/// Returns true if `a` is better than `b`.
#[inline]
pub fn better(a: Score, b: Score) -> bool {
    a.hits > b.hits || (a.hits == b.hits && a.dist2 < b.dist2)
}
