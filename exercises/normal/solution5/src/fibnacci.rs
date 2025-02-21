pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut pre_pre = 1;
    let mut pre = 1;
    let mut ret = 2;
    while true{
        let temp = pre;
        pre = pre_pre + pre;
        pre_pre = temp;
        if pre >= threshold{
            break;
        }
        if pre % 2 == 1{
            ret += pre;
        }
    }
    ret
}
