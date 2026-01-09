use derive_builder::Builder;
///Information of the uri we are going to stress,
///and how to stress it
#[derive(Builder)]
pub struct Stressor {
    pub description: String,
    ///Minimum length between requests
    pub reqwest_delay: f32,
    pub url: String,
}

impl Stressor {
    pub fn new(description: String, raw_strength: f32) -> Self {
        Self {
            description,
            //how much strength to apply? percentage
            raw_strength,
            num_threads: todo!(),
            url: todo!(),
        }
    }

    pub fn description(&mut self, description: String) -> Self {
        self.description = description;
        self
    }
}
