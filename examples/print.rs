use ref_mut_n::ref_mut_n;

fn main() {
    let mut data = [[0u8; 3]; 4];
    let _ref_data: &mut [&mut [u8]] = ref_mut_n!(data, 4);
}
