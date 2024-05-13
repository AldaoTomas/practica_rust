struct Persona{
    nombre: String,
    edad: u32,
    direccion: Option<String>,


}



impl Persona {
    // Constructor
    fn new(nombre: String, edad: u32, direccion: Option<String>) -> Self {
        Persona {
            nombre,
            edad,
            direccion,
        }
    }

    // Método para obtener la representación de la persona como String
    fn to_string(&self) -> String {
        let mut result = format!("Nombre: {}, Edad: {}", self.nombre, self.edad);
        if let Some(direccion) = &self.direccion {
            result.push_str(&format!(", Dirección: {}", direccion));
        }
        result
    }

    // Método para obtener la edad de la persona
    fn obtener_edad(&self) -> u32 {
        self.edad
    }

    // Método para actualizar la dirección de la persona
    fn actualizar_direccion(&mut self, nueva_direccion: Option<String>) {
        self.direccion = nueva_direccion;
    }
}


#[test]
    fn test_constructor_persona() {
        let persona = Persona::new(String::from("Juan"), 30, Some(String::from("Calle Principal")));
        assert_eq!(persona.nombre, "Juan");
        assert_eq!(persona.edad, 30);
        assert_eq!(persona.direccion, Some(String::from("Calle Principal")));
    }

    #[test]
    fn test_to_string() {
        let persona1 = Persona::new(String::from("Juan"), 30, Some(String::from("Calle Principal")));
        assert_eq!(persona1.to_string(), "Nombre: Juan, Edad: 30, Dirección: Calle Principal");

        let persona2 = Persona::new(String::from("María"), 25, None);
        assert_eq!(persona2.to_string(), "Nombre: María, Edad: 25");
    }

    #[test]
    fn test_obtener_edad() {
        let persona = Persona::new(String::from("Juan"), 30, Some(String::from("Calle Principal")));
        assert_eq!(persona.obtener_edad(), 30);
    }

    #[test]
    fn test_actualizar_direccion() {
        let mut persona = Persona::new(String::from("Juan"), 30, Some(String::from("Calle Principal")));
        assert_eq!(persona.direccion, Some(String::from("Calle Principal")));

        persona.actualizar_direccion(Some(String::from("Calle Secundaria")));
        assert_eq!(persona.direccion, Some(String::from("Calle Secundaria")));

        persona.actualizar_direccion(None);
        assert_eq!(persona.direccion, None);
    }