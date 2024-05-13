struct Rectangulo {
    longitud: f64,
    ancho: f64,
}

impl Rectangulo{
    
    fn new (longitud: f64, ancho: f64) -> Self{
        Rectangulo{
            longitud,
            ancho,
        }
    }

    fn calcular_area (&self) -> f64{
        self.longitud * self.ancho
    }

    fn calcular_perimetro (&self) -> f64{
        (self.longitud * 2.0)+(self.ancho * 2.0)
    }

    fn es_cuadrado (&self) -> bool{
        self.longitud == self.ancho
    }
}

#[test]
fn rectangulo_test (){
    let rect = Rectangulo::new(5.0, 3.0);

    assert_eq!(rect.calcular_perimetro(), 16.0);
    assert_eq!(rect.es_cuadrado(), false);

    assert_eq!(rect.calcular_area(), 15.0);
}