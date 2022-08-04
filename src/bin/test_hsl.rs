fn main() {
    let rgb = color::RGB::rand();
    let hsl = color::HSL::from_rgb(&rgb);

    println!("{:?} {}", rgb, rgb.to_hex_str());
    println!("{:?}", hsl);
    let _ = hsl.to_rgb();
}
