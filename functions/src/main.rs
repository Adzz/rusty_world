fn main() {
    // another_function(5);

    let x: Result<u32, u32> = Err(13);
    assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));
}
// fn another_function(x: i32) {
//     println!("Other function takes in {}, {}", x, 6);
// }
