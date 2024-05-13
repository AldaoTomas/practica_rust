#[derive(Clone)]
struct Auto {
    marca: String,
    modelo: String,
    año: i32,
    precio_bruto: f64,
    color: Color
}
#[derive(Clone)]
enum Color {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro
}

impl Color {
    pub fn color_to_int(&self) -> u8 {
        match self {
            Self::Rojo => 0,
            Self::Verde => 1,
            Self::Azul => 2,
            Self::Amarillo => 3,
            Self::Blanco => 4,
            Self::Negro => 5,
        }
    }

    pub fn equals (&self, color: &Color)-> bool {
        self.color_to_int() == color.color_to_int()
    }

}



struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    capacidad: u32,
    autos: Vec<Auto>
}

impl ConcesionarioAuto {

    fn new (nombre: String, direccion: String, capacidad: u32, autos: Vec<Auto> ) -> Self {
        ConcesionarioAuto{
            nombre,
            direccion,
            capacidad,
            autos
        }
    }

    fn agregar_auto (&mut self, auto: Auto) -> bool{
        if self.autos.len() < self.capacidad as usize{
            self.autos.push(auto);
            return true;
        }
        false
    }


    fn eliminar_auto (&mut self, auto: Auto) {
        for i in 0..self.autos.len() {
            let aux = &self.autos[i];
            if aux.año == auto.año
                && aux.marca == auto.marca
                && aux.precio_bruto == auto.precio_bruto
                && aux.modelo == auto.modelo
                && aux.color.equals(&auto.color)
            {
                self.autos.remove(i);
                break;
            }
        }
    }

    fn buscar_auto (&self, auto: Auto) -> Option<&Auto>{
        for i in 0..self.autos.len() {
            let aux = &self.autos[i];
            if aux.año == auto.año
                && aux.marca == auto.marca
                && aux.precio_bruto == auto.precio_bruto
                && aux.modelo == auto.modelo
                && aux.color.equals(&auto.color)
            {
                return Some(&self.autos[i]);
                
            }
        }
        return None;
    }


}


impl Auto {

    fn new (marca: String, modelo: String, año: i32, precio_bruto: f64, color: Color) -> Self{
        Auto {
            marca,
            modelo,
            año,
            precio_bruto,
            color
        }
    }


    fn calcular_precio (&self) -> f64{
        let mut total: f64 = self.precio_bruto;
        if self.color.color_to_int()==0 || self.color.color_to_int()== 1 || self.color.color_to_int()== 3  {
            total += total/100.0 * 25.0;
        }
        else {
            total -= total/100.0 * 10.0;
        }

        if self.marca == "BMW" {
            total+= total/100.0 * 15.0;
        }

        if self.año < 2000 {
            total -= total/100.0 * 5.0;
        }


        return total;
    }

    
}


#[test]
fn buscar_auto() {
    let auto = Auto::new(
        "Renault".to_string(),
        "Kangoo".to_string(),
        1998,
        10000.0,
        Color::Rojo,
    );
    let auto2 = Auto::new(
        "Peugeot".to_string(),
        "308".to_string(),
        2007,
        10000.0,
        Color::Verde,
    );
    let auto3 = Auto::new(
        "BMW".to_string(),
        "Sedan".to_string(),
        1927,
        10000.0,
        Color::Negro,
    );
    let mut concesionario = ConcesionarioAuto::new(
        "Concesionario".to_string(),
        "Direccion".to_string(),
        2,
        vec![],
    );

    concesionario.agregar_auto(auto.clone());
    concesionario.agregar_auto(auto2);
    let encontre: bool = concesionario.buscar_auto(auto).is_some();
    let no_encontre: bool = concesionario.buscar_auto(auto3).is_none();
    assert_eq!(encontre, true);
    assert_eq!(no_encontre, true);
}


#[test]
fn eliminar_auto() {
    let auto = Auto::new(
        "Renault".to_string(),
        "Kangoo".to_string(),
        1998,
        10000.0,
        Color::Rojo,
    );
    let auto2 = Auto::new(
        "Peugeot".to_string(),
        "308".to_string(),
        2007,
        10000.0,
        Color::Verde,
    );
    let mut concesionario = ConcesionarioAuto::new(
        "Concesionario".to_string(),
        "Direccion".to_string(),
        2,
        vec![],
    );
    concesionario.agregar_auto(auto);
    concesionario.agregar_auto(auto2.clone());
    concesionario.eliminar_auto(auto2);
    assert_eq!(concesionario.autos.len(), 1);
}


#[test]
fn agregar_auto() {
    let auto = Auto::new(
        "Audi".to_string(),
        "A5".to_string(),
        2007,
        10000.0,
        Color::Rojo,
    );

    let mut concesionario = ConcesionarioAuto::new(
        "Concesionario".to_string(),
        "Direccion".to_string(),
        1,
        vec![],
    );
    assert_eq!(concesionario.agregar_auto(auto.clone()), true);
    assert_eq!(concesionario.agregar_auto(auto), false);
}


#[test]
fn calcular_precio() {
    let auto = Auto::new(
        "BMW".to_string(),
        "Sedan".to_string(),
        1927,
        10000.0,
        Color::Rojo,
    );
    assert_eq!(auto.calcular_precio().round(), 13656.0);
}

#[test]
fn constructor() {
    let auto = Auto::new(
        "Fiat".to_string(),
        "Uno".to_string(),
        2006,
        10000.0,
        Color::Rojo,
    );
    assert_eq!(auto.marca, "Fiat".to_string());
    assert_eq!(auto.modelo, "Uno".to_string());
    assert_eq!(auto.año, 2006);
    assert_eq!(auto.precio_bruto, 10000.0);
    assert_eq!(auto.color.color_to_int(), 0);
}