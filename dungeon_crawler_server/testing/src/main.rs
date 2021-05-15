use std::{f32::consts::PI, time::Duration};

const MIN: f32 = -PI/2.0;
const MAX: f32 = PI/2.0;
const STEPS: i32 = 25;
const SIGHT_RNG: f32 = 4.0;

fn main() -> ! {
    loop {
        let mut f = 0;
        while f < STEPS {
            f += 1;
            let fstep = f as f32 / STEPS as f32;
            let arith = calc_arith(fstep*PI/2.0);
            println!("f = {}*PI / 2.0, ({}, {})", fstep, arith.cos() * SIGHT_RNG, arith.sin() * SIGHT_RNG);
            std::thread::sleep(Duration::from_millis(100));
        }
        while f > (-STEPS) {
            f -= 1;
            let fstep = f as f32 / STEPS as f32;
            let arith = calc_arith(fstep*PI/2.0);
            println!("f = {}*PI/2.0, ({}, {})", fstep, ((fstep*PI)/2.0).cos() * SIGHT_RNG, arith.sin() * SIGHT_RNG);
            std::thread::sleep(Duration::from_millis(100));
        }
        while f < 0 {
            f += 1;
            let fstep = f as f32 / STEPS as f32;
            let arith = calc_arith(fstep*PI/2.0);
            println!("f = {}*PI / 2.0, ({}, {})", fstep, ((fstep*PI)/2.0).cos() * SIGHT_RNG, arith.sin() * SIGHT_RNG);
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

fn calc_arith(f: f32) -> f32 {
    (MAX - f32::min((MIN - f).abs(), (MAX - f).abs())) / MAX
}