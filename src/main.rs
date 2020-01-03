use rpgdk::player;

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
    let mut p = player::Player {
        name: String::from("Delza"), 
        level: 1, 
        x: 0,
        y: 0,
    };

    intro();
    p.move_xy(0, 1);
    p.move_xy(1, 1);
    p.search(20);
    p.move_xy(2, 1);
    p.search(3);
}
