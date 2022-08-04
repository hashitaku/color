fn main() {
    let origin = color::RGB::rand();

    let comp = {
        let mut ret = Vec::default();
        for _ in 0..20 {
            ret.push(color::RGB::rand());
        }

        ret
    };

    println!("{}", origin.to_hex_str());
    comp.iter().for_each(|x| print!("{} ", x.to_hex_str()));
    println!("");

    let v = {
        let mut ret = comp
            .iter()
            .zip(
                comp.iter()
                    .map(|x| origin.diff(x, color::euclid))
                    .collect::<Vec<_>>(),
            )
            .collect::<Vec<_>>();

        ret.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());

        ret
    };

    v.iter().for_each(|x| print!("{} ", x.0.to_hex_str()));
    println!("");
}
