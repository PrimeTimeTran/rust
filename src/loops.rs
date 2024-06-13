pub fn demo_iter() {
    // foobar, foo, bar, baz, qux, quux, corge, grault, garply, waldo, fred, plugh, xyzzy, and thud
    // "qux", "quux", "corge", "grault", garply, waldo, fred, plugh, xyzzy, and thud
    let names = vec!["foo", "bar", "baz"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);
}

pub fn demo_into_iter() {
    let names = vec!["spam", "ham", "eggs", "cheese"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

pub fn demo_iter_mut() {
    let mut names = vec!["qux", "quux", "corge", "grault"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
}
