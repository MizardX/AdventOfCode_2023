use divan::AllocProfiler;

#[global_allocator]
static GLOBAL_ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}
mod tests {
    use divan::black_box;

    use aoc_rust_2023::day21::*;
    use divan::Bencher;

    #[divan::bench]
    fn run_parse_input(b: Bencher) {
        b.bench(|| black_box(parse_test_input()));
    }

    #[divan::bench]
    fn run_part_1(b: Bencher) {
        let input = parse_test_input();
        b.bench(|| black_box(part_1(&input, 64)));
    }

    #[divan::bench]
    fn run_part_2(b: Bencher) {
        let input = parse_test_input();
        b.bench(|| black_box(part_2(&input, 26_501_365)));
    }
}
