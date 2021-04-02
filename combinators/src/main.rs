use std::sync::Arc;
use std::sync::Mutex;

struct Data {
    member: Option<i32>,
}

impl Data {
    fn new(num: i32) -> Self {
        Data { member: Some(num) }
    }
    fn read(&self) -> Option<i32> {
        self.member
    }
}

fn combinators(arc: Arc<Mutex<Data>>) {
    match arc.lock().map(|guard| guard.read()).and_then(|member| {
        Ok(member.map(|num| num * num).and_then(|result| {
            println!("squared {}", result);
            Some(())
        }))
    }) {
        Ok(_) => println!("success"),
        Err(_) => println!("fail"),
    }
}

fn main() {
    let data = Data::new(4);
    let arc = Arc::new(Mutex::new(data));
    combinators(arc.clone());
}
