use std::collections::HashSet;

use super::coordenada::Coordenada;

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
            x: x,
            y: y,
            vida: vida,
            bombas_impactadas: HashSet::new(),
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

    /// Agrega una bomba al set de bombas que ya impactaron en este turno.
    pub fn recibir_impacto(&mut self, x: usize, y: usize) {
        self.bombas_impactadas.insert(Coordenada { x: x, y: y });
    }

    /// Devuelve true si la bomba ya impacto en este turno, caso contrario false.
    pub fn ya_impactado(&self, x: usize, y: usize) -> bool {
        self.bombas_impactadas.contains(&Coordenada { x: x, y: y })
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

    #[test]
    fn test_enemigo_recibir_impacto() {
        let mut enemigo = Enemigo::crear(0, 0, 1);
        enemigo.recibir_impacto(0, 0);
        assert!(enemigo.ya_impactado(0, 0));
        assert!(!enemigo.ya_impactado(0, 1));
    }

    #[test]
    fn test_enemigo_ya_impactado() {
        let mut enemigo = Enemigo::crear(0, 0, 1);
        enemigo.recibir_impacto(0, 0);
        assert!(enemigo.ya_impactado(0, 0));
        assert!(!enemigo.ya_impactado(0, 1));
    }
}
