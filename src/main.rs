mod lib;

fn main() {
    println!("\n=== 30 Seconds of Rust ===\n");
    println!("hex_to_rgb: FFA501 to {:?}", lib::hex_to_rgb("FFA501"));
    println!("rgb_to_hex: [255, 165, 1] to {}", lib::rgb_to_hex(255, 165, 1));
    println!("all_equal: [2, 2, 2] is {}", lib::all_equal(vec![2, 2, 2]));
    println!("all_unique: [1, 2, 3] is {}", lib::all_unique(vec![1, 2, 3]));
}

