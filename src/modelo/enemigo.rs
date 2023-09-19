use super::coordenada::Coordenada;
use std::collections::HashSet;

/// Enemigo que tiene vida y puede ser afectado por las bombas.
/// Ser alcanzado por una explosion no significa que vaya a ser destruido.
/// Si la vida es menor o igual al daño de la bomba, es destruido
#[derive(Debug, Clone, PartialEq)]
pub struct Enemigo {
    pub x: usize,
    pub y: usize,
    pub vida: u32,
    pub bombas_impactadas: HashSet<Coordenada>,
}

impl Enemigo {
    /// Crea un enemigo.
    pub fn crear(x: usize, y: usize, vida: u32) -> Enemigo {
        Enemigo {
            x,
            y,
            vida,
            bombas_impactadas: std::collections::HashSet::new(),
        }
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

    /// Devuelve las coordenadas del enemigo.
    pub fn coordenadas(&self) -> Coordenada {
        Coordenada {
            x: self.x,
            y: self.y,
        }
    }

    /// Recibe las coordenadas de una bomba y las guarda en el set de bombas que ya impactaron.
    /// Una misma bomba no puede causarle daño a un enemigo mas de una vez.
    pub fn recibir_impacto(&mut self, x: usize, y: usize) {
        self.bombas_impactadas.insert(Coordenada { x, y });
    }

    /// Devuelve true si la bomba ya impacto en el enemigo.
    pub fn ya_impactado(&self, x: usize, y: usize) -> bool {
        self.bombas_impactadas.contains(&Coordenada { x, y })
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
