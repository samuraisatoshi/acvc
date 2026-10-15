pub fn unified(before: &str, after: &str) -> String {
    if before == after {
        return String::from("unchanged");
    }
    let left: Vec<&str> = before.lines().collect();
    let right: Vec<&str> = after.lines().collect();
    if left.len() > 800 || right.len() > 800 {
        return format!("changed lines {} -> {}", left.len(), right.len());
    }
    let table = lcs(&left, &right);
    let mut out = String::new();
    emit(&left, &right, &table, 0, 0, &mut out);
    out
}

fn lcs(left: &[&str], right: &[&str]) -> Vec<Vec<u16>> {
    let mut table = vec![vec![0u16; right.len() + 1]; left.len() + 1];
    for row in (0..left.len()).rev() {
        for col in (0..right.len()).rev() {
            table[row][col] = if left[row] == right[col] {
                table[row + 1][col + 1].saturating_add(1)
            } else {
                table[row + 1][col].max(table[row][col + 1])
            };
        }
    }
    table
}

fn emit(
    left: &[&str],
    right: &[&str],
    table: &[Vec<u16>],
    row: usize,
    col: usize,
    out: &mut String,
) {
    if row == left.len() && col == right.len() {
        return;
    }
    if row < left.len() && col < right.len() && left[row] == right[col] {
        out.push_str("  ");
        out.push_str(left[row]);
        out.push('\n');
        emit(left, right, table, row + 1, col + 1, out);
    } else if col < right.len() && (row == left.len() || table[row][col + 1] >= table[row + 1][col])
    {
        out.push_str("+ ");
        out.push_str(right[col]);
        out.push('\n');
        emit(left, right, table, row, col + 1, out);
    } else if row < left.len() {
        out.push_str("- ");
        out.push_str(left[row]);
        out.push('\n');
        emit(left, right, table, row + 1, col, out);
    }
}
