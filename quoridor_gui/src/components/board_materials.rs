use crate::*;

pub struct BoardMaterials {
    pub base_mat_handle: Handle<ColorMaterial>,
    pub wall_slot_mat_handle: Handle<ColorMaterial>,
    pub select: Handle<ColorMaterial>,
    pub highlight: Handle<ColorMaterial>,
    pub wall_mat_handle: Handle<ColorMaterial>,
    pub pawn_materials: Vec<Handle<ColorMaterial>>,
}

impl FromResources for BoardMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let player_count = resources.get_mut::<Quoridor>().unwrap().player_count;

        BoardMaterials {
            base_mat_handle: materials.add(Color::rgb(0.08, 0.01, 0.003).into()),
            wall_slot_mat_handle: materials.add(Color::rgb(0.175, 0.045, 0.004).into()),
            select: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
            highlight: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
            wall_mat_handle: materials.add(Color::rgb(0.32, 0.16, 0.04).into()),
            pawn_materials: (0..player_count).fold(vec![], |mut vec, i| {
                let color = to_rgb(((256 / player_count as u16) as u8) * i, 255, 128);
                vec.push(materials.add(Color::rgb_u8(color.0, color.1, color.2).into()));
                vec
            }),
        }
    }
}

pub fn to_rgb(h: u8, s: u8, l: u8) -> (u8, u8, u8) {
    if s == 0 {
        // Achromatic, i.e., grey.
        return (l, l, l);
    }

    let h = (h as f64) / 255.0; // treat this as 0..1 instead of degrees
    let s = (s as f64) / 255.0;
    let l = (l as f64) / 255.0;

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - (l * s)
    };
    let p = 2.0 * l - q;

    fn percent_to_byte(percent: f64) -> u8 {
        (percent * 255.0).round() as u8
    }

    fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
        // Normalize
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

    (
        percent_to_byte(hue_to_rgb(p, q, h + 1.0 / 3.0)),
        percent_to_byte(hue_to_rgb(p, q, h)),
        percent_to_byte(hue_to_rgb(p, q, h - 1.0 / 3.0)),
    )
}
