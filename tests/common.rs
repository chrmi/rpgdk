extern crate rpgdk;

#[test]
fn player() {
    let p = rpgdk::player::Player {
        name: String::from("ITestName"), 
        level: 1, 
        x: 0,
        y: 0,
    };

    assert!(p.name == "ITestName", "Player name: {}", p.name);
}
