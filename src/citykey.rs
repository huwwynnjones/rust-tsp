use internment::Intern;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct CityKey {
    pub start: Intern<String>,
    pub end: Intern<String>,
}

impl CityKey {
    pub fn new(start: Intern<String>, end: Intern<String>) -> Self {
        CityKey { start, end }
    }

    pub fn from(city_pair: &(Intern<String>, Intern<String>)) -> Self {
        CityKey {
            start: city_pair.0,
            end: city_pair.1,
        }
    }

    pub fn reverse_key(&mut self) -> Self {
        CityKey {
            start: self.end,
            end: self.start,
        }
    }
}
