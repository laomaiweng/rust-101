fn main() {
    let some_u8 = Some(42u8);
    if let Some(42u8) = some_u8 {
        println!("forty-two!");
    }
    if some_u8 == Some(42u8) {
        println!("forty-two too…");
    }
    if let Some(aaa) = some_u8 {
        println!("{}", aaa);
    }
}
