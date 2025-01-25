mod data;

fn main() {
    let data = data::get_data();
    println!("{:#?}", data);
}
