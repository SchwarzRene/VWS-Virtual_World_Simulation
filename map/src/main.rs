pub mod map;

fn main() {
    let mut w = map::water::Water::new();
    w.generate_water( 4 );

    println!( "{:?}", w.position );
}
