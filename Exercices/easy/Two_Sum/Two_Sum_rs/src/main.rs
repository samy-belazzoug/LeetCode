/*impl Solution {
 */   
    /*pub*/ fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut result_index_1 = 0;
        let mut result_index_2 = 0;
        let mut result: Vec<i32> = Vec::new();

        let mut not_found: bool = true;
        while not_found == true {

            for loopindex in 0..nums.len()-1 {

                let indexequal = loopindex;
                let loopindex = loopindex as usize; //So we can use it to use Arrays's elements
                let mut indexnumbers = 0;

                for numbers in &nums {
                    /*println!("Loopindex : {}",loopindex);
                    println!("IndexEqual : {}",indexequal);
                    println!("indexnumbers : {}",indexnumbers);*/
                    
                    if loopindex != indexnumbers { //So we don't use the same element twice
                    
                        let calculation = nums[loopindex] + numbers;
                                //println!("{} + {} = {}",nums[loopindex], numbers, calculation);
                        
                        if calculation == target {
                            result_index_1 = indexequal;
                            result_index_2 = indexnumbers;
                                    //println!("FOUND!!!!!");
                            not_found = false;
                        }
                    }
                    indexnumbers += 1;
                }
                
                if not_found == false {
                    let result_index_1 = result_index_1 as i32;
                    let result_index_2 = result_index_2 as i32;
                    result.push(result_index_1);
                    result.push(result_index_2);
                    //println!("[{},{}]",result_index_1,result_index_2);
                    break;
                }
            }
        }
        return result
    }
/*}*/
fn main() {
    two_sum(vec![2,7,11,15],9);
}