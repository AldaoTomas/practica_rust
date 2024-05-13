use super::ej3::Fecha;

#[derive(Clone)]
enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros
}
#[derive(Clone)]
struct Libro {
    titulo: String,
    autor: String,
    numero_de_paginas: u32,
    genero: Genero
}

#[derive(Clone)]
struct Cliente{
    nombre: String,
    telefono: u32,
    correo: String
}

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
    pub fn obtener_copias(&self, libro: &Libro) -> u32{
        for i in 0.. self.libros_disponibles.len(){
            if self.libros_disponibles[i].libro.equals(&libro){
                return self.libros_disponibles[i].cant;
            }
        }
        0
    }


    pub fn decrementar_copias (&mut self, libro: &Libro) {
        for i in 0..self.libros_disponibles.len(){
            if self.libros_disponibles[i].libro.equals(&libro){
                self.libros_disponibles[i].cant -=1;
            }
        }
    }


    pub fn incrementar_copias(&mut self, libro: &Libro){
        for i in 0..self.libros_disponibles.len(){
            if self.libros_disponibles[i].libro.equals(&libro){
                self.libros_disponibles[i].cant+= 1;
            }
        }
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
        if cantidad_de_prestamos <= 5 && cantidad_copias_disponibles > 0 {
            self.decrementar_copias(&libro);
            let mut prestamo = Prestamos::new(libro, cliente, fehca_de_vencimiento);
            self.prestamos.push(prestamo);
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
                return true;
            }
        }
        false
    }


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
    assert_eq!(biblioteca.obtener_copias(&libro), 5);
}

#[test]
fn test_decrementar_copias() {
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
    biblioteca.decrementar_copias(&libro);
    assert_eq!(biblioteca.obtener_copias(&libro), 4);
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
    assert_eq!(biblioteca.obtener_copias(&libro), 6);
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
    assert_eq!(biblioteca.obtener_copias(&libro), 9);
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
    assert_eq!(biblioteca.obtener_copias(&libro), 10);
    assert_eq!(biblioteca.devolver_libro(&libro2, &cliente.clone(), fecha.clone()),true);
    assert_eq!(biblioteca.obtener_copias(&libro2), 8);
    assert_eq!(
        biblioteca.devolver_libro(&libro3, &cliente.clone(), fecha.clone()),
        true
    );
    assert_eq!(biblioteca.obtener_copias(&libro3), 12);

    assert_eq!(biblioteca.prestamos[0].estado, true);
    assert_eq!(biblioteca.prestamos[1].estado, true);
    assert_eq!(biblioteca.prestamos[2].estado, true);
}