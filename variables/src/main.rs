const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    let a = "s";
    println!("Firing {} of my {} mistakes...", ready, missiles);
    // missiles -= ready;

    println!("{} missiles left", missiles - ready);
}
