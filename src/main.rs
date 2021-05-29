mod balances;
mod withdraw;
mod deposit;
mod vault;
mod transfer;

fn main() {
    println!("Hello, world!");
    balances::checkbalance();
    vault::withdrawvault();
}
