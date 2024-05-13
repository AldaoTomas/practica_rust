
pub fn cantidad_impares(arreglo: [i32;4]) -> i32{
    let mut sum=0;
    for numero in arreglo {
        if numero%2!=0 {
            sum+=1;
        }
    }
    return sum;
}