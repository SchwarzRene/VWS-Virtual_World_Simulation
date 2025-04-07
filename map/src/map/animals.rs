pub mod rabit;

mod animal {
    use super::rabit;

    enum Gender {
        Male,
        Female
    }

    #[derive(PartialEq)]
    enum Typ {
        Rabit
    }

    struct Properties{
        speed : f32,
        hunger : f32,
        lust : f32,
        thirst : f32,
        position : Vec<usize>,
        gender : Gender,
        typ : Typ,
        search_radius : f32
    }
    
    pub fn setup( typ : Typ ){
        let new_anmial = rabit::setup();
    }

    pub fn update( ){

    }
}