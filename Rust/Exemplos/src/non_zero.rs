use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct NonZeroU8(u8);

impl NonZeroU8 {
    pub fn new(num: u8) -> Option<Self> {
        if num == 0 {
            None
        } else {
            Some(Self(num))
        }
    }
    /// Método que muda o numero interior e morre (consome).
    pub fn mutate_and_consume(mut self) {
        self.0 = 3
    }
    /// Método que muda o numero interior e não morre (acessa valor por referencia).
    pub fn mutate(&mut self) {
        self.0 = 3
    }
    /// Método retorna o numero interior e morre (consome).
    pub fn read_and_consume(self) -> u8 {
        self.0
    }
    /// Método retorna o numero interior e não morre (acessa valor por referencia).
    pub fn read(&self) -> u8{
        self.0
    }

}

impl Display for NonZeroU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}