const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let (mut list_a, mut list_b) = INPUT
        .lines()
        .map_while(|s| {
            let mut it = s.split_whitespace();

            Some((
                it.next()?.parse::<u32>().ok()?,
                it.next()?.parse::<u32>().ok()?,
            ))
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    list_a.sort_unstable();
    list_b.sort_unstable();

    let result = list_a
        .iter()
        .copied()
        .zip(list_b.iter().copied())
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>();

    println!("part 1 result: {result}");

    let mut it_a = list_a.iter().copied().peekable();
    let mut it_b = list_b.iter().copied().peekable();

    let mut result = 0u32;
    while !(it_a.peek().is_none() || it_b.peek().is_none()) {
        let a = it_a.next().unwrap();
        let mut cnt_a = 1;
        while it_a.next_if(|a2| *a2 == a).is_some() {
            cnt_a += 1;
        }

        // skip b
        while it_b.next_if(|b| *b < a).is_some() {}

        let mut cnt_b = 0;
        while it_b.next_if(|b| *b == a).is_some() {
            cnt_b += 1;
        }

        result += cnt_a * cnt_b * a;
    }

    println!("part 2 result: {result}");
}
