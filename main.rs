use std::io::{self, Read};

#[derive(Copy, Clone)]
struct World {
    k: f64,
    mass: f64,
    dt: f64,
    x: f64,
    v: f64,
    steps: u32,
}

fn f4(x: f64) -> String {
    let s = format!("{:.4}", x);
    if s == "-0.0000" {
        "0.0000".to_string()
    } else {
        s
    }
}
fn energy(_w: World) -> f64 {
    0.5 * _w.mass * _w.v * _w.v + 0.5 * _w.k * _w.x * _w.x
}
fn ratio(e: f64, e0: f64) -> f64 {
    if e0 == 0.0 {
        1.0
    } else {
        e / e0
    }
}
fn symplectic(_w: World) -> (f64, f64) {
    let mut w = _w;
    for _i in 0..w.steps {
        let a = -w.k * w.x / w.mass;
        w.v += a * w.dt;
        w.x += w.v * w.dt;
    }
    (w.x, w.v)
}
fn explicit(_w: World) -> (f64, f64) {
    let mut w = _w;
    for _i in 0..w.steps {
        let a = -w.k * w.x / w.mass;
        w.x += w.v * w.dt;
        w.v += a * w.dt;
    }
    (w.x, w.v)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut w = World {
        k: 1.0,
        mass: 1.0,
        dt: 0.1,
        x: 0.0,
        v: 0.0,
        steps: 0,
    };
    let mut e0 = 0.0;
    for line in input.lines() {
        let p: Vec<&str> = line.split_whitespace().collect();
        if p.is_empty() {
            continue;
        }
        let mut n = [0.0f64; 4];
        for i in 1..p.len() {
            if i <= 2 {
                n[i - 1] = p[i].parse().unwrap();
            }
        }
        match p[0] {
            "SPRING" => {
                w.k = n[0];
                w.mass = n[1];
            }
            "DT" => w.dt = n[0],
            "START" => {
                w.x = n[0];
                w.v = n[1];
                e0 = energy(w);
            }
            "RUN" => {
                w.steps += n[0] as u32;
                let (ex, ev) = explicit(w);
                let (sx, sv) = symplectic(w);
                println!(
                    "EXPLICIT {} {} {}",
                    f4(ex),
                    f4(ev),
                    f4(ratio(
                        energy(World {
                            k: w.k,
                            mass: w.mass,
                            x: ex,
                            v: ev,
                            dt: w.dt,
                            steps: w.steps
                        }),
                        e0
                    ))
                );
                println!(
                    "SYMPLECTIC {} {} {}",
                    f4(sx),
                    f4(sv),
                    f4(ratio(
                        energy(World {
                            k: w.k,
                            mass: w.mass,
                            x: sx,
                            v: sv,
                            dt: w.dt,
                            steps: w.steps
                        }),
                        e0
                    ))
                );
            }
            _ => {}
        }
    }
}
