const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {
    let _boo = "boo";
    // READY_AMOUNT = 1;
    // let (mut missiles, ready) = (8, 2);
    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left without reassign", missiles - ready);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
