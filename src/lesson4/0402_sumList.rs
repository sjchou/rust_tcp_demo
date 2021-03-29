fn sum(list:&[u32])->Option<u32> {
    let mut sumResult:Option<u32> = Some(0);
    for number in list.iter() {
        sumResult = match number.checked_add(sumResult.unwrap()) {
            Some(val) => Some(val),
            None => None,
        };
    }
    sumResult
}
fn main() {
    let mut list: [u32; 5] = [1, 2, 3, 4, u32::max_value()];
    println!("result:{:?}", sum(&list));
    list = [1, 2, 3, 4, 5];
    println!("result:{:?}", sum(&list));
}
