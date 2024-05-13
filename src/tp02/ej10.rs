
pub fn cantidad_cadenas_mayor_a (arreglo_cadenas: [&str; 5], limite: usize) -> usize{
    arreglo_cadenas.iter().filter(|&x| x.len()> limite).count()
}