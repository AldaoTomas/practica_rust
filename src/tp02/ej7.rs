
pub fn cantidad_de_mayores (array_enteros: [i32; 5], limite: i32) -> usize {
    array_enteros.iter().filter(|&x| *x > limite).count()
}