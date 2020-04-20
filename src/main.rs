#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Points {
    xs: Vec<i32>,
    ys: Vec<i32>,
}

impl Points {
    fn increment_x(&mut self) {
        for e in self.xs.iter_mut() {
            *e += 1;
        }
    }
}

impl Point {
    fn increment_x(&mut self) {
        self.x += 1;
    }
}

fn main() {
    let size = 100000000;
    let aos = run_aos(size);
    let soa = run_soa(size);
    println!(
        "Increment: ({} - {}) / {} = {}%",
        aos,
        soa,
        aos,
        (aos - soa) * 100 / aos
    )
}

fn initialize_soa(size: i32) -> Points {
    let xs: Vec<i32> = (0..size).into_iter().collect();
    let ys: Vec<i32> = (0..size).into_iter().collect();
    Points { xs: xs, ys: ys }
}

fn initialize_aos(size: i32) -> Vec<Point> {
    (0..size)
        .into_iter()
        .map(|x| (x, x))
        .map(|(x, y)| Point { x, y })
        .collect()
}

fn run_soa(size: i32) -> u128 {
    let start_time = std::time::SystemTime::now();

    let mut points = initialize_soa(size);

    let run_time = start_time.elapsed().unwrap().as_millis();

    points.increment_x();

    let elapsed_time = start_time.elapsed().unwrap().as_millis() - run_time;
    println!("Time with soa {}", elapsed_time);
    elapsed_time
}

fn run_aos(size: i32) -> u128 {
    let start_time = std::time::SystemTime::now();

    let mut vector = initialize_aos(size);

    let run_time = start_time.elapsed().unwrap().as_millis();

    for point in vector.iter_mut() {
        point.increment_x();
    }

    let elapsed_time = start_time.elapsed().unwrap().as_millis() - run_time;
    println!("Time with aos {}", elapsed_time);
    elapsed_time
}
