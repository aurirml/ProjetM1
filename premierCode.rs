use std::io::{self, Write};
//use std::{thread, time};


fn display_tab_int(tab: &[[i32;10];10]){
    print!("   ");
    let mut u:usize;
    for u in 0..10{
        print!("{}    ", u+1);
    }
    println!();
    let mut i:usize;
    let mut j:usize=0;
    for lettre in 'A'..'K'{
        print!("{} ", lettre);
        for i in 0..10{
            print!("[{}]  ", tab[i][j]);
            
        }
        j=j+1;
        println!();
    }
    println!();
}




fn display_tab_str(tab: &[[&str;10];10]){
    print!("   ");
    let mut u:usize;
    for u in 0..10{
        print!("{}    ", u+1);
    }
    println!();
    let mut i:usize;
    let mut j:usize=0;
    for lettre in 'A'..'K'{
        print!("{} ", lettre);
        for i in 0..10{
            print!("[{}]  ", tab[i][j]);
            
        }
        j=j+1;
        println!();
    }
    println!();
}


fn init_players(mut gamer: Player) -> Player{
    println!("********Iniialisation du joueur {}********\nPseudonyme : ",gamer.player_number);
    
    /*let hundread_millis = time::Duration::from_millis(100);    
    thread::sleep(hundread_millis);*/
    io::stdin()
        .read_line(&mut gamer.pseudo)
        .expect("Échec de la lecture de l'entrée utilisateur");
    println!("\nBonjour {}",gamer.pseudo);
    return gamer;   
}



fn set_battleships(mut gamer: Player) -> Player{
    let mut input_string= String::new();
    println!("********Joueur {} Veuillez placer vos navires********\n-Porte-avion (5 cases)\n-Croiseur (4 cases)\n-Contre-torpilleur (3 cases)\n-Sous-marin (3 cases)\n-Torpilleur (2 cases)",gamer.player_number);
    println!("Pour placer vos navires, veuillez choisir deux cases définissant les extrémités de vos navires\n");
    println!("********Porte-avion********\nPremière extrémité :");
    input_string.clear();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Échec de la lecture de l'entrée utilisateur");
    gamer.pa.ex1 = input_string.trim().to_string();//.parse().expect("Impossible de convertir en entier");
    input_string.clear();

    println!("Seconde extrémité :"); 
    io::stdin()
        .read_line(&mut input_string)
        .expect("Échec de la lecture de l'entrée utilisateur"); 
    gamer.pa.ex2 = input_string.trim().to_string();//.parse().expect("Impossible de convertir en entier");

    println!("********Croiseur********\nPremière extrémité :");
    input_string.clear();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Échec de la lecture de l'entrée utilisateur");
    gamer.cr.ex1 = input_string.trim().to_string();//.parse().expect("Impossible de convertir en entier");

    println!("Seconde extrémité :");
    input_string.clear();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Échec de la lecture de l'entrée utilisateur");
    gamer.cr.ex2 = input_string.trim().to_string();//.parse().expect("Impossible de convertir en entier");

    println!("********Contre-torpilleur********\nPremière extrémité :");
    input_string.clear();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Échec de la lecture de l'entrée utilisateur");
    gamer.ct.ex1 = input_string.trim().to_string();//.parse().expect("Impossible de convertir en entier");
    
    println!("Seconde extrémité :");
    input_string.clear();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Échec de la lecture de l'entrée utilisateur");
    gamer.ct.ex2 = input_string.trim().to_string();//.parse().expect("Impossible de convertir en entier");
    println!("********Sous-marin********\nPremière extrémité :");
    input_string.clear();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Échec de la lecture de l'entrée utilisateur");
    gamer.sm.ex1 = input_string.trim().to_string();//.parse().expect("Impossible de convertir en entier");
    
    println!("Seconde extrémité :");
    input_string.clear();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Échec de la lecture de l'entrée utilisateur");
    gamer.sm.ex2 = input_string.trim().to_string();//.parse().expect("Impossible de convertir en entier");
    
    println!("********Torpilleur********\nPremière extrémité :");
    input_string.clear();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Échec de la lecture de l'entrée utilisateur");
    gamer.tr.ex1 = input_string.trim().to_string();//.parse().expect("Impossible de convertir en entier");
   
    println!("Seconde extrémité :");
    input_string.clear();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Échec de la lecture de l'entrée utilisateur");
    gamer.tr.ex2 = input_string.trim().to_string();//.parse().expect("Impossible de convertir en entier");

    /*println!("Porte-avion : {} {}\n
            Croiseur : {} {}\n
            Conre-torpilleur : {} {}\n
            Sous-marin : {} {}\n
            Torpilleur : {} {}\n",
            gamer.pa.ex1,gamer.pa.ex2,
            gamer.cr.ex1,gamer.cr.ex2,
            gamer.ct.ex1,gamer.ct.ex2,
            gamer.sm.ex1,gamer.sm.ex2,
            gamer.tr.ex1,gamer.tr.ex2);*/

    return gamer;

}

fn valeur_absolue_usize(nombre: usize) -> usize {
    nombre
}


fn verif_position(ship: Battleship, gamer: Player) -> bool {
    let mut ex1: String;
    let mut ex2: String;
    let tab1:[usize;2];
    let tab2:[usize;2];
    let mut i:usize=0;
    ex1=ship.ex1;
    ex2=ship.ex2;
    for chiffre in ex1.chars() {
        tab1[i]=chiffre.parse().expect("Impossible de convertir en entier");
        i=i+1;
    }
    i=0;
    for chiffre in ex2.chars() {
        tab2[i]=chiffre.parse().expect("Impossible de convertir en entier");
        i=i+1;
    }
    if (tab1[0]<=9 && tab1[0]>=0) && (tab2[0]<=9 && tab2[0]>=0) && (tab1[1]<=9 && tab1[1]>=0) && (tab2[1]<=9 && tab2[1]>=0) {
        if (tab1[0]==tab2[0]) && (valeur_absolue_usize(tab2[1]-tab1[1])==ship.size-1){
            if tab2[1]>tab1[1]{
                for chiffre in tab1[1]..=tab2[1]{
                    gamer.play_grid[tab1[0]][chiffre]=1;
                }
                return true;
            }
            else{
                for chiffre in tab2[1]..=tab1[1]{
                    gamer.play_grid[tab1[0]][chiffre]=1;
                }
                return true;
            }  
        }
        else if (tab1[1]==tab2[1]) && (valeur_absolue_usize(tab2[0]-tab1[0])==ship.size-1){
            if tab2[0]>tab1[0]{
                for chiffre in tab1[1]..=tab2[1]{
                    gamer.play_grid[tab1[1]][chiffre]=1;
                }
                return true;
            }
            else{
                for chiffre in tab2[0]..=tab1[0]{
                    gamer.play_grid[tab1[1]][chiffre]=1;
                }
                return true;
            }  
        }
        else {
            return false;
        }
    }
    else {
        return false;
    }
}




struct Player<'a>{
    pseudo : String,
    player_number: i32,
    play_grid : [[i32;10];10],
    display_grid: [[&'a str;10];10],
    pa: Battleship,
    cr: Battleship,
    ct: Battleship,
    sm: Battleship,
    tr: Battleship,
}

struct Battleship{
    kind: String,
    ex1: String,
    ex2: String,
    size: i32,
    sunk: bool,
}

/*struct PorteAvion{
    kind: String,
    ex1: String,
    ex2: String,
    size: i32,
    sunk: bool,
}

struct Croiseur{
    kind: String,
    ex1: String,
    ex2: String,
    size: i32,
    sunk: bool,
}
struct ContreTorpilleur{
    kind: String,
    ex1: String,
    ex2: String,
    size: i32,
    sunk: bool,
}
struct SousMarin{
    kind: String,
    ex1: String,
    ex2: String,
    size: i32,
    sunk: bool,
}
struct Torpilleur{
    kind: String,
    ex1: String,
    ex2: String,
    size: i32,
    sunk: bool,
}*/


fn main() {
    let PorteAvionJ1 = Battleship{
        kind: "pa".to_string(),
        ex1: String::new(),
        ex2: String::new(),
        size: 5,
        sunk: false,
    };
    let CroiseurJ1 = Battleship{
        kind: "cr".to_string(),
        ex1: String::new(),
        ex2: String::new(),
        size: 4,
        sunk: false,
    };
    let ContreTorpilleurJ1 = Battleship{
        kind: "ct".to_string(),
        ex1: String::new(),
        ex2: String::new(),
        size: 3,
        sunk: false,
    };
    let SousMarinJ1 = Battleship{
        kind: "sm".to_string(),
        ex1: String::new(),
        ex2: String::new(),
        size: 3,
        sunk: false,
    };
    let TorpilleurJ1 = Battleship{
        kind: "tr".to_string(),
        ex1: String::new(),
        ex2: String::new(),
        size: 2,
        sunk: false,
    };

    let PorteAvionJ2 = Battleship{
        kind: "pa".to_string(),
        ex1: String::new(),
        ex2: String::new(),
        size: 5,
        sunk: false,
    };
    let CroiseurJ2 = Battleship{
        kind: "cr".to_string(),
        ex1: String::new(),
        ex2: String::new(),
        size: 4,
        sunk: false,
    };
    let ContreTorpilleurJ2 = Battleship{
        kind: "ct".to_string(),
        ex1: String::new(),
        ex2: String::new(),
        size: 3,
        sunk: false,
    };
    let SousMarinJ2 = Battleship{
        kind: "sm".to_string(),
        ex1: String::new(),
        ex2: String::new(),
        size: 3,
        sunk: false,
    };
    let TorpilleurJ2 = Battleship{
        kind: "tr".to_string(),
        ex1: String::new(),
        ex2: String::new(),
        size: 2,
        sunk: false,
    };

    let mut player_1 = Player{
        pseudo: String::new(),
        player_number: 1,
        play_grid: [[0;10];10],
        display_grid: [[(" ");10];10],
        pa: PorteAvionJ1,
        cr: CroiseurJ1,
        ct: ContreTorpilleurJ1,
        sm: SousMarinJ1,
        tr: TorpilleurJ1,
    };

    let mut player_2 = Player{
        pseudo: String::new(),
        player_number: 2,
        play_grid: [[0;10];10],
        display_grid: [[(" ");10];10],
        pa: PorteAvionJ2,
        cr: CroiseurJ2,
        ct: ContreTorpilleurJ2,
        sm: SousMarinJ2,
        tr: TorpilleurJ2,
    };

    println!("Hello World");
    display_tab_int(&player_1.play_grid);
    display_tab_str(&player_1.display_grid);
    display_tab_int(&player_2.play_grid);
    display_tab_str(&player_2.display_grid);
    player_1=init_players(player_1);
    player_2=init_players(player_2);
    player_1=set_battleships(player_1);
    player_2=set_battleships(player_2);
}