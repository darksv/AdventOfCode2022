pub(crate) fn day_01(input: &str) -> (u32, u32) {
    let mut sums = [0; 3];
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            if sum > sums[0] {
                sums[0] = sum;
                sums.sort_unstable();
            }
            sum = 0;
        } else {
            let cal: u32 = line.parse().unwrap();
            sum += cal;
        }
    }

    if sum > sums[0] {
        sums[0] = sum;
        sums.sort_unstable();
    }


    (sums[2], sums[0] + sums[1] + sums[2])
}