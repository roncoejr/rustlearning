pub fn depositvault() {
    loop {
        println!("deposit token to vault");
        break;
    }
}

pub fn withdrawvault() {
    let mut counter = 0;
    loop {
        println!("withdraw token from vault");
        counter = counter + 1;
        if counter > 3 {
            break;
        }
    }
}
