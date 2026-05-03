// use test2::*;
fn main() {
    let list1 = vec![0, 1, 2, 3];
    let list_iter = list1.iter();
    for i in list_iter {
        println!("{}", i);
    }
}
