
pub fn duplicar_valores(arreglo_flotantes: [f64; 4]) -> Vec<f64>{
    return arreglo_flotantes.iter().map(|&numero| numero*2.0).collect();
}