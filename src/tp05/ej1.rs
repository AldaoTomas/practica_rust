/*
a- Al agregar un auto si supera el límite de la concesionaria debe arrojar un error
propio con un mensaje de contexto.
b- Haga todos los tests correspondientes para probar en profundidad los métodos
que agregan un auto y eliminan un auto de la concesionaria , obteniendo el mayor
porcentaje de coverage sobre el código que realiza las operaciones.
c- Una vez hecho el punto anterior debe hacer que los autos de la concesionaria se
almacenen en un archivo en formato JSON. Agregue y modifique lo que considere
necesario para que:
- Al agregar un nuevo auto se abre el archivo de autos guardados y lo agregue a
dicho archivo.
- Eliminar un auto: al eliminar un auto se debe eliminar este del archivo.
No debe modificar los tests hechos en el punto b. Si puede agregar más en caso de que
haga nueva funcionalidad..
*/ 

use serde::{Serialize, Deserialize};
use std::{
    error::Error,
    fmt::{self, Display},
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct Auto {
    marca: String,
    modelo: String,
    año: i32,
    precio_bruto: f64,
    color: Color
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
enum Color {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro
}


#[derive(Debug)]
struct MiError {
    msg: String,
}
impl std::error::Error for MiError {}

impl std::fmt::Display for  MiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<std::io::Error> for MiError {
    fn from(error: std::io::Error) -> Self {
        MiError { msg: error.to_string() }
    }
}

impl From<serde_json::Error> for MiError {
    fn from(error: serde_json::Error) -> Self {
        MiError { msg: error.to_string() }
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

    fn agregar_auto (&mut self, auto: Auto) -> Result<(), MiError>{
        if self.autos.len() < self.capacidad as usize{
            self.autos.push(auto);
            self.guardar_autos()?;

            return Ok(());
        }
        else {
            return Err(MiError{msg: "No hay mas lugar en la concesionaria".to_string()});
        }
        
    }


    fn eliminar_auto (&mut self, auto: Auto) -> Result<(), MiError>{
        for i in 0..self.autos.len() {
            let aux = self.autos[i].clone();
            if aux == auto
            {
                self.autos.remove(i);
                break;
            }
        }

        self.guardar_autos()?;

        Ok(())
    }

    fn guardar_autos(&self) -> Result<(), MiError> {
        let path = "src/tp05/autos.json";
        let json = serde_json::to_string(&self.autos)?;
        let mut archivo = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
        archivo.write_all(json.as_bytes())?;
        Ok(())
    }

    fn cargar_autos(&mut self) -> Result<(), MiError> {
        let path = "src/tp05/autos.json";
        let mut archivo = OpenOptions::new().read(true).open(path)?;
        let mut contenido = String::new();
        archivo.read_to_string(&mut contenido)?;
        self.autos = serde_json::from_str(&contenido)?;
        Ok(())
    }

    fn buscar_auto (&self, auto: Auto) -> Option<&Auto>{
        for i in 0..self.autos.len() {
            let aux = &self.autos[i];
            if aux.año == auto.año
                && aux.marca == auto.marca
                && aux.precio_bruto == auto.precio_bruto
                && aux.modelo == auto.modelo
                && aux.color == auto.color
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
        if self.color == Color::Rojo || self.color == Color::Verde || self.color == Color::Amarillo  {
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



#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use serde_json;

    #[test]
    fn test_display_mierror() {
        let mi_error = MiError { msg: "Este es un error".to_string() };
        assert_eq!(format!("{}", mi_error), "Este es un error");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "Archivo no encontrado");
        let mi_error: MiError = io_error.into();
        assert_eq!(mi_error.msg, "Archivo no encontrado");
    }

    #[test]
    fn test_serde_error_conversion() {
        let json_str = "{ invalid json }";
        let result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(json_str);
        assert!(result.is_err());

        if let Err(error) = result {
            let mi_error: MiError = error.into();
            assert_eq!(mi_error.msg, "key must be a string at line 1 column 3"); 
        }
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
    assert_eq!(concesionario.agregar_auto(auto.clone()).is_ok(), true);
    assert_eq!(concesionario.agregar_auto(auto).is_err(), true);
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
    assert_eq!(auto.color == Color::Rojo, true);
}
#[test]
fn guardar_autos() {
    let auto = Auto::new("Audi".to_string(), "A5".to_string(), 2007, 10000.0, Color::Rojo);
    let mut concesionario = ConcesionarioAuto::new("Concesionario".to_string(), "Direccion".to_string(), 1, vec![]);
    concesionario.agregar_auto(auto).unwrap();
    concesionario.guardar_autos().unwrap();

    let mut concesionario2 = ConcesionarioAuto::new("Concesionario".to_string(), "Direccion".to_string(), 1, vec![]);
    concesionario2.cargar_autos().unwrap();
    assert_eq!(concesionario.autos, concesionario2.autos);
}

#[test]
fn eliminar_auto_del_archivo() {
    let auto = Auto::new("Audi".to_string(), "A5".to_string(), 2007, 10000.0, Color::Rojo);
    let auto2 = Auto::new("BMW".to_string(), "Sedan".to_string(), 1927, 10000.0, Color::Negro);
    let mut concesionario = ConcesionarioAuto::new("Concesionario".to_string(), "Direccion".to_string(), 2, vec![]);
    concesionario.agregar_auto(auto.clone()).unwrap();
    concesionario.agregar_auto(auto2.clone()).unwrap();

    concesionario.eliminar_auto(auto).unwrap();
    concesionario.guardar_autos().unwrap();

    let mut concesionario2 = ConcesionarioAuto::new("Concesionario".to_string(), "Direccion".to_string(), 2, vec![]);
    concesionario2.cargar_autos().unwrap();
    assert_eq!(concesionario2.autos.len(), 1);
    assert!(concesionario2.buscar_auto(auto2.clone()).is_some());
}
#[test]
fn test_kapum () {
    let auto = Auto::new("Audi".to_string(), "A5".to_string(), 2007, 10000.0, Color::Rojo);
    let mut concesionario = ConcesionarioAuto::new("Concesionario".to_string(), "Direccion".to_string(), 0, vec![]);

    assert!(concesionario.agregar_auto(auto.clone()).is_err());
}