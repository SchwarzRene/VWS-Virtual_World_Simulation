use rabit::RabitStruct;

pub mod rabit;


#[derive(PartialEq)]
pub enum Gender {
    Male,
    Female
}

pub enum AnimalType {
    Rabit(rabit::RabitStruct)
}

impl AnimalType{
    pub fn update_animal( &self ){
        self.update();
    }
}


#[derive(PartialEq)]
pub enum Typ{
    Rabit
}


pub struct Animal{
    typ : AnimalType,
    gender : Gender,
    hunger : u8,
    lust : u8,
    attractiveness : u8,
    thirst : u8,
    position : Vec<usize>
}

impl Animal {
    fn setup( typ : Typ, gender : Gender, position : Vec<usize> ) -> Self{

        match typ{
            Typ::Rabit => {
                Animal{
                    typ : AnimalType::Rabit( RabitStruct{
                        speed : 1.,
                    }),
                    gender : gender,
                    hunger : 0,
                    lust : 0,
                    thirst : 0,
                    attractiveness : 0,
                    position : position
                }
            }
        }
        
    }

    fn update( &self ){
        self.typ.update();
    }
}