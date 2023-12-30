use divan::AllocProfiler;

#[global_allocator]
static GLOBAL_ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

mod tests {
    use aoc_rust_2023::day01::*;
    use divan::Bencher;

    #[divan::bench]
    fn run_parse_input(b: Bencher) {
        b.bench(parse_test_input);
    }

    #[divan::bench]
    fn run_part_1(b: Bencher) {
        let input = parse_test_input();
        b.bench(|| part_1(&input));
    }

    #[divan::bench]
    fn run_part_2(b: Bencher) {
        let input = parse_test_input();
        b.bench(|| part_2(&input));
    }
}
