use reqwest::Client;
///Information of the uri we are going to stress,
///and how to stress it
struct Stressor {
    pub description: String,
    pub raw_strength: f32,
    pub num_threads: usize,
    pub url: String,
}

impl Stressor {
    pub fn new(description: String, raw_strength: f32) -> Self {
        Self {
            description,
            ///how much strength to apply? percentage
            raw_strength,
            num_threads: todo!(),
            url: todo!(),
        }
    }
}
