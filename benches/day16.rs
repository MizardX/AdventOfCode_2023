use divan::AllocProfiler;

#[global_allocator]
static GLOBAL_ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}
mod tests {
    use divan::black_box;

    use aoc_rust_2023::day16::*;
    use divan::Bencher;

    #[divan::bench]
    fn run_parse_input(b: Bencher) {
        b.bench(|| black_box(parse_test_input()));
    }

    #[divan::bench]
    fn run_part_1(b: Bencher) {
        let graph = parse_test_input();
        b.bench(|| black_box(part_1(&graph)));
    }

    #[divan::bench]
    fn run_part_2(b: Bencher) {
        let graph = parse_test_input();
        b.bench(|| black_box(part_2(&graph)));
    }
}
