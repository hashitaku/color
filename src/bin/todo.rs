fn main() {
    let _j = {
        let f = std::fs::File::open("resources/256-colors.json").unwrap();

        #[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
        #[allow(non_snake_case)]
        struct Obj {
            colorId: u32,
            hexString: String,
            rgb: color::RGB,
            hsl: color::HSL,
            name: String,
        }

        serde_json::from_reader::<_, std::vec::Vec<Obj>>(f).unwrap()
    };
}
