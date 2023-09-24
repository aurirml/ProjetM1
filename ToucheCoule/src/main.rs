use action::futurcoup;

mod init;
mod action;

fn main() {
    let _tab_j1int: [i32;100]=[0;100];
    let _tab_j1str: [&str;100]=["x";100];
    let _tab_j2int: [i32;100]=[0;100];
    let _tab_j2str: [&str;100]=[" ";100];
    init::display_tab_int(&_tab_j1int);
    init::display_tab_str(&_tab_j1str);

    let coup = action::futurcoup();
    let tir = coup(&coup);
    
}