pub fn linear_search<T>(key: T, list: &Vec<T>) -> Result<usize,bool>
where
    T: PartialEq,
{
    for (pos, n) in list.iter().enumerate() {
        if *n == key {
            return Ok(pos)
        }
    }
    return Err(false)
}

