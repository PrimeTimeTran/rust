pub fn example() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let x = 5;
    let y = &x; //set y to a reference to x

    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference y
    println!("ogogogo");
    // assert_eq!(5, y);
    //               ^ Would throw an error
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
