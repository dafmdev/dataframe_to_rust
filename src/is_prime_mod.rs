use rayon::prelude::*;
use polars::prelude::*;
use polars_core::utils::accumulate_dataframes_vertical;

pub fn is_prime_scalar(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    true
}

fn split_offsets(len: usize, n: usize) -> Vec<(usize, usize)> {
    if n == 1 {
        vec![(0, len)]
    } else {
        let chunk_size = len / n;

        (0..n)
            .map(|partition| {
                let offset = partition * chunk_size;
                let len = if partition == (n - 1) {
                    len - offset
                } else {
                    chunk_size
                };
                (partition * chunk_size, len)
            })
            .collect()
    }
}

fn compute_is_prime(sa: &Series) -> PolarsResult<Series>{
    let sa: Series = sa.i64()?.into_iter().map(|opt_n| opt_n.map(|n| is_prime_scalar(n))).collect();

    Ok(sa)
}

pub fn is_prime_scalar_polars(df: DataFrame, col_a: &str) -> PolarsResult<DataFrame> {
    let offsets = split_offsets(df.height(), rayon::current_num_threads());

    let dfs = offsets.par_iter().map(|(offset, len)| {
        let sub_df = df.slice(*offset as i64, *len);
        let a = sub_df.column(col_a)?;

        let out= compute_is_prime(a)?;

        df!(
            "jaccard" => out
        )

    }).collect::<PolarsResult<Vec<_>>>()?;
    accumulate_dataframes_vertical(dfs)
}