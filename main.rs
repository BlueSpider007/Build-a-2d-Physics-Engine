use std::io::{self, Read};

#[derive(Copy, Clone)]
struct World {
    k: f64,
    mass: f64,
    dt: f64,
    x: f64,
    v: f64,
}

fn f4(x: f64) -> String {
    let s = format!("{:.4}", x);
    if s == "-0.0000" {
        "0.0000".to_string()
    } else {
        s
    }
}

fn energy(w: World) -> f64 {
    0.5 * w.mass * w.v * w.v + 0.5 * w.k * w.x * w.x
}

fn ratio(e: f64, e0: f64) -> f64 {
    if e0 == 0.0 {
        1.0
    } else {
        e / e0
    }
}

fn explicit(w: &mut World, steps: u32) {
    for _ in 0..steps {
        let a = -w.k * w.x / w.mass;
        w.x += w.v * w.dt;
        w.v += a * w.dt;
    }
}

fn symplectic(w: &mut World, steps: u32) {
    for _ in 0..steps {
        let a = -w.k * w.x / w.mass;
        w.v += a * w.dt;
        w.x += w.v * w.dt;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut k = 1.0;
    let mut mass = 1.0;
    let mut dt = 0.1;

    let mut ex = World {
        k,
        mass,
        dt,
        x: 0.0,
        v: 0.0,
    };
    let mut sy = ex;

    let mut e0 = 0.0;

    for line in input.lines() {
        let p: Vec<&str> = line.split_whitespace().collect();

        if p.is_empty() {
            continue;
        }

        match p[0] {
            "SPRING" => {
                k = p[1].parse().unwrap();
                mass = p[2].parse().unwrap();
                ex.k = k;
                ex.mass = mass;
                sy.k = k;
                sy.mass = mass;
            }
            "DT" => {
                dt = p[1].parse().unwrap();
                ex.dt = dt;
                sy.dt = dt;
            }
            "START" => {
                let x: f64 = p[1].parse().unwrap();
                let v: f64 = p[2].parse().unwrap();

                ex.x = x;
                ex.v = v;
                sy.x = x;
                sy.v = v;

                e0 = energy(ex);
            }
            "RUN" => {
                let steps: u32 = p[1].parse().unwrap();

                explicit(&mut ex, steps);
                symplectic(&mut sy, steps);

                println!(
                    "EXPLICIT {} {} {}",
                    f4(ex.x),
                    f4(ex.v),
                    f4(ratio(energy(ex), e0))
                );

                println!(
                    "SYMPLECTIC {} {} {}",
                    f4(sy.x),
                    f4(sy.v),
                    f4(ratio(energy(sy), e0))
                );
            }
            _ => {}
        }
    }
}
