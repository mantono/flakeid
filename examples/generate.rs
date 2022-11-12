use flakeid::gen::FlakeGen;

fn main() {
    let mut gen = FlakeGen::with_mac_addr().expect("Unable to create generator");
    let id = gen.next().expect("Unable to generate ID");
    println!("base64: {id}");
    println!("binary: {id:b}");
}
