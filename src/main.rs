use std::time::Instant;

mod bubble;

const ARR_SIZE: usize = 100000;

fn benchmark(f: &dyn Fn(&mut [i32]), name: &str) {
    let mut arr: [i32; ARR_SIZE] = [0; ARR_SIZE];
    for i in 0..ARR_SIZE {
        arr[i] = i as i32 + 1;
    }
    print!("Benchmarking {}...", name);

    let start = Instant::now();
    f(&mut arr);
    println!(" {:?}", start.elapsed());
}

fn run_benchmarks() {
    println!("Running benchmarks for {} array elements...", ARR_SIZE);

    benchmark(&bubble::sort, "bubble sort");
}

fn main() {
    run_benchmarks();
}
