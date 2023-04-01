use anyhow::Result;
use rand::Rng;

fn main() -> Result<()> {
    let data = std::fs::read_to_string("data.txt")?;
    let data = data.lines().collect::<Vec<&str>>();
    let mut rng = rand::thread_rng();
    loop {
        let idx = rng.gen_range(0..data.len());
        let line = data[idx];
        let now = chrono::Utc::now();
        println!("{} {}", now, line);
        let sleep = rng.gen_range(20..300);
        std::thread::sleep(std::time::Duration::from_millis(sleep));
    }
}
