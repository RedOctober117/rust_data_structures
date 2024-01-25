use std::rc::Rc;

fn main() {
    let five = 5;
    let rc_five = Rc::new(five);
    take_ownership(rc_five);
    print!("after own {}", five);
}

fn take_ownership(value: Rc<i32>) {
    println!("{}", value);
}