use std::io::{self, Read};

#[derive(Clone, Copy)]
struct V2 {
    x: f64,
    y: f64,
}

fn f4(x: f64) -> String {
    let s = format!("{:.4}", x);
    if s == "-0.0000" {
        "0.0000".to_string()
    } else {
        s
    }
}

fn add(a: V2, b: V2) -> V2 {
    V2 {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}
fn sub(a: V2, b: V2) -> V2 {
    V2 {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}
fn scale(a: V2, s: f64) -> V2 {
    V2 {
        x: a.x * s,
        y: a.y * s,
    }
}
fn length(a: V2) -> f64 {
    (a.x * a.x + a.y * a.y).sqrt()
}
fn perp(a: V2) -> V2 {
    V2 { x: -a.y, y: a.x }
}

fn dot(a: V2, b: V2) -> f64 {
    a.x * b.x + a.y * b.y
}

fn cross(a: V2, b: V2) -> f64 {
    a.x * b.y - a.y * b.x
}

fn normalize(a: V2) -> V2 {
    if a.x == 0.0 && a.y == 0.0 {
        V2 { x: 0.0, y: 0.0 }
    } else {
        scale(a, 1.0 / length(a))
    }
}

fn project(a: V2, b: V2) -> V2 {
    if b.x == 0.0 && b.y == 0.0 {
        V2 { x: 0.0, y: 0.0 }
    } else {
        scale(b, dot(a, b) / dot(b, b))
    }
}

fn reflect(v: V2, n: V2) -> V2 {
    if n.x == 0.0 && n.y == 0.0 {
        v
    } else {
        sub(v, scale(n, 2.0 * dot(v, n) / dot(n, n)))
    }
}

fn show(v: V2) -> String {
    format!("{} {}", f4(v.x), f4(v.y))
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    for line in input.lines() {
        let p: Vec<&str> = line.split_whitespace().collect();
        if p.is_empty() {
            continue;
        }
        let mut n = [0.0f64; 4];
        for i in 1..p.len() {
            if i <= 4 {
                n[i - 1] = p[i].parse().unwrap();
            }
        }
        let a = V2 { x: n[0], y: n[1] };
        let b = V2 { x: n[2], y: n[3] };
        match p[0] {
            "ADD" => println!("{}", show(add(a, b))),
            "SUB" => println!("{}", show(sub(a, b))),
            "SCALE" => println!("{}", show(scale(a, n[2]))),
            "DOT" => println!("{}", f4(dot(a, b))),
            "CROSS" => println!("{}", f4(cross(a, b))),
            "LEN" => println!("{}", f4(length(a))),
            "NORM" => println!("{}", show(normalize(a))),
            "PERP" => println!("{}", show(perp(a))),
            "PROJ" => println!("{}", show(project(a, b))),
            "REFLECT" => println!("{}", show(reflect(a, b))),
            _ => {}
        }
    }
}
