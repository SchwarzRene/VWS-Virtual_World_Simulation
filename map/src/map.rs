pub mod plant;
pub mod water;

pub struct Map{
    pub plants : Vec<plant::Plant>,
    pub waters : Vec<water::Water>
}