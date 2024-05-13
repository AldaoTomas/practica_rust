
pub fn suma_pares(arreglo: [i32;4]) -> i32{
    let mut sum=0;
    for numero in arreglo {
        if numero%2==0 {
            sum+=numero;
        }
    }
    return sum;
}