
pub fn es_primo(numerito: u32) -> bool{
    if numerito <= 1 {
        return false; 
    }
    for i in 2..numerito {
        if numerito % i == 0{
            return false;
        }
    }
    return true;
}

