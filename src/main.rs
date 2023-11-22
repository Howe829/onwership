fn main() {
    // on the heap
    let s2 = {
        let s1 = String::from("Hello");
        s1.clone()
    };
    println!("The value of s2: {s2}");

    // known size on the stack
    let x = 5;
    let y = x; // the x won't move
    println!("x = {x}, y = {y}");
    takes_ownership(s2);
    // println!("S2 in the main scope: {s2}");
    makes_copy(x);
    let s = gives_ownership();
    let s3 = takes_and_gives_ownership("A little ".to_string());
    println!("s: {s}, s3: {s3}");
    let (a, b) = calculate_length(s3);
    // calculate length by reference
    let c = calculate_length_1(&a);
    println!("The length of \"{a}\": {} calculated by reference: {c}", b);

    let mut d = String::from("Hello ");
    let r1 = &d;
    println!("{}", r1);
    let r2 = &mut d;
    change(r2);
    println!("{d}");
    // println!("{}", r1);

    // let e = dangle();
    let hello = String::from("Hello world");
    // let world = "Hello world";
    // let first = first_word(world);
    let first = first_word(&hello);
    println!("The first word of \"{hello}\" is {first}");

    let list = [1, 2, 3, 4, 5];
    let slice = &list[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn change(s: &mut String) {
    s.push_str("hhh");
}

fn takes_ownership(s: String) {
    println!("The s in takes_ownership:{s}");
}

fn makes_copy(mut x: u32) {
    x += 1;
    println!("The x in makes_copy {x}");
}

fn gives_ownership() -> String {
    let s = String::from("Yours");
    s
}

fn takes_and_gives_ownership(mut s: String) -> String {
   s.push_str("tail.");
   s
}

fn calculate_length(s: String) -> (String, usize) {
   let length = s.len();
   (s, length)
}

fn calculate_length_1(s: &String) -> usize {
    s.len()
}

// fn dangle() -> &String {
//     let s = String::from("Dangling pointer");
//     &s
// }

fn first_word(s: &str) ->&str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
