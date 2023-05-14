fn find_median(nums: &[i32]) -> f64 {
    let n = nums.len();
    if n%2 == 0 {
        nums[n/2] as f64
    }
    else {
        let mid = n/2;
        (nums[mid-1] as f64 + nums[mid] as f64) /2.0
    }
}


pub fn main() {
    let nums1 = [1,2,3];
    let nums2 = [4,5,6,7];

    let median1 = find_median(&nums1);
    let median2 = find_median(&nums2);
    println!("median1: {}", median1);
    println!("median2: {}", median2);
}
