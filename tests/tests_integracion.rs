use bomberman::{bomberman_game, turno};

#[test]
fn test_bomberman_mapa1() {
    let mut mapa = bomberman_game::transformar_a_mapa("mapas/mapa_1.txt").unwrap();
    let _ = turno::jugar_turno(&mut mapa, 0, 0);
    let resultado_deseado = bomberman_game::transformar_a_mapa("mapas/mapa_1_deseado.txt").unwrap();

    assert_eq!(mapa, resultado_deseado);
}

#[test]
fn test_bomberman_mapa2() {
    let mut mapa = bomberman_game::transformar_a_mapa("mapas/mapa_2.txt").unwrap();
    let _ = turno::jugar_turno(&mut mapa, 2, 4);
    let resultado_deseado = bomberman_game::transformar_a_mapa("mapas/mapa_2_deseado.txt").unwrap();

    assert_eq!(mapa, resultado_deseado);
}

#[test]
fn test_bomberman_mapa3() {
    let mut mapa = bomberman_game::transformar_a_mapa("mapas/mapa_3.txt").unwrap();
    let _ = turno::jugar_turno(&mut mapa, 0, 4);
    let resultado_deseado = bomberman_game::transformar_a_mapa("mapas/mapa_3_deseado.txt").unwrap();

    assert_eq!(mapa, resultado_deseado);
}
