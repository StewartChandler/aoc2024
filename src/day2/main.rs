const INPUT: &'static str = include_str!("input.txt");

fn filter_pt2(inner: &Box<[u32]>) -> bool {
    if inner.len() <= 2 {
        return true;
    }

    let mut is_inc = inner[0].cmp(&inner[1]);

    let mut it = inner.windows(3).peekable();

    // check first and second
    if inner[0].abs_diff(inner[1]) >= 1
        && inner[0].abs_diff(inner[1]) <= 3
        && inner[1].cmp(&inner[2]) == is_inc
    {
        // first is fine so iterate through until you find an erronious pairing
        while it
            .next_if(|x| {
                x[1].cmp(&x[2]) == is_inc && x[1].abs_diff(x[2]) >= 1 && x[1].abs_diff(x[2]) <= 3
            })
            .is_some()
        {}

        // if there is still elements left than you found consecutive values that do not go
        // together, thus we must remove either the first or second element for the list to
        // remain valid
        if let Some(x1) = it.next() {
            // check if the next pairing is also invalid, and if so, see if we can fix it
            // by removing the second value
            if let Some(x2) = it.next_if(|x| {
                !(x[1].cmp(&x[2]) == is_inc && x[1].abs_diff(x[2]) >= 1 && x[1].abs_diff(x[2]) <= 3)
            }) {
                if !(x2[0].cmp(&x2[2]) == is_inc
                    && x2[0].abs_diff(x2[2]) >= 1
                    && x2[0].abs_diff(x2[2]) <= 3)
                {
                    // even removing the value does not fix it
                    return false;
                }
            // otherwise we cann fix the invalid pairing by skipping either the first or
            // second of the values, to skip the first, we don't have to do anything, so
            // we try that first and if not, see if we can skip the second value
            } else if !(x1[0].cmp(&x1[2]) == is_inc
                && x1[0].abs_diff(x1[2]) >= 1
                && x1[0].abs_diff(x1[2]) <= 3)
            {
                // removing the first value doesn't fix it
                if let Some(x2) = it.next() {
                    if !(x2[0].cmp(&x2[2]) == is_inc
                        && x2[0].abs_diff(x2[2]) >= 1
                        && x2[0].abs_diff(x2[2]) <= 3)
                    {
                        // removing the second value doesn't fix it
                        return false;
                    }
                }
            }
        }
    } else {
        // we must skip either the first or second
        if let Some(x1) = it.next() {
            if let Some(x2) = it.peek() {
                is_inc = x2[1].cmp(&x2[2]);
                if !(x1[1].cmp(&x1[2]) == is_inc
                    && x1[1].abs_diff(x1[2]) >= 1
                    && x1[1].abs_diff(x1[2]) <= 3)
                {
                    if x2[0].cmp(&x2[2]) == is_inc
                        && x2[0].abs_diff(x2[2]) >= 1
                        && x2[0].abs_diff(x2[2]) <= 3
                    {
                        // skip 1
                        it.next();
                    } else {
                        return false;
                    }
                }
            } else {
                return (x1[1].abs_diff(x1[2]) >= 1 && x1[1].abs_diff(x1[2]) <= 3)
                    || (x1[0].abs_diff(x1[2]) >= 1 && x1[0].abs_diff(x1[2]) <= 3);
            }
        }
    }

    it.all(|x| x[1].cmp(&x[2]) == is_inc && x[1].abs_diff(x[2]) >= 1 && x[1].abs_diff(x[2]) <= 3)
}

fn filter_pt2_trivial(inner: &Box<[u32]>) -> bool {
    (0..inner.len()).any(|i| {
        let inner2 = inner[0..i]
            .iter()
            .chain(inner[i + 1..].iter())
            .copied()
            .collect::<Vec<_>>();

        let is_inc = inner2[0].cmp(&inner2[1]);

        inner2.windows(2).all(|x| {
            x[0].cmp(&x[1]) == is_inc && x[0].abs_diff(x[1]) >= 1 && x[0].abs_diff(x[1]) <= 3
        })
    })
}

fn main() {
    let reports = INPUT
        .lines()
        .map(|s| {
            s.split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
        .filter(|b| !b.is_empty())
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let result = reports
        .iter()
        .filter(|inner| {
            if inner.len() <= 1 {
                return true;
            }

            let is_inc = inner[0].cmp(&inner[1]);

            inner.windows(2).all(|x| {
                x[0].cmp(&x[1]) == is_inc && x[0].abs_diff(x[1]) >= 1 && x[0].abs_diff(x[1]) <= 3
            })
        })
        .count();

    println!("part 1 result: {result}");

    // reports.iter().for_each(|inner| {
    //     if filter_pt2(inner) != filter_pt2_trivial(inner) {
    //         dbg!(inner);
    //         println!(
    //             "left: {}, right: {}",
    //             filter_pt2(inner),
    //             filter_pt2_trivial(inner)
    //         );
    //     }
    // });

    let result = reports
        .iter()
        .filter(|inner| filter_pt2_trivial(inner))
        .count();

    println!("part 2 result: {result}");
}
