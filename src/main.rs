
mod doc;
mod writer;
mod reader;

pub use doc::{DocValue};

fn main(){

    let mut person = DocValue::object();

    person.insert("name","akku");
    person.insert("bin",vec![1,2,3]);
    person.insert("age","five");
    person.insert("score",12.65);
    person.insert(&"king",true);
    person.insert("network",());

    let mut scores = DocValue::vec();
    scores.push(12.5);
    scores.push(15);
    scores.push(());
    scores.push(false);

    let mut game_match = DocValue::object();
    game_match.insert("scores",scores);

    let mut game = DocValue::object();
    game.insert("match", game_match);
    game.insert("game","criket");

    person.insert("sports",game);

    println!("{:?}",person);

    let bin = person.write();
    let rebuild = DocValue::read(&bin);

    println!("rebuild : {:#?}",rebuild);


}
