use nested_array_ref::{nested_ref, nested_ref2};

fn main() {
    pretty_env_logger::init();

    nested_ref!(0, 3, 5, raw, ref_raw);
    nested_ref2!([[u8; 4]; 6]);

    <[u8; 4]>::default();

    print_type_of(&ref_raw);
}

fn print_type_of<T>(_: &T) {
    assert_eq!("&mut [&mut [i32; 3]; 5]", std::any::type_name::<T>())
}
