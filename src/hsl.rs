use crate::RGB;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct HSL {
    h: f64,
    s: f64,
    l: f64,
}

impl HSL {
    pub fn from_rgb(x: &RGB) -> HSL {
        let max = x.r.max(x.g.max(x.b)) as f64;
        let min = x.r.min(x.g.min(x.b)) as f64;
        let r = x.r as f64;
        let g = x.g as f64;
        let b = x.b as f64;

        let h = {
            if min == max {
                0.0
            } else if min == r {
                60.0 * ((b - g) / (max - min)) + 180.0
            } else if min == g {
                60.0 * ((r - b) / (max - min)) + 300.0
            } else {
                60.0 * ((g - r) / (max - min)) + 60.0
            }
        };

        let s = {
            if min == max {
                0.0
            } else {
                (max - min) / (255.0 - (max + min - 255.0).abs()) * 100.0
            }
        };

        let l = (max + min) / 2.0 / 255.0 * 100.0;

        HSL { h, s, l }
    }

    pub fn to_rgb(&self) -> RGB {
        let s = self.s / 100.0;
        let l = self.l * 2.55;

        let max = l + s * (1.0 - (2.0 * l - 1.0).abs()) / 2.0;
        let min = l - s * (1.0 - (2.0 * l - 1.0).abs()) / 2.0;

        println!("{} {}", max, min);
        RGB::default()

        // if self.h >= 300.0 {
        // max.round()
        // }
    }
}
