#[derive(Debug, Clone)]
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
        let mut aux: u32;
        while dias > 0 {
            match self.mes {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                    if self.dia >= dias {
                        self.dia -= dias;
                        dias = 0;
                    } else {
                        dias -= self.dia;
                        self.mes -= 1;
                        self.dia = 31;
                        if self.mes < 1 {
                            self.año -= 1;
                            self.mes = 12;
                        }
                    }
                }
                2 => {
                    if self.es_bisiesto() {
                        aux = 29;
                    } else {
                        aux = 28;
                    }
                    if self.dia >= dias {
                        self.dia -= dias;
                        dias = 0;
                    } else {
                        dias -= self.dia;
                        self.mes -= 1;
                        self.dia = aux;
                        if self.mes < 1 {
                            self.año -= 1;
                            self.mes = 12;
                        }
                    }
                }
                _=> {
                    if self.dia >= dias {
                        self.dia -= dias;
                        dias = 0;
                    } else {
                        dias -= self.dia;
                        self.mes -= 1;
                        self.dia = 30;
                        if self.mes < 1 {
                            self.año -= 1;
                            self.mes = 12;
                        }
                    }
                }
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

#[test]
fn constructor() {
    let fecha = Fecha::new(5, 6, 2004);
    assert_eq!(fecha.dia, 5);
    assert_eq!(fecha.mes, 6);
    assert_eq!(fecha.año, 2004);
}

#[test]
fn fecha_valida() {
    let fecha = Fecha::new(5, 6, 2004);
    assert_eq!(fecha.es_fecha_valida(), true);
}

#[test]
fn bisiesto() {
    let fecha = Fecha::new(5, 6, 2003);
    assert_eq!(fecha.es_bisiesto(), false);
    let fecha = Fecha::new(5, 6, 2004);
    assert_eq!(fecha.es_bisiesto(), true);
}
#[test]
fn sumar_y_restar_dias() {
    let mut fecha = Fecha::new(5, 6, 2004);
    fecha.sumar_dias(3);
    assert_eq!(fecha.dia, 8);
    assert_eq!(fecha.mes, 6);
    assert_eq!(fecha.año, 2004);

    fecha.restar_dias(3);
    assert_eq!(fecha.dia, 5);
    assert_eq!(fecha.mes, 6);
    assert_eq!(fecha.año, 2004);
}

#[test]
fn mayor() {
    let fecha1 = Fecha::new(7, 6, 2002);
    let fecha2 = Fecha::new(7, 6, 2002);
    assert_eq!(fecha1.es_mayor(&fecha2), false);
    let fecha2 = Fecha::new(7, 6, 2001);
    assert_eq!(fecha1.es_mayor(&fecha2), true);
    let fecha2 = Fecha::new(7, 5, 2002);
    assert_eq!(fecha1.es_mayor(&fecha2), true);
    let fecha2 = Fecha::new(6, 6, 2002);
    assert_eq!(fecha1.es_mayor(&fecha2), true);
}
