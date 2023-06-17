use std::collections::HashMap;

pub struct ValuesMapper {}

impl ValuesMapper {
    pub fn vector_to_hashmap<T, F>(vector: Vec<T>, get_field: F) -> HashMap<i32, T>
    where
        F: Fn(&T) -> i32,
    {
        vector
            .into_iter()
            .map(|item| (get_field(&item), item))
            .collect()
    }
}
