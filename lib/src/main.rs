use lib::greet; // ! lib could be any name of the project same as toml file
fn main() {
    greet();
    let s1 = String::from("asa");
    let s2 = s1;
    println!("{}", s2);
    let s1 = String::from("aaa");
    println!("{}", s1);
}
