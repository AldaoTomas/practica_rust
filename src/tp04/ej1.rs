


trait EsPrimo {
    fn es_primo(&self) -> bool;
}

impl EsPrimo for i32{
    fn es_primo(&self) -> bool {
        if *self <= 1 {
            return false; 
        }
        for i in 2..*self {
            if self % i == 0{
                return false;
            }
        }
        return true;
    }
}
fn cantidad_primos (v: &Vec<i32>) -> usize {
    v.iter().filter(|x| x.es_primo()).count()
}


#[test]
fn test_cantidad_primos () {
    let vec = vec![1, 3, 4, 5, 7, 8, 9];
    assert_eq!(cantidad_primos(&vec), 3);

    let v = vec![1, 4, 6, 8, 9];
    assert_eq!(cantidad_primos(&v), 0);

    let vector = vec![2, 3, 5, 7];
    assert_eq!(cantidad_primos(&vector), 4);
}