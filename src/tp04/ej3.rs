use std::collections::HashMap;
use super::fecha::Fecha;

struct TransmisionRust {
    usuarios: Vec<Usuario>,
    suscripciones: Vec<Suscripcion>,
}

struct Usuario {
    id: u32,
    nombre: String,
    metodo_pago: MetodoPago,
}

struct Suscripcion {
    tipo: TipoSuscripcion,
    estado: bool,
    duracion_meses: u32,
    fecha_inicio: Fecha,
    id_usuario: u32,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum TipoSuscripcion {
    Basica,
    Clasica,
    Premium,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum MetodoPago {
    Efectivo,
    MercadoPago { id_cuenta: String },
    TarjetaCredito { numero_tarjeta: String, expiracion: String },
    TransferenciaBancaria { cuenta_bancaria: String },
    Cripto { billetera: String },
}

impl TransmisionRust {
    fn new() -> Self {
        TransmisionRust {
            usuarios: Vec::new(),
            suscripciones: Vec::new(),
        }
    }

    fn crear_suscripcion(&mut self, id_usuario: u32, nombre: String, tipo_suscripcion: TipoSuscripcion, duracion_meses: u32, metodo_pago: MetodoPago) {
        if self.obtener_usuario(id_usuario).is_none() {
            self.crear_usuario(id_usuario, nombre.clone(), metodo_pago.clone());
        }

        if self.obtener_suscripcion(id_usuario).is_none() {
            let usuario = self.obtener_usuario(id_usuario).unwrap();
            let suscripcion = Suscripcion::new(tipo_suscripcion, duracion_meses, usuario.id);
            self.suscripciones.push(suscripcion);
        }
    }

    fn crear_usuario(&mut self, id_usuario: u32, nombre: String, metodo_pago: MetodoPago) -> &Usuario {
        let usuario = Usuario::new(id_usuario, nombre, metodo_pago);
        self.usuarios.push(usuario);
        self.usuarios.last().unwrap()
    }

    fn obtener_usuario(&self, id_usuario: u32) -> Option<&Usuario> {
        self.usuarios.iter().find(|usuario| usuario.id == id_usuario)
    }

    fn metodo_pago_activo_mas_usado(&self) -> Option<MetodoPago> {
        let mut metodo_pagos = HashMap::new();

        self.suscripciones.iter().filter(|suscripcion| suscripcion.esta_activa()).map(|suscripcion| self.obtener_usuario(suscripcion.id_usuario)).filter(|usuario| usuario.is_some())
            .map(|usuario| usuario.unwrap()).for_each(|usuario| {
                *metodo_pagos.entry(usuario.metodo_pago.clone()).or_insert(0) += 1;
            });
        metodo_pagos.iter().max_by_key(|(_, count)| *count).map(|(tipo_suscripcion, _)| tipo_suscripcion.clone())
    }

    fn tipo_suscripcion_activa_mas_usada(&self) -> Option<TipoSuscripcion> {
        let mut suscripciones = HashMap::new();
        self.suscripciones.iter().filter(|suscripcion| suscripcion.esta_activa()).for_each(|suscripcion| {
                *suscripciones.entry(suscripcion.tipo.clone()).or_insert(0) += 1;
            });
        suscripciones.iter().max_by_key(|(_, count)| *count).map(|(tipo_suscripcion, _)| tipo_suscripcion.clone())
    }

    fn metodo_pago_mas_usado(&self) -> Option<MetodoPago> {
        let mut metodo_pagos = HashMap::new();
        for usuario in &self.usuarios {
            *metodo_pagos.entry(usuario.metodo_pago.clone()).or_insert(0) += 1;
        }
        metodo_pagos.iter().max_by_key(|(_, count)| *count).map(|(metodo_pago, _)| metodo_pago.clone())
    }

    fn tipo_suscripcion_mas_usado(&self) -> Option<TipoSuscripcion> {
        let mut suscripciones = HashMap::new();
        for suscripcion in &self.suscripciones {
            *suscripciones.entry(suscripcion.tipo.clone()).or_insert(0) += 1;
        }

        suscripciones.iter().max_by_key(|(_, count)| *count).map(|(tipo_suscripcion, _)| tipo_suscripcion.clone())
    }

    fn mejorar_suscripcion(&mut self, id_usuario: u32) {
        let suscripcion: Option<&mut Suscripcion> = self.obtener_suscripcion(id_usuario);
        if let Some(suscripcion) = suscripcion {
            suscripcion.mejorar();
        }
    }

    fn degradar_suscripcion(&mut self, id_usuario: u32) {
        let suscripcion = self.obtener_suscripcion(id_usuario);
        if let Some(suscripcion) = suscripcion {
            suscripcion.degradar();
        }
    }

    fn cancelar_suscripcion(&mut self, id_usuario: u32) {
        let suscripcion = self.obtener_suscripcion(id_usuario);
        if let Some(suscripcion) = suscripcion {
            suscripcion.cancelar();
        }
    }

    fn obtener_suscripcion(&mut self, id_usuario: u32) -> Option<&mut Suscripcion> {
        self.suscripciones.iter_mut().find(|suscripcion| suscripcion.id_usuario == id_usuario)
    }
}

impl Usuario {
    fn new(id: u32, nombre: String, metodo_pago: MetodoPago) -> Self {
        Usuario {
            id,
            nombre,
            metodo_pago,
        }
    }
}

impl Suscripcion {
    fn new(tipo_suscripcion: TipoSuscripcion, duracion_meses: u32, id_usuario: u32) -> Self {
        Suscripcion {
            estado: true,
            fecha_inicio: Fecha::new(1, 1, 2021),
            tipo: tipo_suscripcion,
            duracion_meses,
            id_usuario,
        }
    }

    fn mejorar(&mut self) {
        self.tipo = match self.tipo {
            TipoSuscripcion::Basica => TipoSuscripcion::Clasica,
            TipoSuscripcion::Clasica => TipoSuscripcion::Premium,
            TipoSuscripcion::Premium => TipoSuscripcion::Premium,
        };
    }

    fn degradar(&mut self) {
        self.tipo = match self.tipo {
            TipoSuscripcion::Premium => TipoSuscripcion::Clasica,
            TipoSuscripcion::Clasica => TipoSuscripcion::Basica,
            TipoSuscripcion::Basica => {
                self.cancelar();
                return;
            }
        };
    }

    fn cancelar(&mut self) {
        self.estado = false;
    }

    fn costo(&self) -> f64 {
        self.tipo.costo() * self.duracion_meses as f64
    }

    fn esta_activa(&self) -> bool {
        self.estado
    }
}

impl TipoSuscripcion {
    fn costo(&self) -> f64 {
        match self {
            TipoSuscripcion::Basica => 10.0,
            TipoSuscripcion::Clasica => 20.0,
            TipoSuscripcion::Premium => 30.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tipo_suscripcion_mas_usado() {
        let mut transmision = TransmisionRust::new();
        transmision.crear_suscripcion(1, "Tomas".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::Efectivo);
        transmision.crear_suscripcion(2, "Luffy".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::Efectivo);
        transmision.crear_suscripcion(3, "Messi".to_string(), TipoSuscripcion::Basica, 3, MetodoPago::Efectivo);
        transmision.crear_suscripcion(4, "Vegeta".to_string(), TipoSuscripcion::Basica, 3, MetodoPago::Efectivo);
        transmision.crear_suscripcion(5, "Kira".to_string(), TipoSuscripcion::Basica, 3, MetodoPago::Efectivo);
        transmision.cancelar_suscripcion(4);
        transmision.cancelar_suscripcion(5);
        let tipo_suscripcion = transmision.tipo_suscripcion_mas_usado().unwrap();
        assert_eq!(tipo_suscripcion, TipoSuscripcion::Basica);
    }

    #[test]
    fn test_metodo_pago_mas_usado() {
        let mut transmision = TransmisionRust::new();
        transmision.crear_suscripcion(1, "Tomas".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::Efectivo);
        transmision.crear_suscripcion(2, "Luffy".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::Efectivo);
        transmision.crear_suscripcion(3, "Messi".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::Efectivo);
        transmision.crear_suscripcion(4, "Vegeta".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::MercadoPago { id_cuenta: "777".to_string() });
        transmision.crear_suscripcion(5, "Kira".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::MercadoPago { id_cuenta: "123".to_string() });
        transmision.cancelar_suscripcion(1);
        transmision.cancelar_suscripcion(2);
        let metodo_pago = transmision.metodo_pago_mas_usado().unwrap();
        assert_eq!(metodo_pago, MetodoPago::Efectivo);
    }

    #[test]
    fn test_tipo_suscripcion_activa_mas_usada() {
        let mut transmision = TransmisionRust::new();
        transmision.crear_suscripcion(1, "Tomas".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::Efectivo);
        transmision.crear_suscripcion(2, "Luffy".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::Efectivo);
        transmision.crear_suscripcion(3, "Messi".to_string(), TipoSuscripcion::Basica, 3, MetodoPago::Efectivo);
        transmision.crear_suscripcion(4, "Vegeta".to_string(), TipoSuscripcion::Basica, 3, MetodoPago::Efectivo);
        transmision.crear_suscripcion(5, "Kira".to_string(), TipoSuscripcion::Basica, 3, MetodoPago::Efectivo);
        transmision.cancelar_suscripcion(4);
        transmision.cancelar_suscripcion(5);
        let tipo_suscripcion = transmision.tipo_suscripcion_activa_mas_usada().unwrap();
        assert_eq!(tipo_suscripcion, TipoSuscripcion::Premium);
    }

    #[test]
    fn test_metodo_pago_activo_mas_usado() {
        let mut transmision = TransmisionRust::new();
        transmision.crear_suscripcion(1, "Tomas".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::Efectivo);
        transmision.crear_suscripcion(2, "Luffy".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::Efectivo);
        transmision.crear_suscripcion(3, "Messi".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::MercadoPago { id_cuenta: "10".to_string() });
        transmision.crear_suscripcion(4, "Vegeta".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::MercadoPago { id_cuenta: "777".to_string() });
        transmision.crear_suscripcion(5, "Kira".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::MercadoPago { id_cuenta: "123".to_string() });
        transmision.cancelar_suscripcion(4);
        transmision.cancelar_suscripcion(5);
        let metodo_pago = transmision.metodo_pago_activo_mas_usado().unwrap();
        assert_eq!(metodo_pago, MetodoPago::Efectivo);
    }

    #[test]
    fn test_cancelar_suscripcion() {
        let mut transmision = TransmisionRust::new();
        transmision.crear_suscripcion(1, "Tomas".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::Efectivo);
        transmision.cancelar_suscripcion(1);
        let suscripcion = transmision.suscripciones.first().unwrap();
        assert_eq!(suscripcion.estado, false);
    }

    #[test]
    fn test_degradar_suscripcion_cancelar() {
        let mut transmision = TransmisionRust::new();
        transmision.crear_suscripcion(1, "Tomas".to_string(), TipoSuscripcion::Basica, 3, MetodoPago::Efectivo);
        transmision.degradar_suscripcion(1);
        let suscripcion = transmision.suscripciones.first().unwrap();
        assert_eq!(suscripcion.estado, false);
    }

    #[test]
    fn test_degradar_suscripcion() {
        let mut transmision = TransmisionRust::new();
        transmision.crear_suscripcion(1, "Tomas".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::Efectivo);
        transmision.degradar_suscripcion(1);
        let suscripcion = transmision.suscripciones.first().unwrap();
        assert_eq!(suscripcion.tipo, TipoSuscripcion::Clasica);
    }

    #[test]
    fn test_mejorar_suscripcion_premium() {
        let mut transmision = TransmisionRust::new();
        transmision.crear_suscripcion(1, "Tomas".to_string(), TipoSuscripcion::Premium, 3, MetodoPago::Efectivo);
        transmision.mejorar_suscripcion(1);
        let suscripcion = transmision.suscripciones.first().unwrap();
        assert_eq!(suscripcion.tipo, TipoSuscripcion::Premium);
    }

    #[test]
    fn test_mejorar_suscripcion() {
        let mut transmision = TransmisionRust::new();
        transmision.crear_suscripcion(1, "Tomas".to_string(), TipoSuscripcion::Basica, 3, MetodoPago::Efectivo);
        transmision.mejorar_suscripcion(1);
        let suscripcion = transmision.suscripciones.first().unwrap();
        assert_eq!(suscripcion.tipo, TipoSuscripcion::Clasica);
    }

    #[test]
    fn test_crear_suscripcion() {
        let mut transmision = TransmisionRust::new();
        transmision.crear_suscripcion(1, "Tomas".to_string(), TipoSuscripcion::Basica, 3, MetodoPago::Efectivo);
        let usuario = transmision.obtener_usuario(1).unwrap();
        assert_eq!(usuario.nombre, "Tomas");
        assert_eq!(usuario.metodo_pago, MetodoPago::Efectivo);
        let suscripcion = transmision.suscripciones.first().unwrap();
        assert_eq!(suscripcion.tipo, TipoSuscripcion::Basica);
        assert_eq!(suscripcion.duracion_meses, 3);
        assert_eq!(suscripcion.id_usuario, 1);
    }

    #[test]
    fn test_costo_suscripcion() {
        let suscripcion = Suscripcion::new(TipoSuscripcion::Basica, 3, 1);
        assert_eq!(suscripcion.costo(), 30.0);
    }
}
