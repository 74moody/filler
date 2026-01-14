pub fn parse_dims(line: &str) -> Option<(usize, usize)> {
    // "Anfield 20 15:" or "Piece 4 3:"
    let mut it = line.split_whitespace();
    let _tag = it.next()?;
    let w = it.next()?.parse::<usize>().ok()?;
    let h = it.next()?.trim_end_matches(':').parse::<usize>().ok()?;
    Some((w, h))
}

pub fn strip_row_prefix(line: &str) -> &str {
    // Engine rows look like: "000 ...."
    // Robust: take after first space
    if let Some(i) = line.find(' ') {
        &line[i + 1..]
    } else {
        line
    }
}
