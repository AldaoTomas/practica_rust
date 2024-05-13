
struct Examen {
    materia: String,
    nota: f64
}

struct Estudiante {
    nombre: String,
    id: u32,
    calificaciones: Vec<Examen>
}


impl Examen {
    fn new (materia: String, nota: f64) -> Self{
        Examen {
            materia,
            nota
        }
    }
}

impl Estudiante {
    fn new (nombre: String, id:u32, calificaciones: Vec<Examen>) -> Self{
        Estudiante{
            nombre,
            id,
            calificaciones
        }
    }

    fn obtener_promedio (&self) -> f64{
        let mut total = 0.0;
        for examen in &self.calificaciones {
            total += examen.nota;
        }
        total / self.calificaciones.len() as f64
    }


    fn obtener_calificaciones_mas_alta (&self) -> f64{
        let mut max: f64 = -1.0;
        for examen in &self.calificaciones {
            if examen.nota > max {
                max = examen.nota;
            }
        }
        max
    }


    fn obtener_calificaciones_mas_baja (&self) -> f64{
        let mut min: f64 = 11.0;
        for examen in &self.calificaciones {
            if examen.nota < min {
                min = examen.nota;
            }
        }
        min
    }


}



#[test]
fn constructor() {
    let examen = Examen::new("CADP".to_string(), 8.0);
    assert_eq!(examen.materia, "CADP".to_string());
    assert_eq!(examen.nota, 8.0);
}

#[test]
fn test() {
    let examen1 = Examen::new("CADP".to_string(), 10.0);
    let examen2 = Examen::new("OC".to_string(), 6.0);
    let examen3 = Examen::new("MATE1".to_string(), 8.0);
    let estudiante = Estudiante {
        nombre: "Tomas".to_string(),
        id: 15,
        calificaciones: vec![examen1, examen2, examen3],
    };

    assert_eq!(estudiante.obtener_promedio(), 8.0);
    assert_eq!(estudiante.obtener_calificaciones_mas_alta(), 10.0);
    assert_eq!(estudiante.obtener_calificaciones_mas_baja(), 6.0);
}
