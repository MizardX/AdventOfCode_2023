use divan::AllocProfiler;

#[global_allocator]
static GLOBAL_ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}
mod tests {
    use divan::black_box;

    use aoc_rust_2023::day25::*;
    use divan::Bencher;

    #[divan::bench]
    fn run_1_parse_input(b: Bencher) {
        b.bench(|| black_box(parse_test_input()));
    }

    const LENS: &[usize] = &[0, 8, 64, 1024];

    #[divan::bench(consts = LENS)]
    fn run_2_run_cycles<const N: usize>(b: Bencher) {
        let input = parse_test_input();
        b.bench(|| black_box(run_cycles(&input, black_box(N))));
    }

    #[divan::bench]
    fn run_3_full(b: Bencher) {
        let input = parse_test_input();
        b.bench(|| black_box(part_1(&input)));
    }
}
