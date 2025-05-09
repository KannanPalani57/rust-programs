
// binary search
fn main() {

    let mut arr = [1,3,6,9, 15, 20, 33, 55]; // sorted array, length = 8, mid = 4

    let searchValue = 20;

    if arr.len() == 0 {
        println!("There is no element to search");
    }else {
        let mut l = 0;
        let mut r = arr.len();
        
        while l < r {
            let mut mid = (l + r) / 2;
            
            if arr[mid] ==  searchValue {
                println!("The search value located in the index of {}", mid);
                break;
            } else if arr[mid] < searchValue {
                l = mid + 1; 
            } else if arr[mid] > searchValue {
                r = mid;
            }
        }
    }
    
    
    
    
    
