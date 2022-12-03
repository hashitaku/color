#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn from_hex_str(s: &str) -> Result<RGB, std::num::ParseIntError> {
        let mut idx = 0;
        if s.starts_with('#') {
            idx += 1;
        }

        let r = u8::from_str_radix(&s[idx..idx + 2], 16)?;
        let g = u8::from_str_radix(&s[idx + 2..idx + 4], 16)?;
        let b = u8::from_str_radix(&s[idx + 4..idx + 6], 16)?;

        Ok(RGB { r, g, b })
    }

    pub fn rand() -> RGB {
        RGB {
            r: rand::random::<u8>(),
            g: rand::random::<u8>(),
            b: rand::random::<u8>(),
        }
    }

    pub fn to_hex_str(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn diff<F>(&self, other: &RGB, f: F) -> f64
    where
        F: Fn(&RGB, &RGB) -> f64,
    {
        f(self, other)
    }
}

pub fn euclid(x: &RGB, y: &RGB) -> f64 {
    let dr = x.r as f64 - y.r as f64;
    let dg = x.g as f64 - y.g as f64;
    let db = x.b as f64 - y.b as f64;

    (dr.powi(2) + dg.powi(2) + db.powi(2)).sqrt()
}

pub fn weight_euclid(x: &RGB, y: &RGB) -> f64 {
    let rbar = (x.r as f64 + y.r as f64) / 2.0;
    let dr = x.r as f64 - y.r as f64;
    let dg = x.g as f64 - y.g as f64;
    let db = x.b as f64 - y.b as f64;

    ((2.0 + rbar / 256.0) * dr.powi(2)
        + 4.0 * dg.powi(2)
        + (2.0 + (256.0 - rbar) / 256.0) * db.powi(2))
    .sqrt()
}
