#[derive(Hash, PartialEq, Eq, Debug)]
pub struct CityKey {
    start: String,
    end: String,
}

impl CityKey {
    pub fn new(start: &str, end: &str) -> Self {
        CityKey {
            start: start.into(),
            end: end.into(),
        }
    }

    pub fn from(city_pair: &[&str]) -> Self {
        CityKey {
            start: city_pair[0].into(),
            end: city_pair[1].into(),
        }
    }

    pub fn start(&self) -> &str {
        &self.start
    }

    pub fn end(&self) -> &str {
        &self.end
    }

    pub fn reverse_key(&mut self) -> Self {
        CityKey {
            start: self.end.clone(),
            end: self.start.clone(),
        }
    }
}
