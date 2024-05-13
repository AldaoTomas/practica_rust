
pub fn longitud_de_cadenas(array_cadenas: [&str; 5]) -> Vec<usize>{
    array_cadenas.iter().map(|cadena| cadena.len()).collect()
}