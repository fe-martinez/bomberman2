use super::coordenada::Coordenada;


#[derive(Clone, Debug)]
pub struct Bomba {
    pub x: usize,
    pub y: usize,
    pub radio: u32,
    pub especial: bool,
    pub enemigos_afectados: Vec<Coordenada>,
}