pub trait Animal {
    fn barulho(&self) -> String;
    fn fazer_barulho(&self) {
        println!("{}", self.barulho())
    }
}

pub struct Cachorro;

impl Animal for Cachorro {
    fn barulho(&self) -> String {
        "Au!".to_string()
    }
}

pub struct Gato;

impl Animal for Gato {
    fn barulho(&self) -> String {
        "Miau!".to_string()
    }
}

pub fn fazer_barulho(animal: impl Animal) {
    println!("{}", animal.barulho())
}

// trait GatoRepository {
//     fn insert(&self, gato: Gato);
// }
// 
// struct Postgres;
// 
// impl GatoRepository to Postgres {
//     fn insert(&self, gato: Gato) {
//         pg.insert(gato);
//     }
// }
// 
// fn init_db(db: impl GatoRepository) {
// 
// }
// 