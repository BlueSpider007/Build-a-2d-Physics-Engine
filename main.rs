use std::io::{self, Read};

// Pairs are (i, j) with i < j. Sort them and remove duplicates.
fn unique_sorted(mut p: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    p.sort();
    p.dedup();
    p
}

// Candidate pairs: bodies that share at least one grid cell.
fn grid_pairs(boxes: &[[f64; 4]], size: f64) -> Vec<(usize, usize)> {
    // TODO: for every body push (cell x, cell y, body) for each cell its box covers
    // (f64::floor, then `as i64`), sort the entries, push the pairs inside each run
    // of equal cells, then return unique_sorted(pairs).
    let mut cells: Vec<(i64, i64, usize)> = vec![];
    for i in 0..boxes.len() {
        for x in (boxes[i][0] / size).floor() as i64..=(boxes[i][2] / size).floor() as i64 {
            for y in (boxes[i][1] / size).floor() as i64..=(boxes[i][3] / size).floor() as i64 {
                cells.push((x, y, i))
            }
        }
    }
    cells.sort();
    let mut pairs: Vec<(usize, usize)> = vec![];
    for i in 1..cells.len() {
        if cells[i - 1].0 == cells[i].0 && cells[i - 1].1 == cells[i].1 {
            pairs.push((cells[i - 1].2, cells[i].2));
        }
    }
    unique_sorted(pairs)
}

// Pairs whose boxes overlap (touching counts).
fn sweep_pairs(boxes: &[[f64; 4]]) -> Vec<(usize, usize)> {
    // TODO: sort the body indices by x0 (ties by index). For each body walk the
    // later ones; stop when one starts right of the current body's x1; otherwise
    // compare the y intervals.
    let mut order: Vec<usize> = (0..boxes.len()).collect();
    order.sort_by(|&a, &b| boxes[a][0].total_cmp(&boxes[b][0]).then_with(|| a.cmp(&b)));
    let mut col = vec![];
    for i in 0..order.len() {
        let a = order[i];
        for j in (i + 1)..order.len() {
            let b = order[j];
            if boxes[b][0] > boxes[a][2] {
                break;
            }

            if boxes[a][1] <= boxes[b][3] && boxes[a][3] >= boxes[b][1] {
                col.push((a.min(b), a.max(b)))
            }
        }
    }
    unique_sorted(col)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut size = 1.0f64;
    let mut boxes: Vec<[f64; 4]> = Vec::new();
    for line in input.lines() {
        let p: Vec<&str> = line.split_whitespace().collect();
        if p.is_empty() {
            continue;
        }
        match p[0] {
            "CELL" => size = p[1].parse().unwrap(),
            "BODY" => {
                let mut b = [0.0f64; 4];
                for i in 0..4 {
                    b[i] = p[i + 1].parse().unwrap();
                }
                boxes.push(b);
            }
            "PAIRS" | "SWEEP" => {
                let n = boxes.len();
                let grid = p[0] == "PAIRS";
                let res = if grid {
                    grid_pairs(&boxes, size)
                } else {
                    sweep_pairs(&boxes)
                };
                let word = if grid { "candidates" } else { "overlaps" };
                println!(
                    "naive={} {}={}",
                    n * (n.saturating_sub(1)) / 2,
                    word,
                    res.len()
                );
                for pr in res {
                    println!("{} {}", pr.0, pr.1);
                }
            }
            _ => {}
        }
    }
}
