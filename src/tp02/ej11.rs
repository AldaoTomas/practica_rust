
pub fn multiplicar_valores(arreglo: [i32; 4], factor: i32) -> Vec<i32>{
    return arreglo.iter().map(|&numero| numero*factor).collect();
}