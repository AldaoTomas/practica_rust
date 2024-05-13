
struct Producto {
    nombre: String,
    precio_bruto: f64,
    id: u32
}

impl Producto{

    fn new (nombre: String, precio_bruto: f64, id: u32) -> Self{
        Producto{
            nombre,
            precio_bruto,
            id
        }
    }


    fn calcular_impuestos (&self, porcentaje_de_impuestos: Option<f64> ) -> f64 {
        match porcentaje_de_impuestos {
            Some(porcentaje) => self.precio_bruto / 100.0 * porcentaje,
            None => 0.0,
        }
    }


    fn aplicar_descuento(&self, porcentaje_de_descuento: Option<f64>) -> f64 {
        match porcentaje_de_descuento {
            Some(porcentaje) => self.precio_bruto / 100.0 * porcentaje,
            None => 0.0,
        }
    }


    fn calcular_precio_total (&self, porcentaje_de_impuestos: Option<f64>, porcentaje_de_descuento: Option<f64>) -> f64{
        self.precio_bruto + self.calcular_impuestos(porcentaje_de_impuestos) - self.aplicar_descuento(porcentaje_de_descuento)
    }
}



#[test]
fn constructor() {
    let producto = Producto::new("Prod1".to_string(), 100.0, 1);
    assert_eq!(producto.nombre, "Prod1".to_string());
    assert_eq!(producto.precio_bruto, 100.0);
    assert_eq!(producto.id, 1);
}

#[test]
fn impuestos() {
    let producto = Producto::new("Prod2".to_string(), 100.0, 2);
    assert_eq!(producto.calcular_impuestos(Some(10.0)), 10.0);
    assert_eq!(producto.calcular_impuestos(None), 0.0);
}

#[test]
fn descuentos() {
    let producto = Producto::new("Prod3".to_string(), 100.0, 3);
    assert_eq!(producto.aplicar_descuento(Some(10.0)), 10.0);
    assert_eq!(producto.aplicar_descuento(None), 0.0);
}

#[test]
fn total (){
    let producto = Producto::new("Prod4".to_string(), 100.0, 4);
    assert_eq!(producto.calcular_precio_total(Some(10.0), Some(20.0)), 90.0);
    assert_eq!(producto.calcular_precio_total(None, Some(20.0)), 80.0);
    assert_eq!(producto.calcular_precio_total(Some(10.0), None), 110.0);
    assert_eq!(producto.calcular_precio_total(None, None), 100.0);
}
