/*2- En base al ejercicio 8 del tp#3 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b- Una vez obtenido dicho coverage, las canciones de la playlist deben ser
guardadas en un archivo en formato JSON, por lo tanto las operaciones que agreguen,
quiten o modifiquen la playlist deben estar respaldadas sobre dicho archivo.
No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que
haga métodos nuevos. Recuerde también que se debe seguir manteniendo un coverage de
al menos 90%,
*/
use std::collections::VecDeque;

use serde::{Serialize, Deserialize};
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


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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

    fn agregar_cancion (&mut self, cancion: Cancion) -> Result<(), MiError> {
        self.canciones.push_back(cancion);
        self.guardar_canciones()?;
        Ok(())
    }


    fn eliminar_cancion_por_nombre (&mut self, cancion: Cancion) -> Result<(), MiError> {
        for i in 0.. self.canciones.len(){
            if self.canciones[i].equals(&cancion){
                self.canciones.remove(i);
                self.guardar_canciones()?;
                return Ok(());
            }
        }
        Err(MiError{msg: "No se encontro la cancion".to_string()})
    }


    fn mover_cancion (&mut self, cancion: Cancion, pos: usize) -> Result<(), MiError> {
        for i in 0..self.canciones.len(){
            if self.canciones[i].equals(&cancion) {
                if let Some(aux) = self.canciones.get(i).cloned(){
                    self.canciones.remove(i);
                    self.canciones.insert(pos, aux);
                }
            }
        }
        self.guardar_canciones()?;
        Ok(())
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


    fn modificar_titulo (&mut self, titulo: String) -> Result<(), MiError>{
        self.nombre = titulo;
        self.guardar_canciones()?;
        Ok(())
    }


    fn eliminar_todas_las_canciones (&mut self) -> Result<(), MiError>{
        self.canciones.clear();

        self.guardar_canciones()?;
        Ok(())
    }


    fn guardar_canciones(&self) -> Result<(), MiError> {
        let path = "src/tp05/playlist.json";
        let json = serde_json::to_string(&self.canciones)?;
        let mut archivo = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
        archivo.write_all(json.as_bytes())?;
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
    let cancion2 = Cancion {
        titulo: "Cancion2".to_string(),
        artista: "Artista2".to_string(),
        genero: Genero::Rock,
    };
    playlist.agregar_cancion(cancion.clone());
    playlist.eliminar_cancion_por_nombre(cancion);
    assert_eq!(playlist.canciones.len(), 0);

    let cancion3 = Cancion {
        titulo: "Cancion3".to_string(),
        artista: "Artista3".to_string(),
        genero: Genero::Rock,
    };

    assert!(playlist.eliminar_cancion_por_nombre(cancion3).is_err());
    
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