use std::collections::VecDeque;
use super::ej3::Fecha;


struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    cola_atenciones: VecDeque<Mascota>,
    realizadas: Vec<Atencion>
}

#[derive(Clone)]
enum Tipo {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Clone)]
struct Atencion {
    datos_mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    fecha: Option<Fecha>,
}

#[derive(Clone)]
struct Mascota {
    nombre: String,
    edad: u32,
    tipo: Tipo,
    dueño: Dueño,
}

#[derive(Clone)]
struct Dueño{
    nombre: String,
    direccion: String,
    telefono: u32,
}

impl Tipo {
    fn tipo_to_int(&self) -> u8 {
        match *self{
            Self::Perro => 0,
            Self::Gato => 1,
            Self::Caballo => 2,
            Self::Otros => 3,
        }
    }

    pub fn equals(&self, tipo: &Tipo)-> bool {
        self.tipo_to_int() == tipo.tipo_to_int()
    }
}

impl Dueño{
    pub fn equals(&self, dueño: &Dueño)-> bool{
        self.nombre == dueño.nombre && self.telefono == dueño.telefono && self.direccion == dueño.direccion
    }   	
}

impl Mascota {
    pub fn equals(&self, mascota: &Mascota)-> bool{ 
        self.nombre == mascota.nombre && self.edad == mascota.edad && self.dueño.equals(&mascota.dueño) && self.tipo.equals(&mascota.tipo)
    }
}

impl Fecha {
    pub fn equals (&self, fecha: &Fecha) -> bool{
        self.dia == fecha.dia && self.mes == fecha.mes && self.año == fecha.año
    }
}


impl Atencion {

    fn fecha_equals (&self, fecha2: &Option<Fecha>) -> bool {
        match (&self.fecha, fecha2){
            (Some(fecha), Some(fecha2)) => fecha.equals(&fecha2),
            (None, None) => true,
            _ => false,
        }
    }

    pub fn equals (&self, atencion: &Atencion)-> bool{
        self.datos_mascota.equals(&atencion.datos_mascota) && self.diagnostico == atencion.diagnostico && self.tratamiento == atencion.tratamiento && self.fecha_equals(&atencion.fecha)
    }
}

impl Veterinaria {
    fn new(nombre: String, direccion: String, id:u32) -> Self{
        Veterinaria{
            nombre,
            direccion,
            id,
            cola_atenciones: VecDeque::new(),
            realizadas: Vec::new(),
        }
    }

    fn agregar_mascota(&mut self, mascota: Mascota){
        self.cola_atenciones.push_back(mascota)
    }

    fn agregar_mascota_prioridad(&mut self, mascota: Mascota){
        self.cola_atenciones.push_front(mascota)
    }

    fn atender_mascota(&mut self){
        self.cola_atenciones.pop_front();
    }

    fn eliminar_mascota(&mut self, mascota: Mascota){
        for i in 0..self.cola_atenciones.len(){
            if self.cola_atenciones[i].equals(&mascota){
                self.cola_atenciones.remove(i);
                break;
            }
        }
    }

    fn registrar_atencion(&mut self, atencion:Atencion){
        self.realizadas.push(atencion);
    }

    fn buscar_atencion(&mut self, nombre_mascota: String, nombre_dueño: String, telefono:u32) -> Option<&Atencion>{
        for i in 0..self.realizadas.len(){
            if self.realizadas[i].datos_mascota.nombre == nombre_mascota && self.realizadas[i].datos_mascota.dueño.nombre == nombre_dueño && self.realizadas[i].datos_mascota.dueño.telefono == telefono {
                return Some(&self.realizadas[i]);
            }
        }
        None
    }

    fn modificar_diagnostico(&mut self, atencion:&Atencion, diagnostico: String) {
        for i in 0..self.realizadas.len() {
            if self.realizadas[i].equals(&atencion) {
                self.realizadas[i].diagnostico = diagnostico;
                break;
            }
        }
    }

    fn modificar_fecha (&mut self, atencion:&Atencion, fecha: Option<Fecha>){
        for i in 0..self.realizadas.len(){
            if self.realizadas[i].equals(&atencion){
                self.realizadas[i].fecha = fecha;
                break;
            }
        }
    }

    fn eliminar_atencion(&mut self, atencion:Atencion) {
        for i in 0..self.realizadas.len(){
            if self.realizadas[i].equals(&atencion){
                self.realizadas.remove(i);
                break;
            }
        }
    }

}

#[test]
fn test_eliminar() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        
    );
    let mascota = Mascota {
        nombre: "Mascota".to_string(),
        edad: 1,
        tipo: Tipo::Perro,
        dueño: Dueño {
            nombre: "Dueño".to_string(),
            direccion: "Direccion".to_string(),
            telefono: 123,
        },
    };
    let atencion = Atencion {
        datos_mascota: mascota.clone(),
        diagnostico: "Diagnostico".to_string(),
        tratamiento: "Tratamiento".to_string(),
        fecha: Some(Fecha::new(1, 1, 2024)),
    };
    veterinaria.registrar_atencion(atencion.clone());
    veterinaria.eliminar_atencion(atencion.clone());
    assert!(veterinaria.realizadas.is_empty());
}


#[test]
fn test_modificar_fecha() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        
    );
    let mascota = Mascota {
        nombre: "Mascota".to_string(),
        edad: 1,
        tipo: Tipo::Perro,
        dueño: Dueño {
            nombre: "Dueño".to_string(),
            direccion: "Direccion".to_string(),
            telefono: 123,
        },
    };
    let atencion = Atencion {
        datos_mascota: mascota.clone(),
        diagnostico: "Diagnostico".to_string(),
        tratamiento: "Tratamiento".to_string(),
        fecha: Some(Fecha::new(1, 1, 2024)),
    };
    veterinaria.registrar_atencion(atencion.clone());
    veterinaria.modificar_fecha(&atencion, Some(Fecha::new(2, 2, 2024)));
    assert_eq!(veterinaria.realizadas[0].fecha_equals(&Some(Fecha::new(2, 2, 2024))),true);
}


#[test]
fn test_modificar_diagnostico() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        
    );
    let mascota = Mascota {
        nombre: "Paco".to_string(),
        edad: 2,
        tipo: Tipo::Perro,
        dueño: Dueño {
            nombre: "Carlitos".to_string(),
            direccion: "CasaCarlitos".to_string(),
            telefono: 6666666,
        },
    };
    let atencion = Atencion {
        datos_mascota: mascota.clone(),
        diagnostico: "Diagnostico".to_string(),
        tratamiento: "Tratamiento".to_string(),
        fecha: Some(Fecha::new(1, 1, 2021)),
    };
    veterinaria.registrar_atencion(atencion.clone());
    veterinaria.modificar_diagnostico(&atencion, "Nuevo Diagnostico".to_string());
    assert_eq!(
        veterinaria.realizadas[0].diagnostico,
        "Nuevo Diagnostico".to_string()
    );
}


#[test]
fn test_buscar_atencion() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        
    );
    let mascota = Mascota {
        nombre: "Rex".to_string(),
        edad: 7,
        tipo: Tipo::Perro,
        dueño: Dueño {
            nombre: "Tomas".to_string(),
            direccion: "CasaTomas".to_string(),
            telefono: 1111111,
        },
    };
    let atencion = Atencion {
        datos_mascota: mascota.clone(),
        diagnostico: "Diagnostico".to_string(),
        tratamiento: "Tratamiento".to_string(),
        fecha: Some(Fecha::new(1, 1, 2021)),
    };
    veterinaria.registrar_atencion(atencion.clone());

    let encontre = veterinaria.buscar_atencion("Rex".to_string(), "Tomas".to_string(), 1111111).is_some();
    let no_encontre = veterinaria.buscar_atencion("Paris".to_string(), "Jazmin".to_string(), 1010101).is_none();
    assert_eq!(encontre, true);
    assert_eq!(no_encontre, true);
}


#[test]
fn test_registrar_atencion() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        
    );
    let mascota = Mascota {
        nombre: "Luna".to_string(),
        edad: 7,
        tipo: Tipo::Perro,
        dueño: Dueño {
            nombre: "Tomas".to_string(),
            direccion: "CasaTomas".to_string(),
            telefono: 1111111,
        },
    };
    let atencion = Atencion {
        datos_mascota: mascota.clone(),
        diagnostico: "Diagnostico".to_string(),
        tratamiento: "Tratamiento".to_string(),
        fecha: Some(Fecha::new(1, 1, 2021)),
    };
    veterinaria.registrar_atencion(atencion.clone());
    assert_eq!(veterinaria.realizadas[0].equals(&atencion), true);
}


#[test]
fn test_eliminar_mascota() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        
    );
    let mascota1 = Mascota {
        nombre: "Megatron".to_string(),
        edad: 1,
        tipo: Tipo::Gato,
        dueño: Dueño {
            nombre: "Karen".to_string(),
            direccion: "CasaKaren".to_string(),
            telefono: 7777777,
        },
    };
    let mascota2 = Mascota {
        nombre: "Paris".to_string(),
        edad: 6,
        tipo: Tipo::Perro,
        dueño: Dueño {
            nombre: "Jazmin".to_string(),
            direccion: "CasaJaz".to_string(),
            telefono: 1010101,
        },
    };
    let mascota3 = Mascota {
        nombre: "Corcho".to_string(),
        edad: 8,
        tipo: Tipo::Perro,
        dueño: Dueño {
            nombre: "Pablo".to_string(),
            direccion: "CasaPablo".to_string(),
            telefono: 3333333,
        },
    };
    veterinaria.agregar_mascota(mascota1.clone());
    veterinaria.agregar_mascota(mascota2.clone());
    veterinaria.agregar_mascota(mascota3.clone());
    veterinaria.eliminar_mascota(mascota2.clone());
    assert!(veterinaria.cola_atenciones.len() == 2);
}


#[test]
fn test_atender_mascota() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        
    );
    let mascota = Mascota {
        nombre: "Raul".to_string(),
        edad: 15,
        tipo: Tipo::Perro,
        dueño: Dueño {
            nombre: "Juan".to_string(),
            direccion: "CasaJuan".to_string(),
            telefono: 1234567,
        },
    };
    veterinaria.agregar_mascota(mascota.clone());
    veterinaria.atender_mascota();
    assert_eq!(veterinaria.cola_atenciones.is_empty(), true);
}


#[test]
fn test_agregar_mascota_prioridad() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        5, // Capacidad para 5 mascotas
        
    );
    let mascota1 = Mascota {
        nombre: "Luna".to_string(),
        edad: 7,
        tipo: Tipo::Perro,
        dueño: Dueño {
            nombre: "Tomas".to_string(),
            direccion: "CasaTomas".to_string(),
            telefono: 1111111,
        },
    };
    let mascota2 = Mascota {
        nombre: "Rex".to_string(),
        edad: 7,
        tipo: Tipo::Perro,
        dueño: Dueño {
            nombre: "Tomas".to_string(),
            direccion: "CasaTomas".to_string(),
            telefono: 1111111,
        },
    };
    let mascota3 = Mascota {
        nombre: "Roco".to_string(),
        edad: 9,
        tipo: Tipo::Perro,
        dueño: Dueño {
            nombre: "Tobias".to_string(),
            direccion: "CasaTobias".to_string(),
            telefono: 2069274,
        },
    };
    veterinaria.agregar_mascota(mascota1.clone());
    veterinaria.agregar_mascota(mascota2.clone());
    veterinaria.agregar_mascota_prioridad(mascota3.clone());

    assert_eq!(veterinaria.cola_atenciones[0].equals(&mascota3), true);
}


#[test]
fn test_agregar_mascota() {
    let mut veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        
    );
    let mascota = Mascota {
        nombre: "Luna".to_string(),
        edad: 7,
        tipo: Tipo::Perro,
        dueño: Dueño {
            nombre: "Tomas".to_string(),
            direccion: "CasaTomas".to_string(),
            telefono: 123,
        },
    };
    veterinaria.agregar_mascota(mascota.clone());
    assert_eq!(veterinaria.cola_atenciones[0].equals(&mascota), true);
}


#[test]
fn constructor() {
    let veterinaria = Veterinaria::new(
        "Veterinaria".to_string(),
        "Direccion".to_string(),
        1,
        
    );
    assert_eq!(veterinaria.nombre, "Veterinaria".to_string());
    assert_eq!(veterinaria.direccion, "Direccion".to_string());
    assert_eq!(veterinaria.id, 1);
    assert!(veterinaria.cola_atenciones.is_empty());
    assert!(veterinaria.realizadas.is_empty());
}