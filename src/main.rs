fn main() {
    let v = vec![10, 20, 30];

    v.clone().into_iter().for_each(|x| {
        println!("x: {x}");
    });

    for x in v {
        println!("x: {x}");
    }

    for i in (0..10).step_by(2) {
        println!("i: {i}");
    }
}
