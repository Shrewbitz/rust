pub fn sum3(nums: Vec<i32>, target: i32) -> i32 {
    let mut sorted = nums;
    sorted.sort();
    // let mut diff = i32::MAX;
    let mut diff = sorted[sorted.len()-1]*3 + target.abs() + 1;
    let mut result: i32 = 0;
    let length = sorted.len();
    for i in 0..length {
        let mut start = i + 1;
        let mut end = length - 1;
        while start < end {
            let sum = sorted[i] + sorted[start] + sorted[end];
            if sum == target {return target};
            if (target - sum).abs() < diff {
                result = sum;
                diff = (target - sum).abs();
            }
            if sum < target {start += 1} else {end -= 1};
        }
    }
    println!("{}", result);
    return result;
}