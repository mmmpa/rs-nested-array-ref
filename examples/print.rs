use nested_array_ref::nested_ref;

fn main() {
    pretty_env_logger::init();

    nested_ref!(0, 3, 5, raw, ref_raw);

    print_type_of(&ref_raw);
    println!("{:?}", ref_raw);
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
