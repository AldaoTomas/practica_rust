
pub fn cantidad_rango (arr: [i32; 6], inferior: i32, superior: i32)-> i32{
    arr.iter().filter(|&x| *x>=inferior && *x<=superior).count().try_into().unwrap()
}