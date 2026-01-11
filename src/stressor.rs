// use derive_builder::Builder;
///Information of the uri we are going to stress,
///and how to stress it
// #[derive(Builder)]
pub struct Stressor {
    pub description: String,
    ///Minimum length between requests
    pub reqwest_delay: f32,
    pub url: String,
}

impl Stressor {
    pub fn default() -> Self {
        Self {
            description: "default".to_string(),
            reqwest_delay: 1.0,
            url: "unknown".to_string(),
        }
    }

    pub fn optional_description(&mut self, optional_description: Option<String>) -> &mut Stressor {
        if let Some(descript) = optional_description {
            self.description(descript);
        } else {
            self.description(String::new());
        };
        self
    }
    pub fn description(&mut self, description: String) -> &mut Stressor {
        self.description = description;
        self
    }

    pub fn reqwest_delay(&mut self, reqwest_delay: f32) -> &mut Stressor {
        self.reqwest_delay = reqwest_delay;
        self
    }

    pub fn url(&mut self, url: String) -> &mut Stressor {
        self.url = url;
        self
    }
}
