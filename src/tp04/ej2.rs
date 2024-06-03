
#[derive(Debug, PartialEq, Clone)]
struct Persona<'a>{
    nombre:&'a str,
    apellido:&'a str,
    direccion:&'a str,
    ciudad:&'a str,
    salario:f64,
    edad:u8,
    }


    fn es_mayor <'a>(v_personas: &'a Vec<Persona<'a>>, sueldo: f64) -> Vec<&'a Persona<'a>> {
        v_personas.iter().filter(|x| x.salario > sueldo).collect()
    }

    fn mayores_edad <'a> (v_personas: &'a Vec<Persona<'a>>, edad: u8, ciudad: &str) -> Vec<&'a Persona<'a>> {
        v_personas.iter().filter(|x| x.edad>edad && x.ciudad == ciudad).collect()
    }

    fn viven_en_la_ciudad <'a> (v_personas: &'a Vec<Persona<'a>>, ciudad: &str) -> bool {
        v_personas.iter().all(|x| x.ciudad == ciudad)
    }

    fn alguno_vive_en_la_ciudad <'a> (v_personas: &'a Vec<Persona<'a>>, ciudad: &str) -> bool {
        v_personas.iter().any(|x| x.ciudad == ciudad)
    }

    fn existe <'a> (v_personas: &'a Vec<Persona<'a>>, pe: Persona) -> bool {
        v_personas.contains(&pe)
        // .iter().any(|x| x.eq(&pe))
    }

    fn edades (v_personas: &Vec<Persona>) -> Vec<u8> {
        v_personas.iter().map(|x| x.edad).collect()
    }

    fn mayor_y_menor_salario <'a> (v_personas: &'a Vec<Persona<'a>>) -> (Persona , Persona) {
        let mut menor = v_personas[0].clone();
        let mut mayor = v_personas[0].clone();

        v_personas.iter().for_each(|persona| {
            if persona.salario < menor.salario || (persona.salario == menor.salario && persona.edad > menor.edad){
                menor = persona.clone();
            } 
            
            if persona.salario > mayor.salario || (persona.salario == mayor.salario && persona.edad > mayor.edad){
                mayor = persona.clone();
            }
        });

        (menor, mayor)
    }



    #[test]
fn test_listado_salario_mayor() {
    let p1 = Persona {
        nombre: "Juan",
        apellido: "Perez",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 1000.0,
        edad: 30,
    };
    let p2 = Persona {
        nombre: "Homero",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 2000.0,
        edad: 40,
    };
    let p3 = Persona {
        nombre: "Ned",
        apellido: "Flanders",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 50,
    };
    let vec = vec![p1.clone(), p2.clone(), p3.clone()];
    assert_eq!(es_mayor(&vec, 1500.0), vec![&p2, &p3]);
}

#[test]
fn test_persona_en_ciudad() {
    let p1 = Persona {
        nombre: "Juan",
        apellido: "Perez",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 1000.0,
        edad: 30,
    };
    let p2 = Persona {
        nombre: "Homero",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 2000.0,
        edad: 40,
    };
    let p3 = Persona {
        nombre: "Ned",
        apellido: "Flanders",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 50,
    };
    let vec = vec![p1.clone(), p2.clone(), p3.clone()];
    assert_eq!(mayores_edad(&vec, 35, "Springfield"), vec![&p2, &p3]);
}

#[test]
fn test_todas_en_ciudad() {
    let p1 = Persona {
        nombre: "Juan",
        apellido: "Perez",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 1000.0,
        edad: 30,
    };
    let p2 = Persona {
        nombre: "Homero",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 2000.0,
        edad: 40,
    };
    let p3 = Persona {
        nombre: "Ned",
        apellido: "Flanders",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 50,
    };
    let vec = vec![p1.clone(), p2.clone(), p3.clone()];
    assert_eq!(viven_en_la_ciudad(&vec, "Springfield"), true);
}

#[test]
fn test_almenos_uno_en_ciudad() {
    let p1 = Persona {
        nombre: "Juan",
        apellido: "Perez",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 1000.0,
        edad: 30,
    };
    let p2 = Persona {
        nombre: "Homero",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 2000.0,
        edad: 40,
    };
    let p3 = Persona {
        nombre: "Ned",
        apellido: "Flanders",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 50,
    };
    let vec = vec![p1.clone(), p2.clone(), p3.clone()];
    assert_eq!(alguno_vive_en_la_ciudad(&vec, "Springfield"), true);
}
#[test]
fn test_existe() {
    let p1 = Persona {
        nombre: "Juan",
        apellido: "Perez",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 1000.0,
        edad: 30,
    };
    let p2 = Persona {
        nombre: "Homero",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 2000.0,
        edad: 40,
    };
    let p3 = Persona {
        nombre: "Ned",
        apellido: "Flanders",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 50,
    };
    let vec = vec![p1.clone(), p2.clone(), p3.clone()];
    assert_eq!(existe(&vec, p1), true);
}

#[test]
fn test_edades() {
    let p1 = Persona {
        nombre: "Juan",
        apellido: "Perez",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 1000.0,
        edad: 30,
    };
    let p2 = Persona {
        nombre: "Homero",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 2000.0,
        edad: 40,
    };
    let p3 = Persona {
        nombre: "Ned",
        apellido: "Flanders",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 50,
    };
    let vec = vec![p1.clone(), p2.clone(), p3.clone()];
    assert_eq!(edades(&vec), vec![30, 40, 50]);
}

#[test]
fn test_retornar_max_y_min_salario() {
    let p1 = Persona {
        nombre: "Juan",
        apellido: "Perez",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 2000.0,
        edad: 30,
    };
    let p2 = Persona {
        nombre: "Homero",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 1000.0,
        edad: 40,
    };
    let p3 = Persona {
        nombre: "Ned",
        apellido: "Flanders",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 50,
    };
    let p4 = Persona {
        nombre: "Marge",
        apellido: "Simpson",
        direccion: "Av. Siempre Viva 123",
        ciudad: "Springfield",
        salario: 4000.0,
        edad: 60,
    };
    let vec = vec![p1.clone(), p2.clone(), p3, p4.clone()];
    assert_eq!(mayor_y_menor_salario(&vec), (p2, p4));
}





