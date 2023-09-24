use action::futurcoup;

mod action;
mod init;
mod modifmatrice;

fn main() {
    let mut matrice = init::display_tab_int();

    let mut i = 0;

    loop {
        let coup = futurcoup();
        let tir = action::coup(&coup);

        modifmatrice::modif(tir.1, tir.0, &mut matrice);

        for row in &matrice {
            for &element in row {
                print!("{} ", element);
            }
            println!(); // Passer à la ligne suivante pour la prochaine ligne de la matrice
        }
        i += 1;

        if i >= 10{
            return;
        }
    }
}
