pub mod id_checker
{
    use anyhow::*;
    use std::ops::{AddAssign, Sub};
    use std::cmp::Ord;
    
    pub trait Integer {}

    impl Integer for u8 {}
    impl Integer for u16 {}
    impl Integer for u32 {}
    impl Integer for u64 {}
    impl Integer for u128 {}
    
    impl Integer for i8 {}
    impl Integer for i16 {}
    impl Integer for i32 {}
    impl Integer for i64 {}
    impl Integer for i128 {}
    
    pub fn check_ids_offset<T>(left_collection: &[T], right_collection: &[T]) -> anyhow::Result<T>
    where
        T: Copy,
        T: Sub<Output = T>,
        T: PartialEq,
        T: Integer,
        T: Ord,
        T: Default,
        T: AddAssign
    {
        if left_collection.len() != right_collection.len() { return Err(Error::msg("Collections length have to be equal")) }
        let mut current_offset: T = T::default();

        for i in 0..left_collection.len() 
        {
            let left_value: T = left_collection[i];    
            let right_value: T = right_collection[i];
            
            
            if left_value < right_value { current_offset += right_value.sub(left_value); }
            else { current_offset += left_value.sub(right_value); } 
        }
        
        Ok(current_offset)
    }
}


#[cfg(test)]
mod tests 
{
    use super::id_checker::*;
    
    #[test]
    fn check_ids_equal()
    {
        let left_ids: Vec<i32> = vec![1, 2, 3, 4, 5];
        let right_ids: Vec<i32> = vec![1, 2, 3, 4, 5];
        let result = check_ids_offset(&left_ids, &right_ids);
        assert_eq!(result.unwrap(), 0);
    }
    
    #[test]
    fn check_ids_unequal()
    {
        let left_ids: Vec<i32> = vec![3, 2, 1, 0, 2];
        let rigt_ids: Vec<i32> = vec![1, 3, 0, 0, 2];
        let result = check_ids_offset(&left_ids, &rigt_ids);
        assert_eq!(result.unwrap(), 4);
    }
    
    #[test]
    fn check_ids_mismatch()
    {
        let left_ids: Vec<i32> = vec![1, 2, 3];
        let right_ids: Vec<i32> = vec![1, 2];
        let result = check_ids_offset(&left_ids, &right_ids);
        assert!(result.is_err());
    }
}
