use std::io::{self, Read};

struct Body {
    px: f64,
    py: f64,
    vx: f64,
    vy: f64,
}

struct World {
    dt: f64,
    mass: f64,
    gx: f64,
    gy: f64,
    drag: f64,
    fx: f64,
    fy: f64,
}

fn f4(x: f64) -> String {
    let s = format!("{:.4}", x);
    if s == "-0.0000" {
        "0.0000".to_string()
    } else {
        s
    }
}

// Advance body b by one step of world w, in place.
fn step(_b: &mut Body, _w: &World) {
    // TODO: mass 0 means immovable (return at once). Otherwise:
    //   net force = applied + mass * g - drag * v   (v from the START of the step)
    //   acceleration = net force * (1 / mass)
    //   update the velocity first, then the position with the NEW velocity
    if _w.mass == 0.0 {
        return;
    } else {
        let fx = _w.fx + _w.mass * _w.gx - _w.drag * _b.vx;
        let fy = _w.fy + _w.mass * _w.gy - _w.drag * _b.vy;
        _b.vx += fx / _w.mass * _w.dt;
        _b.vy += fy / _w.mass * _w.dt;
        _b.px += _b.vx * _w.dt;
        _b.py += _b.vy * _w.dt;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut w = World {
        dt: 0.0,
        mass: 1.0,
        gx: 0.0,
        gy: 0.0,
        drag: 0.0,
        fx: 0.0,
        fy: 0.0,
    };
    let mut b = Body {
        px: 0.0,
        py: 0.0,
        vx: 0.0,
        vy: 0.0,
    };
    let mut k = 0;
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
        match p[0] {
            "DT" => w.dt = n[0],
            "MASS" => w.mass = n[0],
            "GRAVITY" => {
                w.gx = n[0];
                w.gy = n[1];
            }
            "DRAG" => w.drag = n[0],
            "FORCE" => {
                w.fx = n[0];
                w.fy = n[1];
            }
            "BODY" => {
                b = Body {
                    px: n[0],
                    py: n[1],
                    vx: n[2],
                    vy: n[3],
                }
            }
            "RUN" => {
                for _ in 0..(n[0] as i32) {
                    step(&mut b, &w);
                    k += 1;
                    println!("{} {} {} {} {}", k, f4(b.px), f4(b.py), f4(b.vx), f4(b.vy));
                }
            }
            _ => {}
        }
    }
}
