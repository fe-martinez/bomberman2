use std::collections::HashSet;

use super::coordenada::Coordenada;


#[derive(Debug, Clone)]
pub struct Enemigo {
    pub x: usize,
    pub y: usize,
    pub vida: u32,
    pub bombas_impactadas: HashSet<Coordenada>
}

impl Enemigo {
    pub fn crear (x: usize, y: usize, vida: u32) -> Enemigo {
        Enemigo {
            x: x,
            y: y,
            vida: vida,
            bombas_impactadas: HashSet::new()
        }
    }

    pub fn descontar_vida (&mut self, dmg: u32) {
        if self.vida <= dmg {
            self.vida = 0;
        } else {
            self.vida -= dmg;
        }
    }

    pub fn recibir_impacto (&mut self, x: usize, y: usize) {
        self.bombas_impactadas.insert(Coordenada {x: x, y: y});
    }

    pub fn ya_impactado (&self, x: usize, y: usize) -> bool {
        self.bombas_impactadas.contains(&Coordenada {x: x, y: y})
    }
}