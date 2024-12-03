const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let result = INPUT
        .split("mul(")
        .skip(1)
        .filter_map(|s| {
            let (str_a, s) = s.split_once(',')?;
            let (str_b, _) = s.split_once(')')?;

            Some(str_a.parse::<u64>().ok()? * str_b.parse::<u64>().ok()?)
        })
        .sum::<u64>();

    println!("part 1 result: {result}");

    let result = INPUT
        .split("do()")
        .map(|s| s.split_once("don't()").map_or(s, |(s1, _)| s1))
        .flat_map(|s| {
            s.split("mul(").skip(1).filter_map(|s| {
                let (str_a, s) = s.split_once(',')?;
                let (str_b, _) = s.split_once(')')?;

                Some(str_a.parse::<u64>().ok()? * str_b.parse::<u64>().ok()?)
            })
        })
        .sum::<u64>();

    println!("part 2 result: {result}");
}
