use rand::Rng;

pub fn generate() -> String {
    let mut rng = rand::thread_rng();

    (0..6)
        .map(|_| {
            let index = rng.gen_range(0..36);
            if index < 10 {
                (b'0' + index) as char
            } else {
                (b'a' + index - 10) as char
            }
        })
        .collect()
}
