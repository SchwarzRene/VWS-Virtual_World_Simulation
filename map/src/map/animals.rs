pub mod rabit;


#[derive(PartialEq)]
pub enum Gender {
    Male,
    Female
}

#[derive(PartialEq)]
pub enum Typ {
    Rabit
}

pub struct AnimalData{
    typ : Typ,
    gender : Typ,
    speed : f32,
    hunger : u8,
    lust : u8,
    thirst : u8,
    position : Vec<usize>
}

pub struct Animal{
    typ : Typ,
    gender : Gender
}

impl Animal {
    fn setup( typ : Typ ){
        if typ == Typ::Rabit{
            let animal_typ = rabit::Rabit::new();
        }
    }

    fn update(){

    }
}