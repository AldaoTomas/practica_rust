/*En base al ejercicio 10 del tp#3 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b- Tanto los libros con sus copias como la administración de préstamos se realizan
sobre archivos en formato JSON. Realice las modificaciones pertinentes para poder hacerlo
así. No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que
haga métodos nuevos para cumplir con este punto . Recuerde también que se debe seguir
manteniendo un coverage de al menos 90%.
*/
use super::fecha::Fecha;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{self, Display},
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
};

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

#[derive(Clone, Serialize, Deserialize)]
enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros
}
#[derive(Clone, Serialize, Deserialize)]
struct Libro {
    titulo: String,
    autor: String,
    numero_de_paginas: u32,
    genero: Genero
}

#[derive(Clone, Serialize, Deserialize)]
struct Cliente{
    nombre: String,
    telefono: u32,
    correo: String
}

#[derive(Clone, Serialize, Deserialize)]
struct Prestamos{
    libro: Libro,
    cliente: Cliente,
    fehca_de_vencimiento: Fecha,
    fehca_de_devolucion: Option<Fecha>,
    estado: bool // fue devuelto
}


struct Biblioteca {
    nombre: String,
    direccion: String,
    libros_disponibles: Vec<LibrosDisponibles>,
    prestamos: Vec<Prestamos>
}

#[derive(Clone, Serialize, Deserialize)]
struct  LibrosDisponibles{
    libro: Libro,
    cant: u32,
}

impl LibrosDisponibles {
    fn new (libro: Libro, cant: u32) -> LibrosDisponibles{
        LibrosDisponibles{
            libro,
            cant,
        }
    }
}

impl Genero {
    fn genero_to_int (&self) -> u32 {
        match self{
            Self::Infantil => 0,
            Self::Novela => 1,
            Self::Tecnico => 2,
            Self::Otros => 3,
        }
    }

    pub fn equals(&self, genero: &Genero) -> bool {
        self.genero_to_int() == genero.genero_to_int()
    }
}


impl Libro {
    pub fn equals(&self, libro: &Libro) -> bool {
        self.autor == libro.autor && self.titulo == libro.titulo && self.numero_de_paginas == libro.numero_de_paginas && self.genero.equals(&libro.genero)
    }
}

impl Cliente{
    pub fn equals(&self, cliente: &Cliente) -> bool{
        self.nombre == cliente.nombre && self.telefono == cliente.telefono && self.correo == cliente.correo
    }
}


impl Prestamos {
    fn new(libro: Libro, cliente: Cliente, fehca_de_vencimiento: Fecha) -> Prestamos {
        Prestamos {
            libro,
            cliente,
            fehca_de_vencimiento,
            fehca_de_devolucion: None,
            estado: false,
        }
    }
}


impl Biblioteca {
    pub fn new(nombre: String, direccion: String) -> Biblioteca {
        let prestamos_r: Vec<Prestamos> = Vec::new();
        
        // Serializar el vector realizadas a JSON
        let json_realizadas = serde_json::to_string(&prestamos_r).unwrap();
        
        // Crear y escribir en el archivo JSON
        let mut file = File::create("src/tp05/prestamosBiblioteca.json").unwrap();
        file.write_all(json_realizadas.as_bytes()).unwrap();


        let copias: Vec<LibrosDisponibles> = Vec::new();
        
        // Serializar el vector realizadas a JSON
        let json_realizadas = serde_json::to_string(&copias).unwrap();
        
        // Crear y escribir en el archivo JSON
        let mut file = File::create("src/tp05/copiasBiblioteca.json").unwrap();
        file.write_all(json_realizadas.as_bytes()).unwrap();

        Biblioteca {
            nombre,
            direccion,
            libros_disponibles: Vec::new(),
            prestamos: Vec::new(),
        }
    }

    pub fn obtener_copias(&self, libro: &Libro) -> Option<u32>{
        for i in 0.. self.libros_disponibles.len(){
            if self.libros_disponibles[i].libro.equals(&libro){
                return Some(self.libros_disponibles[i].cant);
            }
        }
        None
    }


    pub fn decrementar_copias (&mut self, libro: &Libro) -> Result<(), MiError>{
        for i in 0..self.libros_disponibles.len(){
            if self.libros_disponibles[i].libro.equals(&libro){
                self.libros_disponibles[i].cant -=1;
                self.guardar_copias().unwrap();
                return Ok(());
            }
        }
        Err(MiError{msg: "No se encontro el libro".to_string()})
    }


    pub fn incrementar_copias(&mut self, libro: &Libro) -> Result<(), MiError>{
        for i in 0..self.libros_disponibles.len(){
            if self.libros_disponibles[i].libro.equals(&libro){
                self.libros_disponibles[i].cant+= 1;
                self.guardar_copias().unwrap();
                return Ok(());
            }
        }
        Err(MiError{msg: "No se encontro el libro".to_string()})
    }


    pub fn contar_prestamos(&self, cliente: &Cliente) -> u32{
        let mut a = 0;
        for i in 0.. self.prestamos.len(){
            if self.prestamos[i].cliente.equals(&cliente){
                a += 1;
            }
        }
        a
    }


    pub fn realizar_prestamo(&mut self, libro: Libro, cliente: Cliente, fehca_de_vencimiento: Fecha) -> bool{
        let cantidad_de_prestamos = self.contar_prestamos(&cliente);
        let cantidad_copias_disponibles = self.obtener_copias(&libro);
        if cantidad_de_prestamos <= 5 && cantidad_copias_disponibles != None {
            self.decrementar_copias(&libro);
            let mut prestamo = Prestamos::new(libro, cliente, fehca_de_vencimiento);
            self.prestamos.push(prestamo);
            self.guardar_prestamos().unwrap();
            return true;
        }
        false
    }


    pub fn prestamos_a_vencer(&self, fecha:Fecha) -> Vec<&Prestamos>{
        let mut v = Vec::new();
        for i in 0..self.prestamos.len(){
            if self.prestamos[i].fehca_de_vencimiento.es_mayor(&fecha){
                v.push(&self.prestamos[i]);
            }
        }
        v
    }


    pub fn prestamos_vencidos(&self, fecha:Fecha) -> Vec<&Prestamos>{
        let mut v = Vec::new();
        for i in 0.. self.prestamos.len(){
            if fecha.es_mayor(&self.prestamos[i].fehca_de_vencimiento){
                v.push(&self.prestamos[i])
            }
        }
        v
    }


    pub fn buscar_prestamo (&self, libro: &Libro, cliente: &Cliente) {
        for i in 0..self.prestamos.len(){
            if self.prestamos[i].cliente.equals(cliente) && self.prestamos[i].libro.equals(libro){
                return;
            }
        }
    }


    pub fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente, fecha:Fecha) -> bool{
        for i in 0..self.prestamos.len(){
            if self.prestamos[i].cliente.equals(cliente) && self.prestamos[i].libro.equals(libro){
                self.prestamos[i].estado = true;
                self.prestamos[i].fehca_de_devolucion = Some(fecha);
                self.incrementar_copias(libro);
                self.guardar_prestamos().unwrap();
                return true;
            }
        }
        false
    }

    pub fn guardar_prestamos(&self) -> Result<(), MiError> {
        let json = serde_json::to_string(&self.prestamos)?;
        let mut file = OpenOptions::new().write(true).open("src/tp05/prestamosBiblioteca.json")?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn guardar_copias(&self) -> Result<(), MiError> {
        let json = serde_json::to_string(&self.libros_disponibles)?;
        let mut file = OpenOptions::new().write(true).open("src/tp05/copiasBiblioteca.json")?;
        file.write_all(json.as_bytes())?;
        Ok(())
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
fn test_new_biblioteca () {
    let biblioteca = Biblioteca::new("Biblioteca".to_string(), "Direccion".to_string());
    assert_eq!(biblioteca.nombre, "Biblioteca".to_string());
    assert_eq!(biblioteca.direccion, "Direccion".to_string());
    assert_eq!(biblioteca.libros_disponibles.len(), 0);
    assert_eq!(biblioteca.prestamos.len(), 0);
}



#[test]
fn test_constructor_prestamo() {
    let libro = Libro {
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };
    let cliente = Cliente {
        nombre: "Cliente".to_string(),
        telefono: 123,
        correo: "correo".to_string(),
    };
    let fecha = Fecha::new(1, 1, 2021);
    let prestamo = Prestamos::new(libro.clone(), cliente.clone(), fecha.clone());
    assert_eq!(prestamo.libro.equals(&libro), true);
    assert_eq!(prestamo.cliente.equals(&cliente), true);
    assert_eq!(prestamo.fehca_de_vencimiento.equals(&fecha), true);
    assert_eq!(prestamo.fehca_de_devolucion.is_none(), true);
    assert_eq!(prestamo.estado, false);
}

#[test]
fn test_obtener_copias() {
    let libro = Libro {
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };

    let libros_disp= LibrosDisponibles::new(libro.clone(), 5);
    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: Vec::new(),
        prestamos: Vec::new(),
    };
    biblioteca.libros_disponibles.push(libros_disp);
    assert_eq!(biblioteca.obtener_copias(&libro), Some(5));
}

#[test]
fn test_decrementar_copias() {
    let libro = Libro {
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };
    let libros_disp= LibrosDisponibles::new(libro.clone(), 1);
    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: Vec::new(),
        prestamos: Vec::new(),
    };
    biblioteca.libros_disponibles.push(libros_disp);
    biblioteca.decrementar_copias(&libro);
    assert_eq!(biblioteca.obtener_copias(&libro), Some(0));
    let libro2 = Libro {
        titulo: "Titulo2".to_string(),
        autor: "Autor2".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };
    assert!(biblioteca.decrementar_copias(&libro2).is_err());
}

#[test]
fn test_incrementar_copias() {
    let libro = Libro {
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };
    let libros_disp= LibrosDisponibles::new(libro.clone(), 5);
    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: Vec::new(),
        prestamos: Vec::new(),
    };

    biblioteca.libros_disponibles.push(libros_disp);
    biblioteca.incrementar_copias(&libro);
    assert_eq!(biblioteca.obtener_copias(&libro), Some(6));

    let libro2 = Libro {
        titulo: "Titulo2".to_string(),
        autor: "Autor2".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };
    assert!(biblioteca.incrementar_copias(&libro2).is_err());
}
#[test]
fn test_contar_prestamos_cliente() {
    let libro = Libro {
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };

    let cliente = Cliente {
        nombre: "Cliente".to_string(),
        telefono: 123,
        correo: "correo".to_string(),
    };


    let libros_disp= LibrosDisponibles::new(libro.clone(), 5);
    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: Vec::new(),
        prestamos: Vec::new(),
    };
    let fehca_de_vencimiento = Fecha::new(1, 1, 2021);
    biblioteca.libros_disponibles.push(libros_disp);

    biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fehca_de_vencimiento.clone());
    biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fehca_de_vencimiento.clone());
    biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fehca_de_vencimiento.clone());


    assert_eq!(biblioteca.contar_prestamos(&cliente), 3);
}

#[test]
fn test_realizar_prestamo() {
    let libro = Libro {
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };

    let cliente = Cliente {
        nombre: "Cliente".to_string(),
        telefono: 123,
        correo: "correo".to_string(),
    };

    let libros_disp= LibrosDisponibles::new(libro.clone(), 10);
    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: Vec::new(),
        prestamos: Vec::new(),
    };
    let fehca_de_vencimiento = Fecha::new(1, 1, 2021);
    biblioteca.libros_disponibles.push(libros_disp);
    assert_eq!(
        biblioteca.realizar_prestamo(libro.clone(), cliente, fehca_de_vencimiento),
        true
    );
    assert_eq!(biblioteca.obtener_copias(&libro), Some(9));
    assert_eq!(biblioteca.prestamos.len(), 1);
}

#[test]
fn test_prestamos_a_vencer() {
    let libro = Libro {
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };

    let cliente = Cliente {
        nombre: "Cliente".to_string(),
        telefono: 123,
        correo: "correo".to_string(),
    };
    let libros_disp= LibrosDisponibles::new(libro.clone(), 15);
    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: Vec::new(),
        prestamos: Vec::new(),
    };
    let fehca_de_vencimiento = Fecha::new(1, 1, 2025);

    biblioteca.libros_disponibles.push(libros_disp);
    let fecha = Fecha::new(1, 1, 2024);

    biblioteca.realizar_prestamo(libro.clone(), cliente, fehca_de_vencimiento);

    let prestamos = biblioteca.prestamos_a_vencer(fecha);
    assert_eq!(prestamos.len(), 1);
    assert_eq!(prestamos[0].libro.equals(&libro), true);
}

#[test]
fn test_prestamo_vencidos() {
    let libro = Libro {
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };

    let cliente = Cliente {
        nombre: "Cliente".to_string(),
        telefono: 123,
        correo: "correo".to_string(),
    };

    let libros_disp= LibrosDisponibles::new(libro.clone(), 15);
    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: Vec::new(),
        prestamos: Vec::new(),
    };

    biblioteca.libros_disponibles.push(libros_disp);

    let fecha = Fecha::new(1, 1, 2024);
    let fecha_vencimiento = Fecha::new(1, 2, 2024);
    biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());

    let prestamos = biblioteca.prestamos_vencidos(fecha);
    assert_eq!(prestamos.len(), 0);

    let fecha = Fecha::new(1, 3, 2024);
    let prestamos = biblioteca.prestamos_vencidos(fecha);

    assert_eq!(prestamos.len(), 1);
    assert_eq!(prestamos[0].libro.equals(&libro), true);
}

#[test]
fn test_devolver_libro() {
    let mut libro = Libro {
        titulo: "Titulo".to_string(),
        autor: "Autor".to_string(),
        numero_de_paginas: 100,
        genero: Genero::Novela,
    };
    let mut libro2 = Libro {
        titulo: "Otro Titulo".to_string(),
        autor: "Otro Autor".to_string(),
        numero_de_paginas: 200,
        genero: Genero::Infantil,
    };

    let mut libro3 = Libro {
        titulo: "Tercer Titulo".to_string(),
        autor: "Tercer Autor".to_string(),
        numero_de_paginas: 150,
        genero: Genero::Tecnico,
    };

    let libros_disp= LibrosDisponibles::new(libro.clone(), 10);
    let libros_disp2= LibrosDisponibles::new(libro2.clone(), 8);
    let libros_disp3= LibrosDisponibles::new(libro3.clone(), 12);
    let mut biblioteca = Biblioteca {
        nombre: "Biblioteca".to_string(),
        direccion: "Direccion".to_string(),
        libros_disponibles: Vec::new(),
        prestamos: Vec::new(),
    };

    let cliente = Cliente {
        nombre: "Cliente".to_string(),
        telefono: 123,
        correo: "correo".to_string(),
    };

    biblioteca.libros_disponibles.push(libros_disp);
    biblioteca.libros_disponibles.push(libros_disp2);
    biblioteca.libros_disponibles.push(libros_disp3);

    let fecha = Fecha::new(1, 1, 2024);
    let fecha_vencimiento = Fecha::new(1, 2, 2024);

    biblioteca.realizar_prestamo(libro.clone(), cliente.clone(), fecha_vencimiento.clone());
    biblioteca.realizar_prestamo(libro2.clone(), cliente.clone(), fecha_vencimiento.clone());
    biblioteca.realizar_prestamo(libro3.clone(), cliente.clone(), fecha_vencimiento.clone());

    assert_eq!(biblioteca.devolver_libro(&libro, &cliente.clone(), fecha.clone()),true);
    assert_eq!(biblioteca.obtener_copias(&libro), Some(10));
    assert_eq!(biblioteca.devolver_libro(&libro2, &cliente.clone(), fecha.clone()),true);
    assert_eq!(biblioteca.obtener_copias(&libro2), Some(8));
    assert_eq!(
        biblioteca.devolver_libro(&libro3, &cliente.clone(), fecha.clone()),
        true
    );
    assert_eq!(biblioteca.obtener_copias(&libro3), Some(12));

    assert_eq!(biblioteca.prestamos[0].estado, true);
    assert_eq!(biblioteca.prestamos[1].estado, true);
    assert_eq!(biblioteca.prestamos[2].estado, true);
}