fn main() {
    let mut thingies: [f32; 5] = [50.0, 25.0, 32.0, 45.0, 42.0];

    for x in 0..thingies.len() {
        thingies[x] = (thingies[x] - 32.0) * (5.0/9.0);
    }

    println!("{:?}", thingies)
}
