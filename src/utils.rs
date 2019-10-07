use rand::Rng;

pub fn random() -> String {
    format!("{:08x}", rand::thread_rng().gen::<u32>())
}
