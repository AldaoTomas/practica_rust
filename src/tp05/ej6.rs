/*
En base al ejercicio 5 del tp#4 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b- Todos los balances de los usuarios así como las transacciones deben persistir en
archivos en formato JSON. No debe modificar los tests hechos en el punto a. Si
puede agregar más en caso de que haga métodos nuevos para cumplir con este
punto . Recuerde también que se debe seguir manteniendo un coverage de al
menos 90%.
*/

use core::hash;
use rand::Rng;
use std::collections::{BTreeMap, HashMap};
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


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Usuario {
    nombre: String,
    apellido: String,
    email: String,
    dni: String,
    kyc: bool,
    balance_crypto: BTreeMap<String, f64>,
    balance_fiat: f64,
}
trait IniciarBalance {
    fn iniciar_balance() -> BTreeMap<String, f64>;
}

impl IniciarBalance for BTreeMap<String, f64> {
    fn iniciar_balance() -> BTreeMap<String, f64> {
        let mut balance = BTreeMap::new();
        balance.insert("BTC".to_string(), 0.0);
        balance.insert("ETH".to_string(), 0.0);
        balance.insert("USDT".to_string(), 0.0);
        balance
    }
}

impl Usuario {
    fn new(nombre: String, apellido: String, email: String, dni: String) -> Self {
        Usuario {
            nombre,
            apellido,
            email,
            dni,
            kyc: false,
            balance_crypto: BTreeMap::iniciar_balance(),
            balance_fiat: 0.0,
        }
    }
    fn kyc(&mut self) {
        self.kyc = true;
    }
    fn validar_usuario_compra(self, fiat: f64) -> bool {
        if self.balance_fiat >= fiat && self.kyc {
            true
        } else {
            false
        }
    }
    fn validar_usuario_venta(self, monto: f64, cripto: Criptomoneda) -> bool {
        if self.balance_crypto.contains_key(&cripto.prefijo) && self.kyc {
            if self.balance_crypto.get(&cripto.prefijo).unwrap() >= &monto {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn validar_usuario_retiro_fiat(self, monto: f64) -> bool {
        if self.balance_fiat >= monto && self.kyc {
            true
        } else {
            false
        }
    }
    fn ingresar_dinero(&mut self, ingreso: f64) {
        self.balance_fiat += ingreso;
    }
    
    fn aumentar_balance_crypto(&mut self, cripto: Criptomoneda, monto: f64) {
        self.balance_crypto
            .entry(cripto.prefijo.clone())
            .and_modify(|c| *c += monto)
            .or_insert(monto);
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Criptomoneda {
    nombre: String,
    prefijo: String,
    listado_blockchain: Vec<Blockchain>,
}
impl Criptomoneda {
    fn new(nombre: String, prefijo: String) -> Self {
        Criptomoneda {
            nombre,
            prefijo,
            listado_blockchain: Vec::new(),
        }
    }

    fn cotizacion(&self) -> f64 {
        match self.prefijo.as_str() {
            "BTC" => 70000.0,
            "ETH" => 4000.0,
            "USDT" => 1.0,
            _ => 0.0,
        }
    }

    fn verificar_blockchain(&self, blockchain: Blockchain) -> bool {
        self.listado_blockchain.contains(&blockchain)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Blockchain {
    nombre: String,
    prefijo: String,
}
impl Blockchain {
    fn new(nombre: String, prefijo: String) -> Self {
        Blockchain { nombre, prefijo }
    }

    fn generar_hash(&self) -> Hash {
        let hash = Hash::new(self.nombre.clone());
        return hash;
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Hash {
    nombre_blockchain: String,
    hash: i32,
}
impl Hash {
    fn new(nombre_blockchain: String) -> Self {
        let mut rng = rand::thread_rng();
        Hash {
            nombre_blockchain,
            hash: rng.gen_range(0..1000),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Transaccion {
    usuario: Usuario,
    cripto: Criptomoneda,
    cotizacion: f64,
    blockchain: Blockchain,
    hash: Hash,
    monto: f64,
    tipo: String,
    fecha: String,
}
impl Transaccion {
    fn transaccion_ingreso(fecha: String, tipo: String, monto: f64, user: Usuario) -> Self {
        Transaccion {
            usuario: user,
            cripto: Criptomoneda::new("".to_string(), "".to_string()),
            cotizacion: 0.0,
            blockchain: Blockchain {
                nombre: "".to_string(),
                prefijo: "".to_string(),
            },
            hash: Hash {
                nombre_blockchain: "".to_string(),
                hash: 0,
            },
            monto: monto,
            tipo: tipo,
            fecha: fecha,
        }
    }

    fn transaccion_compra(
        fecha: String,
        user: Usuario,
        cripto: Criptomoneda,
        tipo: String,
        monto: f64,
        cotizacion: f64,
    ) -> Self {
        Transaccion {
            usuario: user,
            cripto: cripto,
            cotizacion: cotizacion,
            blockchain: Blockchain {
                nombre: "".to_string(),
                prefijo: "".to_string(),
            },
            hash: Hash {
                nombre_blockchain: "".to_string(),
                hash: 0,
            },
            monto: monto,
            tipo: tipo,
            fecha: fecha,
        }
    }

    fn transaccion_venta(
        fecha: String,
        user: Usuario,
        cripto: Criptomoneda,
        tipo: String,
        monto: f64,
        cotizacion: f64,
    ) -> Self {
        Transaccion {
            usuario: user,
            cripto: cripto,
            cotizacion: cotizacion,
            blockchain: Blockchain {
                nombre: "".to_string(),
                prefijo: "".to_string(),
            },
            hash: Hash {
                nombre_blockchain: "".to_string(),
                hash: 0,
            },
            monto: monto,
            tipo: tipo,
            fecha: fecha,
        }
    }

    fn transaccion_retiro(
        fecha: String,
        user: Usuario,
        tipo: String,
        blockchain: Blockchain,
        hash: Hash,
        monto: f64,
        cotizacion: f64,
    ) -> Self {
        Transaccion {
            usuario: user,
            cripto: Criptomoneda::new("".to_string(), "".to_string()),
            cotizacion: cotizacion,
            blockchain: blockchain,
            hash: hash,
            monto: monto,
            tipo: tipo,
            fecha: fecha,
        }
    }

    fn transaccion_recibir(
        fecha: String,
        user: Usuario,
        tipo: String,
        blockchain: Blockchain,
        cripto: Criptomoneda,
        monto: f64,
        cotizacion: f64,
    ) -> Self {
        Transaccion {
            usuario: user,
            cripto: cripto,
            cotizacion: cotizacion,
            blockchain: blockchain,
            hash: Hash {
                nombre_blockchain: "".to_string(),
                hash: 0,
            },
            monto: monto,
            tipo: tipo,
            fecha: fecha,
        }
    }

    fn transaccion_retirar_fiat(
        fecha: String,
        user: Usuario,
        tipo: String,
        monto: f64,
        medio: MedioPago,
    ) -> Self {
        Transaccion {
            usuario: user,
            cripto: Criptomoneda::new("".to_string(), "".to_string()),
            cotizacion: 0.0,
            blockchain: Blockchain {
                nombre: "".to_string(),
                prefijo: "".to_string(),
            },
            hash: Hash {
                nombre_blockchain: "".to_string(),
                hash: 0,
            },
            monto: monto,
            tipo: tipo,
            fecha: fecha,
        }
    }
}
enum MedioPago {
    MercadoPago,
    TransfenciaBancaria,
}
struct XYZ {
    usuarios: BTreeMap<String, Usuario>,
    transacciones: Vec<Transaccion>,
}

impl XYZ {
    fn new() -> Self {
        let usuarios: BTreeMap<String, Usuario> = BTreeMap::new();
        
        // Serializar el vector realizadas a JSON
        let json_realizadas = serde_json::to_string(&usuarios).unwrap();
        
        // Crear y escribir en el archivo JSON
        let mut file = File::create("src/tp05/usuarios.json").unwrap();
        file.write_all(json_realizadas.as_bytes()).unwrap();

        let transacciones: Vec<Transaccion> = Vec::new();
        
        // Serializar el vector realizadas a JSON
        let json_realizadas = serde_json::to_string(&transacciones).unwrap();
        
        // Crear y escribir en el archivo JSON
        let mut file = File::create("src/tp05/transacciones.json").unwrap();
        file.write_all(json_realizadas.as_bytes()).unwrap();

        XYZ {
            usuarios: BTreeMap::new(),
            transacciones: Vec::new(),
        }
    }
    fn ingresar_dinero(&mut self, user: Usuario, ingreso: f64) {
        self.usuarios.get_mut(&user.dni).unwrap().balance_fiat += ingreso;
        let transaccion = Transaccion::transaccion_ingreso(
            "fecha".to_string(),
            "ingreso".to_string(),
            ingreso,
            user,
        );
        self.transacciones.push(transaccion);
        self.guardar_transacciones().unwrap();
        self.guardar_usuarios().unwrap();
    }

    fn comprar_cripto(&mut self, user: Usuario, fiat: f64, cripto: Criptomoneda) -> Result<(), MiError>{
        if user.clone().validar_usuario_compra(fiat) {
            let cant_crypto = fiat / cripto.cotizacion();
            let usuario = self.usuarios.entry(user.dni.clone());

            usuario
                .and_modify(|u| u.balance_fiat -= fiat)
                .and_modify(|u| {
                    u.balance_crypto
                        .entry(cripto.prefijo.clone())
                        .and_modify(|c| *c += cant_crypto);
                });
            let transaccion = Transaccion::transaccion_compra(
                "fecha".to_string(),
                user,
                cripto.clone(),
                "compra cripto".to_string(),
                cant_crypto,
                cripto.cotizacion(),
            );
            self.transacciones.push(transaccion);
            self.guardar_transacciones().unwrap();
            self.guardar_usuarios().unwrap();
            Ok(())
        }
        else {
            Err(MiError { msg: "Usuario no valido".to_string() })
        }
    }

    fn vender_cripto(&mut self, user: Usuario, monto: f64, cripto: Criptomoneda) -> Result<(), MiError>{
        if user.clone().validar_usuario_venta(monto, cripto.clone()) {
            let cant_fiat = monto * cripto.cotizacion();
            let usuario = self.usuarios.entry(user.dni.clone());
            usuario
                .and_modify(|u| {
                    u.balance_crypto
                        .entry(cripto.prefijo.clone())
                        .and_modify(|c| *c -= monto);
                })
                .and_modify(|u| u.balance_fiat += cant_fiat);

            let transaccion = Transaccion::transaccion_venta(
                "fecha".to_string(),
                user,
                cripto.clone(),
                "venta cripto".to_string(),
                monto,
                cripto.cotizacion(),
            );
            self.transacciones.push(transaccion);
            self.guardar_transacciones().unwrap();
            self.guardar_usuarios().unwrap();
            Ok(())
        }
        else {
            Err(MiError { msg: "Usuario no valido".to_string() })
        }
    }

    fn retirar_cripto(&mut self, user: Usuario, monto: f64, cripto: Criptomoneda, blockchain: Blockchain,) -> Result<(), MiError> {
        if user.clone().validar_usuario_venta(monto, cripto.clone())
            && cripto.verificar_blockchain(blockchain.clone())
        {
            let usuario = self.usuarios.entry(user.dni.clone());
            usuario.and_modify(|u| {
                u.balance_crypto
                    .entry(cripto.prefijo.clone())
                    .and_modify(|c| *c -= monto);
            });

            let hash = blockchain.generar_hash();
            let transaccion = Transaccion::transaccion_retiro(
                "fecha".to_string(),
                user,
                "retiro".to_string(),
                blockchain,
                hash,
                monto,
                cripto.cotizacion(),
            );
            self.transacciones.push(transaccion);
            self.guardar_transacciones().unwrap();
            self.guardar_usuarios().unwrap();
            Ok(())
        }
        else {
            Err(MiError { msg: "Usuario no valido o blockchain no verificada".to_string() })
        }
    }

    fn recibir_cripto(&mut self, user: Usuario, monto: f64, cripto: Criptomoneda, blockchain: Blockchain, ) -> Result<(), MiError> {
        if cripto.verificar_blockchain(blockchain.clone()) {
            let usuario = self.usuarios.entry(user.dni.clone());
            usuario.and_modify(|u| {
                u.balance_crypto
                    .entry(cripto.prefijo.clone())
                    .and_modify(|c| *c += monto);
            });
            let cotizacion = cripto.cotizacion();
            let transaccion = Transaccion::transaccion_recibir(
                "".to_string(),
                user,
                "".to_string(),
                blockchain,
                cripto,
                monto,
                cotizacion,
            );
            self.transacciones.push(transaccion);
            self.guardar_transacciones().unwrap();
            self.guardar_usuarios().unwrap();
            Ok(())
        }
        else {
            Err(MiError { msg: "Blockchain no verificada".to_string() })
        }

    }

    fn retirar_fiat(&mut self, monto: f64, user: Usuario) {
        if user.clone().validar_usuario_retiro_fiat(monto) {
            let usuario = self.usuarios.entry(user.dni.clone());
            usuario.and_modify(|u| u.balance_fiat -= monto);

            let transaccion = Transaccion::transaccion_retirar_fiat(
                "".to_string(),
                user,
                "".to_string(),
                monto,
                MedioPago::MercadoPago,
            );
            self.transacciones.push(transaccion);
            self.guardar_transacciones().unwrap();
            self.guardar_usuarios().unwrap();
        }
    }

    fn cripto_mas_vendida(&self) -> String {
        let mut contador_cripto: BTreeMap<String, u32> = BTreeMap::new();

        for venta in self.transacciones.clone() {
            if venta.tipo == "venta cripto" {
                let cant = contador_cripto
                    .entry(venta.cripto.prefijo.clone())
                    .or_insert(0);
                *cant += 1;
            }
        }

        let mut max_cripto = None;
        let mut max_value = 0;

        for (cripto, &value) in &contador_cripto {
            if value > max_value {
                max_value = value;
                max_cripto = Some(cripto);
            }
        }

        if let Some(cripto) = max_cripto {
            return cripto.clone();
        } else {
            return "".to_string();
        }
    }

    fn cripto_mas_comprada(&self) -> String {
        let mut contador_cripto: BTreeMap<String, u32> = BTreeMap::new();

        for compra in self.transacciones.clone() {
            if compra.tipo == "compra cripto" {
                let cant = contador_cripto
                    .entry(compra.cripto.prefijo.clone())
                    .or_insert(0);
                *cant += 1;
            }
        }

        let mut max_cripto = None;
        let mut max_value = 0;

        for (cripto, &value) in &contador_cripto {
            if value > max_value {
                max_value = value;
                max_cripto = Some(cripto);
            }
        }

        if let Some(cripto) = max_cripto {
            return cripto.clone();
        } else {
            return "".to_string();
        }
    }

    fn cripto_mas_volumen_venta(&self) -> String {
        let mut contador_cripto: BTreeMap<String, f64> = BTreeMap::new();
        for venta in self.transacciones.clone() {
            if venta.tipo == "venta cripto" {
                let cant = contador_cripto
                    .entry(venta.cripto.prefijo.clone())
                    .or_insert(0.0);
                *cant += venta.monto;
            }
        }
        let mut max_cripto = None;
        let mut max_value = 0.0;
        for (cripto, &value) in &contador_cripto {
            if value > max_value {
                max_value = value;
                max_cripto = Some(cripto);
            }
        }
        if let Some(cripto) = max_cripto {
            return cripto.clone();
        } else {
            return "".to_string();
        }
    }
    fn cripto_mas_volumen_compra(&self) -> String {
        let mut contador_cripto: BTreeMap<String, f64> = BTreeMap::new();
        for compra in self.transacciones.clone() {
            if compra.tipo == "compra cripto" {
                let cant = contador_cripto
                    .entry(compra.cripto.prefijo.clone())
                    .or_insert(0.0);
                *cant += compra.monto;
            }
        }
        let mut max_cripto = None;
        let mut max_value = 0.0;
        for (cripto, &value) in &contador_cripto {
            if value > max_value {
                max_value = value;
                max_cripto = Some(cripto);
            }
        }
        if let Some(cripto) = max_cripto {
            return cripto.clone();
        } else {
            return "".to_string();
        }
    }

    fn pushear_transaccion(&mut self, t: Transaccion) {
        self.transacciones.push(t);
    }

    pub fn guardar_usuarios(&self) -> Result<(), MiError> {
        let json = serde_json::to_string(&self.usuarios)?;
        let mut file = OpenOptions::new().write(true).open("src/tp05/usuarios.json")?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn guardar_transacciones(&self) -> Result<(), MiError> {
        let json = serde_json::to_string(&self.transacciones)?;
        let mut file = OpenOptions::new().write(true).open("src/tp05/transacciones.json")?;
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

#[test]
fn test_cripto_mas_volumen_compra() {
    let mut usuario = Usuario::new(
        "Tomas".to_string(),
        "Aldao".to_string(),
        " ".to_string(),
        "11111111".to_string(),
    );
    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    let cripto2 = Criptomoneda::new("Ethereum".to_string(), "ETH".to_string());
    usuario.aumentar_balance_crypto(cripto.clone(), 100.0);
    usuario.aumentar_balance_crypto(cripto2.clone(), 100.0);
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto.clone());

    assert_eq!(plataforma.cripto_mas_volumen_compra(), "ETH".to_string());
}

#[test]
fn test_cripto_mas_volumen_venta() {
    let mut usuario = Usuario::new(
        "Tomas".to_string(),
        "Aldao".to_string(),
        " ".to_string(),
        "11111111".to_string(),
    );
    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    let cripto2 = Criptomoneda::new("Ethereum".to_string(), "ETH".to_string());
    usuario.aumentar_balance_crypto(cripto.clone(), 100.0);
    usuario.aumentar_balance_crypto(cripto2.clone(), 100.0);
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto.clone());

    assert_eq!(plataforma.cripto_mas_volumen_venta(), "BTC".to_string());
}

#[test]
fn test_cripto_mas_comprada() {
    let mut usuario = Usuario::new(
        "Tomas".to_string(),
        "Aldao".to_string(),
        " ".to_string(),
        "11111111".to_string(),
    );
    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    let cripto2 = Criptomoneda::new("USDT".to_string(), "USDT".to_string());
    usuario.aumentar_balance_crypto(cripto.clone(), 100.0);
    usuario.aumentar_balance_crypto(cripto2.clone(), 100.0);
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.comprar_cripto(usuario.clone(), 50.0, cripto.clone());

    assert_eq!(plataforma.cripto_mas_comprada(), "BTC".to_string());
}

#[test]
fn test_cripto_mas_vendida() {
    let mut usuario = Usuario::new(
        "Tomas".to_string(),
        "Aldao".to_string(),
        " ".to_string(),
        "11111111".to_string(),
    );
    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    let cripto2 = Criptomoneda::new("USDT".to_string(), "USDT".to_string());
    usuario.aumentar_balance_crypto(cripto.clone(), 100.0);
    usuario.aumentar_balance_crypto(cripto2.clone(), 100.0);
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto2.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto.clone());

    assert_eq!(plataforma.cripto_mas_vendida(), "BTC".to_string());
}

#[test]
fn test_retirar_fiat() {
    let mut usuario = Usuario::new(
        "Tomas".to_string(),
        "Aldao".to_string(),
        " ".to_string(),
        "11111111".to_string(),
    );
    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.retirar_fiat(100.0, usuario.clone());

    let user = plataforma.usuarios.get(&usuario.dni).unwrap();
    assert_eq!(user.balance_fiat, 900.0);
    assert_eq!(plataforma.transacciones.len(), 1);
}

#[test]
fn test_recibir_cripto() {
    let mut usuario = Usuario::new(
        "Tomas".to_string(),
        "Aldao".to_string(),
        " ".to_string(),
        "11111111".to_string(),
    );
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let mut cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    let blockchain = Blockchain::new("Blockchain1".to_string(), "BC1".to_string());
    cripto.listado_blockchain.push(blockchain.clone());
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());

    let mut user = plataforma.usuarios.get(&usuario.dni).unwrap();
    let mut balance_cripto = user.balance_crypto.get(&cripto.prefijo).unwrap();

    assert_eq!(*balance_cripto, 0.0);
    plataforma.recibir_cripto(usuario.clone(), 50.0, cripto.clone(), blockchain.clone());

    user = plataforma.usuarios.get(&usuario.dni).unwrap();
    balance_cripto = user.balance_crypto.get(&cripto.prefijo).unwrap();

    assert_eq!(*balance_cripto, 50.0);
    assert_eq!(plataforma.transacciones.len(), 1);
}

#[test]
fn test_retirar_cripto() {
    let mut usuario = Usuario::new(
        "Tomas".to_string(),
        "Aldao".to_string(),
        " ".to_string(),
        "11111111".to_string(),
    );
    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let mut cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    let blockchain = Blockchain::new("Blockchain1".to_string(), "BC1".to_string());
    usuario.aumentar_balance_crypto(cripto.clone(), 100.0);
    cripto.listado_blockchain.push(blockchain.clone());
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.retirar_cripto(usuario.clone(), 50.0, cripto.clone(), blockchain.clone());

    let user = plataforma.usuarios.get(&usuario.dni).unwrap();
    let balance_cripto = user.balance_crypto.get(&cripto.prefijo).unwrap();

    assert_eq!(user.balance_fiat, 1000.0);
    assert_eq!(*balance_cripto, 50.0);
    assert_eq!(plataforma.transacciones.len(), 1);
}

#[test]
fn test_vender_cripto() {
    let mut usuario = Usuario::new(
        "Tomas".to_string(),
        "Aldao".to_string(),
        " ".to_string(),
        "11111111".to_string(),
    );
    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    usuario.aumentar_balance_crypto(cripto.clone(), 100.0);
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.vender_cripto(usuario.clone(), 50.0, cripto.clone());

    let user = plataforma.usuarios.get(&usuario.dni).unwrap();
    let balance_cripto = user.balance_crypto.get(&cripto.prefijo).unwrap();

    assert_eq!(user.balance_fiat, 1000.0 + 50.0 * cripto.cotizacion());
    assert_eq!(*balance_cripto, 50.0);
    assert_eq!(plataforma.transacciones.len(), 1);
}

#[test]
fn test_comprar_cripto() {
    let mut usuario = Usuario::new(
        "Tomas".to_string(),
        "Aldao".to_string(),
        " ".to_string(),
        "11111111".to_string(),
    );

    usuario.balance_fiat = 1000.0;
    usuario.kyc();
    let mut plataforma = XYZ::new();
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.comprar_cripto(usuario.clone(), 100.0, cripto.clone());
    let user = plataforma.usuarios.get(&usuario.dni).unwrap();
    let balance_cripto = user.balance_crypto.get(&cripto.prefijo).unwrap();
    assert_eq!(user.balance_fiat, 900.0);
    assert_eq!(*balance_cripto, 100.0 / cripto.cotizacion());
    assert_eq!(plataforma.transacciones.len(), 1);
}

#[test]
fn test_ingresar_dinero() {
    let usuario = Usuario::new(
        "Tomas".to_string(),
        "Aldao".to_string(),
        " ".to_string(),
        "11111111".to_string(),
    );
    let mut plataforma = XYZ::new();
    plataforma
        .usuarios
        .insert(usuario.dni.clone(), usuario.clone());
    plataforma.ingresar_dinero(usuario.clone(), 100.0);
    assert_eq!(
        plataforma.usuarios.get(&usuario.dni).unwrap().balance_fiat,
        100.0
    );
}

#[test]
fn test_generar_hash() {
    let blockchain = Blockchain::new("Tron".to_string(), "TRC20".to_string());
    let hash = blockchain.generar_hash();
    assert_eq!(hash.nombre_blockchain, "Tron".to_string());
    assert!(hash.hash >= 0 && hash.hash < 1000);
}

#[test]
fn test_transacciones() {
    let mut usuario = Usuario::new(
        "Tomas".to_string(),
        "Aldao".to_string(),
        "tomasaldao@gmail.com".to_string(),
        "11111111".to_string(),
    );
    let mut plataforma = XYZ::new();
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    let blockchain = Blockchain::new("Bitcoin".to_string(), "BTC".to_string());

    let transaccion_ingreso = Transaccion::transaccion_ingreso(
        "fecha".to_string(),
        "ingreso".to_string(),
        100.0,
        usuario.clone(),
    );
    let transaccion_compra = Transaccion::transaccion_compra(
        "fecha".to_string(),
        usuario.clone(),
        cripto.clone(),
        "compra cripto".to_string(),
        100.0,
        cripto.cotizacion(),
    );

    let transaccion_venta = Transaccion::transaccion_venta(
        "fecha".to_string(),
        usuario.clone(),
        cripto.clone(),
        "venta cripto".to_string(),
        100.0,
        cripto.cotizacion(),
    );

    let transaccion_retiro = Transaccion::transaccion_retiro(
        "fecha".to_string(),
        usuario.clone(),
        "retiro".to_string(),
        blockchain.clone(),
        blockchain.generar_hash(),
        100.0,
        cripto.cotizacion(),
    );

    let transaccion_recibir = Transaccion::transaccion_recibir(
        "fecha".to_string(),
        usuario.clone(),
        "recibir".to_string(),
        blockchain.clone(),
        cripto.clone(),
        100.0,
        cripto.cotizacion(),
    );
    let transaccion_retirar_fiat = Transaccion::transaccion_retirar_fiat(
        "fecha".to_string(),
        usuario.clone(),
        "retirar".to_string(),
        100.0,
        MedioPago::MercadoPago,
    );

    plataforma.pushear_transaccion(transaccion_ingreso.clone());
    plataforma.pushear_transaccion(transaccion_compra.clone());
    plataforma.pushear_transaccion(transaccion_venta.clone());
    plataforma.pushear_transaccion(transaccion_retiro.clone());
    plataforma.pushear_transaccion(transaccion_recibir.clone());
    plataforma.pushear_transaccion(transaccion_retirar_fiat.clone());

    assert_eq!(plataforma.transacciones.len(), 6);
    assert_eq!(plataforma.transacciones[0], transaccion_ingreso);
    assert_eq!(plataforma.transacciones[1], transaccion_compra);
    assert_eq!(plataforma.transacciones[2], transaccion_venta);
    assert_eq!(plataforma.transacciones[3], transaccion_retiro);
    assert_eq!(plataforma.transacciones[4], transaccion_recibir);
    assert_eq!(plataforma.transacciones[5], transaccion_retirar_fiat);
}

#[test]
fn test_validar_usuario() {
    let mut usuario = Usuario::new(
        "Tomas".to_string(),
        "Aldao".to_string(),
        " ".to_string(),
        "11111111".to_string(),
    );
    assert_eq!(usuario.kyc, false);
    usuario.kyc();
    assert_eq!(usuario.kyc, true);

    usuario.ingresar_dinero(100.0);
    let cripto = Criptomoneda::new("Bitcoin".to_string(), "BTC".to_string());
    usuario.aumentar_balance_crypto(cripto.clone(), 500.0);

    assert_eq!(usuario.clone().validar_usuario_compra(100.0), true);
    assert_eq!(usuario.clone().validar_usuario_venta(100.0, cripto), true);
    assert_eq!(usuario.validar_usuario_retiro_fiat(100.0), true);
}
}