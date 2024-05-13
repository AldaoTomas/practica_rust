use std::collections::VecDeque;

#[derive(Clone)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero
}

#[derive(Clone)]
enum Genero{
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros
}

struct Playlist {
    nombre: String,
    canciones: VecDeque<Cancion>
}
impl Genero {
    fn genero_to_int (&self) -> u8{
        match self{
            Self::Rock => 0,
            Self::Pop => 1,
            Self::Rap => 2,
            Self::Jazz => 3,
            Self::Otros => 4,
        }
    }

    pub fn equals(&self, genero: &Genero) -> bool {
        self.genero_to_int() == genero.genero_to_int()
    }
}

impl Cancion {
    pub fn equals (&self, cancion: &Cancion) -> bool {
        self.titulo == cancion.titulo && self.artista == cancion.artista && self.genero.equals(&cancion.genero)
    }
}



impl Playlist {

    fn agregar_cancion (&mut self, cancion: Cancion) {
        self.canciones.push_back(cancion);
    }


    fn eliminar_cancion_por_nombre (&mut self, cancion: Cancion) {
        for i in 0.. self.canciones.len(){
            if self.canciones[i].equals(&cancion){
                self.canciones.remove(i);
                break;
            }
        }
    }


    fn mover_cancion (&mut self, cancion: Cancion, pos: usize) {
        for i in 0..self.canciones.len(){
            if self.canciones[i].equals(&cancion) {
                if let Some(aux) = self.canciones.get(i).cloned(){
                    self.canciones.remove(i);
                    self.canciones.insert(pos, aux);
                }
            }
        }
    }


    fn buscar (&self, cancion: Cancion) -> Option<&Cancion> {
        for i in 0..self.canciones.len(){
            if self.canciones[i].equals(&cancion){
                return Some(&self.canciones[i]);
            }
        }
        None
    }


    fn obtener_cancion_genero (&self, genero: Genero) -> VecDeque<Cancion>{
        let mut aux: VecDeque<Cancion> = VecDeque::new();
        for i in 0..self.canciones.len(){
            if self.canciones[i].genero.equals(&genero) {
                aux.push_back(self.canciones[i].clone());
            }
        }
        return aux;
    }   


    fn obtener_cancion_artista (&self, artista: String) -> VecDeque<Cancion> {
        let mut a: VecDeque<Cancion> = VecDeque::new();
        for i in 0..self.canciones.len(){
            if self.canciones[i].artista == artista {
                a.push_back(self.canciones[i].clone());
            }
        }
        return a;
    }


    fn modificar_titulo (&mut self, titulo: String) {
        self.nombre = titulo
    }


    fn eliminar_todas_las_canciones (&mut self) {
        self.canciones.clear()
    }


}


#[test]
fn test_eliminar_todas_las_canciones() {
    let mut playlist = Playlist {
        canciones: VecDeque::new(),
        nombre: "Playlist".to_string(),
    };
    let cancion = Cancion {
        titulo: "Cancion".to_string(),
        artista: "Artista".to_string(),
        genero: Genero::Rock,
    };
    playlist.agregar_cancion(cancion.clone());
    playlist.eliminar_todas_las_canciones();
    assert_eq!(playlist.canciones.len(), 0);
}

#[test]
fn test_modificar_titulo() {
    let mut playlist = Playlist {
        canciones: VecDeque::new(),
        nombre: "Playlist".to_string(),
    };
    playlist.modificar_titulo("Nuevo".to_string());
    assert_eq!(playlist.nombre, "Nuevo".to_string());
}


#[test]
fn test_obtener_cancion_artista() {
    let mut playlist = Playlist {
        canciones: VecDeque::new(),
        nombre: "Playlist".to_string(),
    };
    let cancion = Cancion {
        titulo: "Cancion".to_string(),
        artista: "Artista".to_string(),
        genero: Genero::Rock,
    };
    let cancion2 = Cancion {
        titulo: "Cancion2".to_string(),
        artista: "Artista2".to_string(),
        genero: Genero::Pop,
    };

    playlist.agregar_cancion(cancion.clone());
    playlist.agregar_cancion(cancion2.clone());
    let canciones = playlist.obtener_cancion_artista("Artista".to_string());
    assert_eq!(canciones[0].equals(&cancion), true);
}


#[test]
fn test_obtener_cancion_genero() {
    let mut playlist = Playlist {
        canciones: VecDeque::new(),
        nombre: "Playlist".to_string(),
    };
    let cancion = Cancion {
        titulo: "Cancion".to_string(),
        artista: "Artista".to_string(),
        genero: Genero::Rock,
    };
    let cancion2 = Cancion {
        titulo: "Cancion2".to_string(),
        artista: "Artista2".to_string(),
        genero: Genero::Pop,
    };

    playlist.agregar_cancion(cancion.clone());
    playlist.agregar_cancion(cancion2.clone());
    let canciones = playlist.obtener_cancion_genero(Genero::Rock);
    assert_eq!(canciones[0].equals(&cancion), true);
}


#[test]
fn test_buscar() {
    let mut playlist = Playlist {
        canciones: VecDeque::new(),
        nombre: "Playlist".to_string(),
    };
    let cancion = Cancion {
        titulo: "Cancion".to_string(),
        artista: "Artista".to_string(),
        genero: Genero::Rock,
    };
    let cancion2 = Cancion {
        titulo: "Cancion2".to_string(),
        artista: "Artista2".to_string(),
        genero: Genero::Rock,
    };

    playlist.agregar_cancion(cancion.clone());
    let encontre = playlist.buscar(cancion.clone()).is_some();
    let no_encontre = playlist.buscar(cancion2.clone()).is_none();
    assert_eq!(encontre, true);
    assert_eq!(no_encontre, true);
}


#[test]
fn test_mover_cancion() {
    let mut playlist = Playlist {
        canciones: VecDeque::new(),
        nombre: "Playlist".to_string(),
    };
    let cancion = Cancion {
        titulo: "Cancion".to_string(),
        artista: "Artista".to_string(),
        genero: Genero::Rock,
    };
    let cancion2 = Cancion {
        titulo: "Cancion2".to_string(),
        artista: "Artista2".to_string(),
        genero: Genero::Rock,
    };
    playlist.agregar_cancion(cancion.clone());
    playlist.agregar_cancion(cancion2.clone());
    playlist.mover_cancion(cancion.clone(), 1);
    assert_eq!(playlist.canciones[1].equals(&cancion), true);
}


#[test]
fn test_eliminar_cancion_por_nombre() {
    let mut playlist = Playlist {
        canciones: VecDeque::new(),
        nombre: "Playlist".to_string(),
    };
    let cancion = Cancion {
        titulo: "Cancion".to_string(),
        artista: "Artista".to_string(),
        genero: Genero::Rock,
    };
    playlist.agregar_cancion(cancion.clone());
    playlist.eliminar_cancion_por_nombre(cancion);
    assert_eq!(playlist.canciones.len(), 0);
}


#[test]
fn test_agregar_cancion() {
    let mut playlist = Playlist {
        canciones: VecDeque::new(),
        nombre: "Playlist".to_string(),
    };
    let cancion = Cancion {
        titulo: "Cancion".to_string(),
        artista: "Artista".to_string(),
        genero: Genero::Rock,
    };
    playlist.agregar_cancion(cancion.clone());
    assert_eq!(playlist.canciones[0].equals(&cancion), true);
}