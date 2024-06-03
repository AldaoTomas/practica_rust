use std::collections::HashMap;



struct  Producto {
    nombre: String,
    categoria: Categoria,
    precio_base: f64,
    descuento: Option<f64>,
    vendedor: Vendedor,
}

struct Cliente {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: u32,
    suscripto: Option<String>, //si es some meter correo elctronico
}
#[derive(Clone)]
struct Vendedor{
    legajo: u32,
    antiguedad: u8,
    salario: f64,
}

struct Ventas {
    registro_ventas: Vec<Venta>,
}
struct Venta {
    fecha: String,
    vendedor: Vendedor,
    listado_de_productos: Vec<Producto>,
    cliente: Cliente,
    medio_de_pago: MedioPago,
}

enum MedioPago {
    Efectivo,
    Tarjeta,
    Debito,
    Transferencia,
}
#[derive(PartialEq)]
enum Categoria{
    Celulares,
    PC,
    Laptop,
    Consolas,
}

impl Categoria {
    fn categoria_to_int (&self) -> usize{
        match self {
            Self::Celulares => 0,
            Self::Consolas => 1,
            Self::Laptop => 2,
            Self::PC => 3,
        }
    }
}


impl Ventas {
    fn new (registro_ventas: Vec<Venta>) -> Self{
        Ventas{
            registro_ventas,
        }
    }

    fn agregar_venta (&mut self, venta: Venta) {
        self.registro_ventas.push(venta)
    }

    fn reporte_categorias (&self) -> Vec<i32>{
        let mut vec_contador = vec![0;6];
        for i in 0..self.registro_ventas.len(){
            for j in 0..self.registro_ventas[i].listado_de_productos.len(){
                vec_contador[self.registro_ventas[i].listado_de_productos[j].categoria.categoria_to_int()] += 1
            }
        }

        vec_contador
    }

    fn reporte_vendedores (&self) -> HashMap<u32, i32> {
        let mut contador = HashMap::new();
        for i in 0..self.registro_ventas.len(){
            let entry = contador.entry(self.registro_ventas[i].vendedor.legajo.clone()).or_insert(0);
            *entry += 1;
        }
        return contador;
    }
}


impl Producto{
    fn new (nombre: String, categoria: Categoria, precio_base: f64, descuento: Option<f64>, vendedor: Vendedor,) -> Self{
        Producto{
            nombre,
            categoria,
            precio_base,
            descuento,
            vendedor,
        }
    }


    fn aplicar_descuento (&mut self) -> f64{
        match self.categoria {
            Categoria::Celulares => {
                self.precio_base *= 0.9;
            }
            Categoria::Consolas => {
                self.precio_base *= 0.75;
            }
            _=> {self.precio_base*=1.0;}
        }

        self.precio_base
    }

}


impl Venta {
    fn new (fecha: String, cliente: Cliente, vendedor: Vendedor, medio_de_pago: MedioPago, listado_de_productos: Vec<Producto>) -> Self {
        Venta {
            fecha,
            cliente,
            vendedor,
            medio_de_pago,
            listado_de_productos
        }
    }

    fn calcular_precio_final (&mut self) -> f64{
        let mut total = 0.0;
        for i in 0..self.listado_de_productos.len(){
            total += self.listado_de_productos[i].aplicar_descuento()
        }
        if self.cliente.suscripto.is_some() {
            total *= 0.85
        }
        total
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calcular_precio_final() {
        let vendedor = crear_vendedor();
        let cliente = crear_cliente(Some("tomasaldao@gmail.com".to_string()));
        let producto1 = crear_producto("Producto1", Categoria::Celulares, 1000.0, None, &vendedor);
        let producto2 = crear_producto("Producto2", Categoria::Consolas, 2000.0, None, &vendedor);
        let mut venta = crear_venta("05-06-2004", &cliente, &vendedor, MedioPago::Efectivo, vec![producto1, producto2]);
        
        let precio_final = venta.calcular_precio_final();
        
        assert_eq!(precio_final, 2040.0); 
    }

    #[test]
    fn test_aplicar_descuento() {
        let vendedor = crear_vendedor();
        let mut producto = crear_producto("Producto1", Categoria::Celulares, 1000.0, Some(10.0), &vendedor);
        let precio_con_descuento = producto.aplicar_descuento();
        
        assert_eq!(precio_con_descuento, 900.0);
    }

    #[test]
    fn test_reporte_vendedores() {
        let vendedor1 = crear_vendedor();
        let vendedor2 = Vendedor { legajo: 2, antiguedad: 3, salario: 40000.0 };
        let cliente = crear_cliente(None);
        let producto1 = crear_producto("Producto1", Categoria::Celulares, 1000.0, None, &vendedor1);
        let producto2 = crear_producto("Producto2", Categoria::PC, 2000.0, None, &vendedor2);
        let venta1 = crear_venta("05-06-2004", &cliente, &vendedor1, MedioPago::Efectivo, vec![producto1]);
        let venta2 = crear_venta("07-06-2002", &cliente, &vendedor2, MedioPago::Efectivo, vec![producto2]);
        
        let ventas = Ventas::new(vec![venta1, venta2]);
        let reporte = ventas.reporte_vendedores();
        
        assert_eq!(reporte.get(&1), Some(&1));
        assert_eq!(reporte.get(&2), Some(&1));
    }

    #[test]
    fn test_reporte_categorias() {
        let vendedor = crear_vendedor();
        let cliente = crear_cliente(None);
        let producto1 = crear_producto("Producto1", Categoria::Celulares, 1000.0, None, &vendedor);
        let producto2 = crear_producto("Producto2", Categoria::PC, 2000.0, None, &vendedor);
        let venta = crear_venta("05-06-2004", &cliente, &vendedor, MedioPago::Efectivo, vec![producto1, producto2]);
        
        let ventas = Ventas::new(vec![venta]);
        let reporte = ventas.reporte_categorias();
        
        assert_eq!(reporte, vec![1, 0, 0, 1, 0, 0]);
    }

    #[test]
    fn test_agregar_venta() {
        let vendedor = crear_vendedor();
        let cliente = crear_cliente(None);
        let producto = Producto::new("Producto1".to_string(), Categoria::Celulares, 1000.0, None, vendedor.clone());
        let venta = Venta::new("05-06-2004".to_string(), cliente, vendedor, MedioPago::Efectivo, vec![producto]);
        
        let mut ventas = Ventas::new(vec![]);
        ventas.agregar_venta(venta);
        
        assert_eq!(ventas.registro_ventas.len(), 1);
    }

    fn crear_venta(fecha: &str, cliente: &Cliente, vendedor: &Vendedor, medio_de_pago: MedioPago, productos: Vec<Producto>) -> Venta {
        Venta {
            fecha: fecha.to_string(),
            vendedor: Vendedor {
                legajo: vendedor.legajo,
                antiguedad: vendedor.antiguedad,
                salario: vendedor.salario,
            },
            cliente: Cliente {
                nombre: cliente.nombre.clone(),
                apellido: cliente.apellido.clone(),
                direccion: cliente.direccion.clone(),
                dni: cliente.dni,
                suscripto: cliente.suscripto.clone(),
            },
            medio_de_pago,
            listado_de_productos: productos,
        }
    }

    fn crear_producto(nombre: &str, categoria: Categoria, precio_base: f64, descuento: Option<f64>, vendedor: &Vendedor) -> Producto {
        Producto {
            nombre: nombre.to_string(),
            categoria,
            precio_base,
            descuento,
            vendedor: Vendedor {
                legajo: vendedor.legajo,
                antiguedad: vendedor.antiguedad,
                salario: vendedor.salario,
            },
        }
    }

    fn crear_cliente(suscripto: Option<String>) -> Cliente {
        Cliente {
            nombre: "Tomas".to_string(),
            apellido: "Aldao".to_string(),
            direccion: "Casa Tomas".to_string(),
            dni: 12345678,
            suscripto,
        }
    }

    fn crear_vendedor() -> Vendedor {
        Vendedor {
            legajo: 1,
            antiguedad: 5,
            salario: 50000.0,
        }
    }
}
