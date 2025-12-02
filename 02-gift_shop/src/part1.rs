use crate::parse_ranges;

pub fn solution(input: &str) -> u64 {
    let mut invalid_id_sum = 0;
    parse_ranges(input).for_each(|(start, end)| {
        print!("{start}..={end} : ");
        let size = ((start as f64).log10() / 2.0).ceil().max(1.0) as usize;
        print!("size:{size} ");
        // let step = 11 + 9 * (10u64.pow(size - 1));
        let step: usize = format!("1{}1", "0".repeat(size - 1)).parse().unwrap();
        print!("step:{step} ");
        let first_invalid = start
            .div_ceil(step as u64)
            .max(10u32.pow(size as u32 - 1) as u64)
            * step as u64;
        let last_invalid = (((end as f64) / step as f64).floor() as u64)
            .min("9".repeat(size).parse().unwrap())
            * step as u64;
        print!("first:{first_invalid}, last:{last_invalid} ");
        print!("Invalids: ");

        for invalid in (first_invalid..=last_invalid).step_by(step) {
            print!("{invalid}, ");
            // println!("{invalid}");
            invalid_id_sum += invalid;
        }
        println!();
    });
    invalid_id_sum
}

pub fn solution_simple(input: &str) -> u64 {
    let mut invalid_id_sum = 0;
    parse_ranges(input).for_each(|(start, end)| {
        let size = ((start as f64).log10() / 2.0).ceil().max(1.0) as usize;
        for i in start..=end {
            if i.ilog10() % 2 == 0 {
                continue;
            }
            if (i / 10u32.pow(size as u32) as u64) == (i % 10u32.pow(size as u32) as u64) {
                println!("{i}");
                invalid_id_sum += i;
            }
        }
    });
    invalid_id_sum
}

#[cfg(test)]
mod tests {
    use super::*;
}
