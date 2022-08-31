use goonstation_asm::assemble;

fn main() {
    let bin = assemble("OEN 0\nSTO 0\nLD 7\nSTO F");
    dbg!(bin);

    let bin = assemble("OEN 0\nSTO 0\nLD 7\nSTO");
    dbg!(bin);
}