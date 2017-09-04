pub struct Game;

impl Game {
    pub fn say_hi(&self) {
        println!("{}", Message::HELLO);
    }
}

struct Message;

impl Message {
    const HELLO: &'static str = "hello";
}
