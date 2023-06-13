use flakeid::gen::FlakeGen;

fn main() {
    let mut gen = FlakeGen::with_mac_addr().expect("Unable to create generator");
    let id = gen.next().expect("Unable to generate ID");
    println!("{:<8}: {id:b}", "binary");
    println!("{:<8}: {}", "decimal", id.value());
    println!("{:<8}: {id:x}", "hex");
    println!("{:<8}: {id}", "base64");
}
