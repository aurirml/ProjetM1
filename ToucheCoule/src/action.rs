use std::io;

pub(crate) fn futurcoup() -> String {
    println!("Quelle case voulez-vous viser ?");

    let mut coup = String::new();

    io::stdin().read_line(&mut coup).expect("Échec de l'entrée");

    coup.trim().to_string()
}

pub(crate) fn coup(a: &str) -> String {
    let mut colonne = 0;
    let mut ligne: i32 = 0;

    let mut tir = String::new();

    let mut coup = Vec::new();

    let c = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
    let l = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

    for character in a.chars() {
        coup.push(character);
    }

    for _item in &coup {
        if coup.len() == 3 {
            for character in a.chars() {
                for i in 0..10 {
                    if character.to_string() == c[i] {
                        colonne = i;
                        break;
                    }
                }
            }
            ligne = 9;
        } else {
            for character in a.chars() {
                for i in 0..10 {
                    if character.to_string() == c[i] {
                        colonne = i;
                        break;
                    }
                }

                for j in 0..10 {
                    if character.to_string() == l[j] {
                        ligne = j as i32;
                        break;
                    }
                }
            }
        }

        let c = colonne.to_string();
        let l = ligne.to_string();

        tir = c + &l;

    }

    return tir;
}