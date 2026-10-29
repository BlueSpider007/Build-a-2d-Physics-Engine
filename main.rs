use std::io::{self, Read};

fn f4(x: f64) -> String {
    let s = format!("{:.4}", x);
    if s == "-0.0000" {
        "0.0000".to_string()
    } else {
        s
    }
}

// One frame: returns (steps, new bank).
fn advance(acc: i64, _elapsed: i64, _dt: i64, _max_frame: i64) -> (i64, i64) {
    // TODO: clamp elapsed to max_frame and add it to the bank (acc),
    // then spend the bank in whole steps of dt. Keep the leftover.
    let mut elapsed = _elapsed;
    let mut acc = acc;
    if elapsed > _max_frame {
        elapsed = _max_frame
    }
    acc += elapsed;
    let mut steps = 0;
    while acc >= _dt {
        acc -= _dt;
        steps += 1;
    }
    (steps, acc)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut dt: i64 = 1;
    let mut max_frame: i64 = 0;
    let mut acc: i64 = 0;
    let mut total: i64 = 0;
    for line in input.lines() {
        let p: Vec<&str> = line.split_whitespace().collect();
        if p.is_empty() {
            continue;
        }
        match p[0] {
            "DT" => dt = p[1].parse().unwrap(),
            "MAX_FRAME" => max_frame = p[1].parse().unwrap(),
            "FRAME" => {
                let e: i64 = p[1].parse().unwrap();
                let (steps, bank) = advance(acc, e, dt, max_frame);
                acc = bank;
                total += steps * dt;
                println!(
                    "steps={} t={} acc={} alpha={}",
                    steps,
                    total,
                    acc,
                    f4(acc as f64 / dt as f64)
                );
            }
            _ => {}
        }
    }
}
