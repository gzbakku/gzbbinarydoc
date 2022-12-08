# gzbBinaryDoc
## Fast Binary Document
this is a very fast binary document structure to save supported data.

## features
- supported data types f64,i64,bool,vec<DocValue>,hashmap<String,DocValue>,null,vec<u8>
- reader is fast and does not copy binary data
- 
```rust

use gzbbinarydoc::DocValue;

fn main(){

    let mut person = DocValue::object();

    person.insert("name","akku");
    person.insert("bin",vec![1,2,3]);
    person.insert("age",24);
    person.insert("avg score",12.65);
    person.insert(&"king",true);
    person.insert("network",());//null type

    let mut scores = DocValue::vec();
    scores.push(12.5);
    scores.push(15);
    scores.push(());
    scores.push(false);

    let mut game_match = DocValue::object();
    game_match.insert("scores",scores);

    let mut game = DocValue::object();
    game.insert("match", game_match);
    game.insert("game","cricket");

    person.insert("sports",game);

    println!("{:?}",person);

    let bin = person.write();
    let rebuild = DocValue::read(&bin);

    println!("rebuild : {:#?}",rebuild);


}

````