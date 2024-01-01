use bstr::ByteSlice;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 03");

    println!("++Example");
    let example = parse_input(EXAMPLE);
    println!("|+-Part 1: {} (expected 4361)", part_1(&example));
    println!("|'-Part 2: {} (expected 467835)", part_2(&example));

    println!("++Input");
    let input = parse_input(INPUT);
    println!("|+-Part 1: {} (expected 532428)", part_1(&input));
    println!("|'-Part 2: {} (expected 84051670)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn parse_test_input<'a>() -> Input<'a> {
    parse_input(INPUT)
}

pub fn profile() {
    use std::hint::black_box;
    let input = parse_test_input();
    black_box(part_1(&input));
    black_box(part_2(&input));
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let mut sum: usize = 0;
    let empty_line = &[b'.'; 140][..input.lines[0].len()];
    let height = input.lines.len();
    let first = [[&empty_line, input.lines[0], input.lines[1]]].into_iter();
    let middle = input.lines.array_windows().into_iter();
    let last = [[
        input.lines[height - 2],
        input.lines[height - 1],
        &empty_line,
    ]]
    .into_iter();
    for [above, current, below] in []
        .into_iter()
        .chain(first)
        .chain(middle.copied())
        .chain(last)
    {
        let mut value = 0;
        let mut len = 0;
        for (c, ch) in current.iter().chain(b".".iter()).copied().enumerate() {
            match ch {
                b'0'..=b'9' => {
                    value = 10 * value + (ch - b'0') as usize;
                    len += 1;
                }
                _ if len > 0 => {
                    let left = (c - len).saturating_sub(1);
                    let right = (c + 1).min(current.len());
                    if [
                        &above[left..right],
                        &current[left..right],
                        &below[left..right],
                    ]
                    .into_iter()
                    .any(|s| s.iter().copied().any(is_symbol))
                    {
                        sum += value;
                    }
                    value = 0;
                    len = 0;
                }
                _ => (),
            }
        }
    }

    sum
}

fn is_symbol(ch: u8) -> bool {
    !matches!(ch, b'.' | b'0'..=b'9')
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let mut sum: usize = 0;
    let empty_line = &[b'.'; 140][..input.lines[0].len()];
    let height = input.lines.len();
    let first = [[&empty_line, input.lines[0], input.lines[1]]].into_iter();
    let middle = input.lines.array_windows().into_iter();
    let last = [[
        input.lines[height - 2],
        input.lines[height - 1],
        &empty_line,
    ]]
    .into_iter();
    for [above, current, below] in []
        .into_iter()
        .chain(first)
        .chain(middle.copied())
        .chain(last)
    {
        'symbol: for (c, ch) in current.iter().copied().enumerate() {
            if ch != b'*' {
                continue;
            }
            let left = c.saturating_sub(3);
            let right = (c + 3).min(current.len() - 1);
            let mut product = 1;
            let mut count = 0;
            for line in [
                &above[left..=right],
                &current[left..=right],
                &below[left..=right],
            ] {
                let mut value = 0;
                let mut len = 0;
                for (i, ch) in line.iter().chain(b".".iter()).copied().enumerate() {
                    match ch {
                        b'0'..=b'9' => {
                            value = value * 10 + (ch - b'0') as usize;
                            len += 1;
                        }
                        _ if len > 0 => {
                            // left+i is the position of the character just after the number. 123_ <-- here
                            // left+i-len is the position of the first character. here --> 123
                            // We want (left+i-len..left+i) to overlap (c-1..=c+1)
                            let start = left + i - len;
                            let end = left + i - 1;
                            if end + 1 >= c && start <= c + 1 {
                                if count == 2 {
                                    continue 'symbol;
                                }
                                product *= value;
                                count += 1;
                            }
                            value = 0;
                            len = 0;
                        }
                        _ => (),
                    }
                }
            }
            if count == 2 {
                sum += product;
            }
        }
    }
    sum
}

#[derive(Debug, Clone, Default)]
pub struct Input<'a> {
    lines: Vec<&'a [u8]>,
}

fn parse_input(text: &str) -> Input<'_> {
    let mut lines = Vec::with_capacity(140);
    lines.extend(text.as_bytes().lines());
    Input { lines }
}
