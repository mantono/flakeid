use flakeid::gen::FlakeGen;

pub fn main() {
    let mut gen = FlakeGen::new().expect("Unable to create generator");
    let id = gen.next().expect("Unable to generate ID");
    println!("base64: {id}");
    println!("binary: {id:b}");
}
