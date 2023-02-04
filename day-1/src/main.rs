use itertools::{FoldWhile, Itertools};
use std::cmp::Reverse;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let answer = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        // consider all lines separated by `None`
        .batching(|it| {
            it.fold_while(None, |acc: Option<u64>, v| match v {
                Some(v) => FoldWhile::Continue(Some(acc.unwrap_or_default() + v)),
                // that's the group separator, `fold_while` is done
                None => FoldWhile::Done(acc),
            })
            // this returns `Some(total)` if we found a group, `None` if we're
            // at the end, to let `batching` end.
            .into_inner()
        })
        // this turns `k_smallest` into `k_largest`
        .map(Reverse)
        .k_smallest(3)
        // and this strips the `Reverse` so we can sum
        .map(|x| x.0)
        .sum::<u64>();
    println!("{answer:?}");

    Ok(())
}
