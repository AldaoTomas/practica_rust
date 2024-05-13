
struct Triangulo {
    lado1: f64,
    lado2: f64,
    lado3: f64
}

impl Triangulo{

    fn new (lado1: f64, lado2: f64, lado3: f64, ) -> Self{
        Triangulo{
            lado1,
            lado2,
            lado3
        }
    }

    fn determinar_tipo (&self) -> String{
        if (self.lado1 == self.lado2 && self.lado1 != self.lado3) || (self.lado1 == self.lado3 && self.lado1 != self.lado2) || (self.lado2 == self.lado3 && self.lado2 != self.lado1) {
            "isosceles".to_string()
        }
        else if self.lado1 != self.lado2 && self.lado1 != self.lado3 && self.lado2 != self.lado3{
            "escaleno".to_string()
        }
        else {
            "equilatero".to_string() 
        }
    }

    fn calcular_area (&self) -> f64{
        // formula de heron
        let s = self.calcular_perimetro() / 2.0;
        (s * (s - self.lado1) * (s - self.lado2) * (s - self.lado3)).sqrt() // El "sqrt" hace raiz de lo q esta entre parentesis
    }

    fn calcular_perimetro (&self) -> f64{
        self.lado1 + self.lado2 + self.lado3
    }

}


#[test]
fn constructor() {
    let triangulo = Triangulo::new(3.0, 4.0, 5.0);
    assert_eq!(triangulo.lado1, 3.0);
    assert_eq!(triangulo.lado2, 4.0);
    assert_eq!(triangulo.lado3, 5.0);
}

#[test]
fn tipo() {
    let triangulo = Triangulo::new(23.0, 14.0, 9.0);
    assert_eq!(triangulo.determinar_tipo(), "escaleno".to_string());
    let triangulo = Triangulo::new(7.0, 7.0, 7.0);
    assert_eq!(triangulo.determinar_tipo(), "equilatero".to_string());
    let triangulo = Triangulo::new(15.0, 15.0, 4.0);
    assert_eq!(triangulo.determinar_tipo(), "isosceles".to_string());
}

#[test]
fn perimetro() {
    let triangulo = Triangulo::new(7.0, 8.0, 5.0);
    assert_eq!(triangulo.calcular_perimetro(), 20.0);
}

#[test]
fn area() {
    let triangulo = Triangulo::new(3.0, 4.0, 5.0);
    assert_eq!(triangulo.calcular_area(), 6.0);
}