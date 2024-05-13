
pub fn reemplazar_pares (arreglo: &mut[i32; 4]) {
    arreglo.iter_mut().for_each(|num | { if *num % 2 == 0 {*num = -1}})
}