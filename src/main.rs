mod todo;
use todo::structs::done::Done;
use todo::structs::pending::Pending;

fn main() {
    let done = Done::new("買い物");
    println!("title: {}", done.super_struct.title);
    println!("status: {}", done.super_struct.status.stringify());

    let pending = Pending::new("洗濯");
    println!("title: {}", pending.super_struct.title);
    println!("status: {}", pending.super_struct.status.stringify());
}
