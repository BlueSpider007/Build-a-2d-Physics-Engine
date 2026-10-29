use std::io::{self, Read};

fn f4(x: f64) -> String {
    let s = format!("{:.4}", x);
    if s == "-0.0000" {
        "0.0000".to_string()
    } else {
        s
    }
}

// a, b = [x0, y0, x1, y1]. Return None for no contact, else Some((nx, ny, depth)).
fn box_hit(a: &[f64], b: &[f64]) -> Option<(f64, f64, f64)> {
    // TODO: per axis, overlap = half_a + half_b - |centre_b - centre_a|.
    // No contact if either overlap is negative. Otherwise use the axis with
    // the smaller overlap (x on a tie); the normal's sign follows centre_b - centre_a.
    if a[0] <= b[2] && a[2] >= b[0] && a[1] <= b[3] && a[3] >= b[1] {
        let hax = ((a[2] - a[0]) / 2.0).abs();
        let cax = ((a[2] + a[0]) / 2.0);
        let hbx = ((b[2] - b[0]) / 2.0).abs();
        let cbx = ((b[2] + b[0]) / 2.0);
        let ox = hax + hbx - (cbx - cax).abs();
        let hay = ((a[3] - a[1]) / 2.0).abs();
        let cay = ((a[3] + a[1]) / 2.0);
        let hby = ((b[3] - b[1]) / 2.0).abs();
        let cby = ((b[3] + b[1]) / 2.0);
        let oy = hay + hby - (cby - cay).abs();
        let (x, y, depth);

        if ox <= oy {
            depth = ox;
            if cbx >= cax {
                (x, y) = (1.0, 0.0);
            } else {
                (x, y) = (-1.0, 0.0);
            }
        } else {
            depth = oy;
            if cby >= cay {
                (x, y) = (0.0, 1.0);
            } else {
                (x, y) = (0.0, -1.0);
            }
        }
        Some((x, y, depth))
    } else {
        None
    }
}

// Return None for no contact, else Some((nx, ny, depth)); the normal points from A to B.
fn circle_hit(ax: f64, ay: f64, ra: f64, bx: f64, by: f64, rb: f64) -> Option<(f64, f64, f64)> {
    // TODO: compare squared distance with the squared radius sum, then
    // normal = d / |d| (f64::sqrt), depth = r - |d|.
    // Coincident centres: normal (1, 0), depth r.
    let d2 = (bx - ax) * (bx - ax) + (by - ay) * (by - ay);
    let r = ra + rb;
    if d2 > r * r {
        None
    } else {
        if bx - ax != 0.0 || by - ay != 0.0 {
            let ad = d2.sqrt();
            let dx = (bx - ax) / ad;
            let dy = (by - ay) / ad;
            let depth = r - ad;
            Some((dx, dy, depth))
        } else {
            Some((1.0, 0.0, r))
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    for line in input.lines() {
        let p: Vec<&str> = line.split_whitespace().collect();
        if p.is_empty() {
            continue;
        }
        let mut n = [0.0f64; 8];
        for i in 1..p.len() {
            if i <= 8 {
                n[i - 1] = p[i].parse().unwrap();
            }
        }
        let h = match p[0] {
            "BOX" => box_hit(&n[0..4], &n[4..8]),
            "CIRCLE" => circle_hit(n[0], n[1], n[2], n[3], n[4], n[5]),
            _ => continue,
        };
        match h {
            None => println!("NO_OVERLAP"),
            Some((nx, ny, d)) => println!("HIT {} {} {}", f4(nx), f4(ny), f4(d)),
        }
    }
}
