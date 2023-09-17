use super::coordenada::Coordenada;

#[derive(Debug, Clone, PartialEq)]
pub struct Enemigo {
    pub x: usize,
    pub y: usize,
    pub vida: u32,
}

impl Enemigo {
    /// Crea un enemigo.
    pub fn crear(x: usize, y: usize, vida: u32) -> Enemigo {
        Enemigo { x, y, vida }
    }

    /// Descuenta vida al enemigo.
    /// Si la vida es menor o igual al daño, la vida se setea en 0.
    pub fn descontar_vida(&mut self, dmg: u32) {
        if self.vida <= dmg {
            self.vida = 0;
        } else {
            self.vida -= dmg;
        }
    }

    pub fn coordenadas(&self) -> Coordenada {
        Coordenada {
            x: self.x,
            y: self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemigo_crear() {
        let enemigo = Enemigo::crear(0, 0, 1);
        assert_eq!(enemigo.x, 0);
        assert_eq!(enemigo.y, 0);
        assert_eq!(enemigo.vida, 1);
    }

    #[test]
    fn test_enemigo_descontar_vida() {
        let mut enemigo = Enemigo::crear(0, 0, 2);
        enemigo.descontar_vida(1);
        assert_eq!(enemigo.vida, 1);
        enemigo.descontar_vida(1);
        assert_eq!(enemigo.vida, 0);
    }
}
