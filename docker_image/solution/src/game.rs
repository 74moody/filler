use crate::types::{Anfield, Player, Pos};

/// Build "opponent frontier":
/// empty cells that are 4-adjacent to ANY opponent cell.
pub fn build_opp_frontier(board: &Anfield, me: Player) -> Vec<Vec<bool>> {
    let h = board.h;
    let w = board.w;
    let mut f = vec![vec![false; w]; h];

    for y in 0..h {
        for x in 0..w {
            if board.cells[y][x] != '.' {
                continue;
            }
            let mut adj = false;

            if y > 0 && me.is_opp(board.cells[y - 1][x]) { adj = true; }
            if !adj && y + 1 < h && me.is_opp(board.cells[y + 1][x]) { adj = true; }
            if !adj && x > 0 && me.is_opp(board.cells[y][x - 1]) { adj = true; }
            if !adj && x + 1 < w && me.is_opp(board.cells[y][x + 1]) { adj = true; }

            f[y][x] = adj;
        }
    }

    f
}

/// Collect positions of all my cells (both upper/lower).
pub fn collect_my_cells(board: &Anfield, me: Player) -> Vec<Pos> {
    let mut out = Vec::new();
    for y in 0..board.h {
        for x in 0..board.w {
            if me.is_me(board.cells[y][x]) {
                out.push(Pos { x: x as i32, y: y as i32 });
            }
        }
    }
    out
}

/// Enemy centroid target in fixed-point numerator form.
/// Returns (tx_num, ty_num, denom). denom = count of enemy cells (>=1),
/// so target position is (tx_num/denom, ty_num/denom).
pub fn enemy_centroid_fp(board: &Anfield, me: Player) -> Option<(i64, i64, i64)> {
    let mut sx: i64 = 0;
    let mut sy: i64 = 0;
    let mut cnt: i64 = 0;

    for y in 0..board.h {
        for x in 0..board.w {
            let c = board.cells[y][x];
            if me.is_opp(c) {
                sx += x as i64;
                sy += y as i64;
                cnt += 1;
            }
        }
    }

    if cnt > 0 { Some((sx, sy, cnt)) } else { None }
}
