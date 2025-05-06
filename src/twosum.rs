use std::collections::HashSet;

pub fn two_sum(lis_nums:&Vec<i32>,tot:i32)-> bool{
    let mut seen = HashSet::new();
    for ele in lis_nums {
        let to_find = tot - ele;
        if seen.contains(&to_find){
            return true;
        }
        seen.insert(ele);
    }
    return false


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_true(){
        let nums = vec![1,2,3,4,5,6];
        let total = 4;
        assert_eq!(two_sum(&nums,total),true);
    }
}