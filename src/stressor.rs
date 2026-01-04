///Information of the uri we are going to stress,
///and how to stress it
struct Stressor {
    pub description: String,
    pub raw_strength: f16,
}

impl Stressor {
    pub fn new(description: &str, raw_strength: f16) -> Self {
        Self {
            description,
            raw_strength,
        }
    }
}
