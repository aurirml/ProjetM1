pub(crate) fn modif(colonne:usize, ligne:usize, matrice: &mut Vec<Vec<i32>>)-> &mut Vec<Vec<i32>>{

    if ligne < matrice.len() && colonne < matrice[0].len() {
        matrice[ligne][colonne] = 1;
    } else {
        println!("Indices hors limites.");
    }

    matrice
}