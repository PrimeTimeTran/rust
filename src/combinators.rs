#![allow(warnings)]

#[derive(Debug)]
struct Student {
    gpa: f32,
    name: String,
}

pub fn main() {
    let s1 = "Bogdan 3.1";
    let s2 = "Wallace 2.3";
    let s3 = "Lidiya 3.5";
    let s4 = "Kyle 3.9";
    let s5 = "Anatoliy 4.0";

    let withCombinators = false;

    let students = vec![s1, s2, s3, s4, s5];
    let mut good_students = vec![];

    if withCombinators {
        println!("withCombinators");
        good_students = students
            .iter()
            .map(|s| {
                let mut s = s.split(' ');
                let name = s.next()?.to_owned();
                let gpa = s.next()?.parse::<f32>().ok()?;

                Some(Student { name, gpa })
            })
            .flatten()
            .filter(|s| s.gpa >= 3.5)
            .collect();
    }

    if !withCombinators {
        println!("!withCombinators");
        for s in students {
            let mut s = s.split(' ');
            let name = s.next();
            let gpa = s.next();

            if name.is_some() && gpa.is_some() {
                let name = name.unwrap().to_owned();
                let gpa = gpa.unwrap().to_owned().parse::<f32>();

                if gpa.is_ok() {
                    let gpa = gpa.unwrap();
                    if gpa >= 3.5 {
                        good_students.push(Student { name, gpa });
                    }
                }
            }
        }
    }

    for s in good_students {
        println!("{:?}", s);
    }
}
