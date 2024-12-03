use std::cmp::Ordering;

const INPUT: &'static str = include_str!("input.txt");

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
            if is_inc == Ordering::Equal {
                return false;
            }

            inner.windows(2).all(|x| {
                x[0].cmp(&x[1]) == is_inc && x[0].abs_diff(x[1]) >= 1 && x[0].abs_diff(x[1]) <= 3
            })
        })
        .count();

    println!("part 1 result: {result}");

    let result = reports
        .iter()
        .filter(|inner| {
            if inner.len() <= 2 {
                return true;
            }

            let mut is_inc = inner[0].cmp(&inner[1]);

            let mut it = inner.windows(3).peekable();

            // check first and second
            if is_inc != Ordering::Equal
                && inner[0].abs_diff(inner[1]) >= 1
                && inner[0].abs_diff(inner[1]) <= 3
                && inner[1].cmp(&inner[2]) == is_inc
            {
                // first is fine so iterate through until you find an erronious pairing
                while it
                    .next_if(|x| {
                        x[1].cmp(&x[2]) == is_inc
                            && x[1].abs_diff(x[2]) >= 1
                            && x[1].abs_diff(x[2]) <= 3
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
                        !(x[1].cmp(&x[2]) == is_inc
                            && x[1].abs_diff(x[2]) >= 1
                            && x[1].abs_diff(x[2]) <= 3)
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
            } else if is_inc == Ordering::Equal {
                // first 2 are the same, so remove the first
                is_inc = inner[1].cmp(&inner[2]);
                if is_inc == Ordering::Equal {
                    return false;
                }
            } else {
                // we must remove one of the first 2 elements, without knowing which ordering to use

                if inner.len() >= 4 {
                    // then we know that the ordering must follow, that of `inner[2]` and `inner[3]`
                    is_inc = inner[2].cmp(&inner[3]);
                    if is_inc == Ordering::Equal {
                        return false;
                    }

                    if inner[1].abs_diff(inner[2]) >= 1
                        && inner[1].abs_diff(inner[2]) <= 3
                        && inner[1].cmp(&inner[2]) == is_inc
                    {
                        // skip the first one (do nothing)
                    } else if inner[0].abs_diff(inner[2]) >= 1
                        && inner[0].abs_diff(inner[2]) <= 3
                        && inner[0].cmp(&inner[2]) == is_inc
                    {
                        // skip the second
                        it.next();
                    } else {
                        // removing either the first or second doesn't save us
                        return false;
                    }
                } else {
                    return (inner[1].abs_diff(inner[2]) >= 1
                        && inner[1].abs_diff(inner[2]) <= 3
                        && inner[1].cmp(&inner[2]) != Ordering::Equal)
                        || (inner[0].abs_diff(inner[2]) >= 1
                            && inner[0].abs_diff(inner[2]) <= 3
                            && inner[0].cmp(&inner[2]) != Ordering::Equal);
                }
            }

            it.all(|x| {
                x[1].cmp(&x[2]) == is_inc && x[1].abs_diff(x[2]) >= 1 && x[1].abs_diff(x[2]) <= 3
            })
        })
        .count();

    println!("part 2 result: {result}");
}
