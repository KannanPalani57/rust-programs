impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut write_index = 0;
        for read_index in 1..nums.len() {
            if nums[write_index] != nums[read_index] {
                write_index += 1;
                nums[write_index] = nums[read_index];
            }
        }
        (write_index + 1) as i32
    }
}

struct Solution;

fn main() {

    let mut nums = vec![0, 1, 1, 2, 3, 3,3];
    let k = Solution::remove_duplicates(&mut nums);
        
    
    println!("After removing duplicates: {:?}", &nums[..k as usize]);
    
    // output: After removing duplicates: [0, 1, 2, 3]

}
