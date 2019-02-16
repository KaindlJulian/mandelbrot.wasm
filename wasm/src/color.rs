#[derive(Clone, Copy, Debug)]
pub struct HSL {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

// defaults to black
impl Default for HSL {
    fn default () -> HSL {
        HSL{h: 0.0, s: 0.0, l: 0.0}
    }
}

impl HSL {
    pub fn to_rgba(&self) -> (u8, u8, u8, u8) {
        if self.s == 0.0 {
            let l = percent_to_byte(self.l);
            return (l, l, l, 255);
        }

        let h = self.h / 360.0;
        let s = self.s;
        let l = self.l;

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - (l * s)
        };
        let p = 2.0 * l - q;

        (percent_to_byte(hue_to_rgb(p, q, h + 1.0 / 3.0)),
         percent_to_byte(hue_to_rgb(p, q, h)),
         percent_to_byte(hue_to_rgb(p, q, h - 1.0 / 3.0)),
         255)
    }
}

fn percent_to_byte(percent: f64) -> u8 {
    (percent * 255.0).round() as u8
}

fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
    let t = if t < 0.0 {
        t + 1.0
    } else if t > 1.0 {
        t - 1.0
    } else {
        t
    };

    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}