use divan::AllocProfiler;

#[global_allocator]
static GLOBAL_ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}
mod tests {
    use divan::black_box;

    use aoc_rust_2023::day23::*;
    use divan::Bencher;

    #[divan::bench]
    fn run_a_parse_input(b: Bencher) {
        b.bench(|| black_box(parse_test_input()));
    }

    #[divan::bench]
    fn run_b_transform_input(b: Bencher) {
        let input = parse_test_input();
        b.bench(|| black_box(transform_test_input(&input)));
    }

    #[divan::bench]
    fn run_c_part_1(b: Bencher) {
        let input = transform_test_input(&parse_test_input());
        b.bench(|| black_box(part_1(&input)));
    }

    #[divan::bench]
    fn run_d_part_2(b: Bencher) {
        let input = transform_test_input(&parse_test_input());
        b.bench(|| black_box(part_2(&input)));
    }
}
