pub fn dp_rec_mc(amount: u32) -> u32 {
    let cashs = [100,50,30,20,10,5,2,1];
    let mut total = amount;
    let mut ret = 0u32;
    for cash in cashs.iter(){
        ret += total / cash;
        total %= cash;
    }
    ret
}
