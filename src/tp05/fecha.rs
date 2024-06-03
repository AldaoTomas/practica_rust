use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct  Fecha {
    pub dia: u32,
    pub mes: u32,
    pub año: i32,
}


impl Fecha{

    pub fn new (dia: u32, mes:u32, año:i32) -> Self{
        Fecha { 
            dia,
            mes, 
            año, 
        }
    }

    pub fn es_bisiesto (&self) -> bool{
        self.año % 4 == 0 && (self.año % 100 != 0 || self.año % 400 == 0)
    }

    pub fn es_fecha_valida (&self) -> bool {
        if self.mes < 1 || self.mes > 12 {
            return false;
        }
        if self.dia < 1 {
            return false;
        }
        match self.mes {
            4 | 6 | 9 | 11 => {
                if self.dia <= 30{
                    return true;
                }
                else {
                    return false;
                }
            } 
            2 => {
                if Fecha::es_bisiesto(&self){
                    self.dia <= 29
                }
                else {
                    self.dia <= 28
                }
            }
            _ => {
                if self.dia <= 31{
                    return true;
                }
                else {
                    return false;
                }
            }
            
        }
        
    }


    


    pub fn sumar_dias(&mut self, mut dias: u32) {
        while dias > 0 {
            let dias_en_mes = match self.mes {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 if self.es_bisiesto()=> 29,
                _=> 28,
            };
            let dias_restantes = dias_en_mes - self.dia + 1;
            if dias_restantes <= dias {
                dias -= dias_restantes;
                self.mes += 1;
                if self.mes > 12 {
                    self.mes = 1;
                    self.año += 1;
                }
                self.dia = 1;
            } else {
                self.dia += dias;
                dias = 0;
            }
        }
    }

    pub fn restar_dias(&mut self, mut dias: u32) {
        while dias > 0 {
            let dias_en_mes = match self.mes {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 if self.es_bisiesto() => 29,
                _ => 28,
            };
            if self.dia > dias {
                self.dia -= dias;
                dias = 0;
            } else {
                dias -= self.dia;
                self.mes -= 1;
                if self.mes < 1 {
                    self.mes = 12;
                    self.año -= 1;
                }
                self.dia = match self.mes {
                    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                    4 | 6 | 9 | 11 => 30,
                    2 if self.es_bisiesto() => 29,
                    _ => 28,
                };
            }
        }
    }

    pub fn es_mayor(&self, fecha: &Fecha) -> bool {
        if self.año > fecha.año {
            return true;
        } else if self.año == fecha.año {
            if self.mes > fecha.mes {
                return true;
            } else if self.mes == fecha.mes {
                if self.dia > fecha.dia {
                    return true;
                }
            }
        }
        false
    }


}

#[cfg(test)]
mod tests {
    use super::Fecha;

    #[test]
    fn test_new() {
        let fecha = Fecha::new(1, 1, 2020);
        assert_eq!(fecha.dia, 1);
        assert_eq!(fecha.mes, 1);
        assert_eq!(fecha.año, 2020);
    }

    #[test]
    fn test_es_bisiesto() {
        assert!(Fecha::new(1, 1, 2020).es_bisiesto());
        assert!(!Fecha::new(1, 1, 2019).es_bisiesto());
        assert!(!Fecha::new(1, 1, 1900).es_bisiesto());
        assert!(Fecha::new(1, 1, 2000).es_bisiesto());
    }

    #[test]
    fn test_es_fecha_valida() {
        assert!(Fecha::new(29, 2, 2020).es_fecha_valida());
        assert!(!Fecha::new(29, 2, 2019).es_fecha_valida());
        assert!(Fecha::new(31, 12, 2020).es_fecha_valida());
        assert!(!Fecha::new(31, 4, 2020).es_fecha_valida());
        assert!(!Fecha::new(0, 1, 2020).es_fecha_valida());
        assert!(!Fecha::new(1, 13, 2020).es_fecha_valida());
    }

    #[test]
    fn test_sumar_dias() {
        let mut fecha = Fecha::new(28, 2, 2020);
        fecha.sumar_dias(1);
        assert_eq!(fecha, Fecha::new(29, 2, 2020));
        
        let mut fecha = Fecha::new(28, 2, 2019);
        fecha.sumar_dias(1);
        assert_eq!(fecha, Fecha::new(1, 3, 2019));
    
        let mut fecha = Fecha::new(31, 12, 2020);
        fecha.sumar_dias(1);
        assert_eq!(fecha, Fecha::new(1, 1, 2021));
    
        let mut fecha = Fecha::new(30, 6, 2020);
        fecha.sumar_dias(1);
        assert_eq!(fecha, Fecha::new(1, 7, 2020));
    
        let mut fecha = Fecha::new(1, 1, 2020);
        fecha.sumar_dias(365);
        assert_eq!(fecha, Fecha::new(31, 12, 2020)); 
    }
    
    #[test]
    fn test_restar_dias() {
        let mut fecha = Fecha::new(1, 3, 2020);
        fecha.restar_dias(1);
        assert_eq!(fecha, Fecha::new(29, 2, 2020));
        
        let mut fecha = Fecha::new(1, 3, 2019);
        fecha.restar_dias(1);
        assert_eq!(fecha, Fecha::new(28, 2, 2019));
    
        let mut fecha = Fecha::new(1, 1, 2021);
        fecha.restar_dias(1);
        assert_eq!(fecha, Fecha::new(31, 12, 2020));
    
        let mut fecha = Fecha::new(1, 7, 2020);
        fecha.restar_dias(1);
        assert_eq!(fecha, Fecha::new(30, 6, 2020));
    
        let mut fecha = Fecha::new(1, 1, 2021);
        fecha.restar_dias(365);
        assert_eq!(fecha, Fecha::new(2, 1, 2020));
    }

    #[test]
    fn test_es_mayor() {
        let fecha1 = Fecha::new(1, 1, 2020);
        let fecha2 = Fecha::new(1, 1, 2019);
        assert!(fecha1.es_mayor(&fecha2));
        assert!(!fecha2.es_mayor(&fecha1));

        let fecha1 = Fecha::new(1, 2, 2020);
        let fecha2 = Fecha::new(1, 1, 2020);
        assert!(fecha1.es_mayor(&fecha2));
        assert!(!fecha2.es_mayor(&fecha1));

        let fecha1 = Fecha::new(2, 1, 2020);
        let fecha2 = Fecha::new(1, 1, 2020);
        assert!(fecha1.es_mayor(&fecha2));
        assert!(!fecha2.es_mayor(&fecha1));
    }
}
