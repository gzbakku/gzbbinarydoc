

mod reader;
mod writer;
mod doc;

///
/// let mut person = DocValue::object();
/// 
/// person.insert("name","akku");
/// person.insert("bin",vec![1,2,3]);
/// person.insert("age","five");
/// person.insert("score",12.65);
/// person.insert(&"king",true);
/// person.insert("network",());
/// 
/// let mut scores = DocValue::vec();
/// scores.push(12.5);
/// scores.push(15);
/// scores.push(());
/// scores.push(false);
/// 
/// let mut game_match = DocValue::object();
/// game_match.insert("scores",scores);
pub use doc::DocValue;