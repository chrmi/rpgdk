use rpgdk::player;
use std::env;

fn intro() {
    println!();
    println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    println!("******* Roll-Playing Game Developer Kit *******");
    println!("*-------------- Legend of Demos --------------*");
    println!("*=========== A Link to the Source ============*");
    println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let name:String;

    intro();

    match args.len() {
        2 => {
            name = String::from(&args[1])
        }
        _ => {
            println!("... No name?  You shall be called... Delza!");
            name = String::from("Delza")
        }
    }

    let mut p = player::Player {
        name: name,
        level: 1, 
        x: 0,
        y: 0,
    };

    println!("{} enters a cold dungeon.", p.name);

    p.move_xy(0, 1);
    p.move_xy(1, 1);
    p.search(20);
    p.move_xy(2, 1);
    p.search(3);
}
