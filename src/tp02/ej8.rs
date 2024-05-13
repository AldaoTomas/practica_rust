
pub fn sumar_arreglos (arr1: [f64;5], arr2: [f64;5]) -> Vec<f64>{
    arr1.iter().zip(arr2.iter()).map(|(&a, &b)| a + b).collect()
}