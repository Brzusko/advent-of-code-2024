pub mod id_checker
{
    use anyhow::*;
    use std::ops::Sub;
    
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
    
    pub fn check_ids<T>() -> anyhow::Result<u32>
    where
        T: Copy,
        T: Sub,
        T: PartialEq,
        T: Integer,
    {
        todo!()
    }
}


#[cfg(test)]
mod tests {
}
