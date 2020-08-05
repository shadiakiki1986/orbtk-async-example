use crossbeam::thread;

fn main() {

let people = vec![
    "Alice".to_string(),
    "Bob".to_string(),
    "Carol".to_string(),
];

thread::scope(|s| {
    for person in &people {
        s.spawn(move |_| {
            println!("Hello, {}!", person);
        });
    }
}).unwrap();

}
