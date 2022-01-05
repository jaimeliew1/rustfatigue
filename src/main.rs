use rand::thread_rng;
use rand::Rng;
use rustfatigue::eq_load;

fn main() {
    let mut rng = thread_rng();
    let now = std::time::Instant::now();
    
    for _ in 0..1000 {
        let inp_vec: Vec<f64> = (0..1000).map(|_| rng.gen_range(0.0..100.0)).collect();
        let _del = eq_load(&inp_vec, 4.0, 600);
    }

    let rate = 1000.0 / ((now.elapsed().as_micros() as f64) / 1000000.0);
    println!("iterations per second: {}", rate);
}
