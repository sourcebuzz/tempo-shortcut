//! Procedurally generated tray icons (ARGB32), so no external image assets
//! are needed: a plain clock face while idle, an hourglass while tracking.
use ksni::Icon;

const SIZE: i32 = 32;

fn blank_argb() -> Vec<u8> {
    vec![0u8; (SIZE * SIZE * 4) as usize]
}

fn set_pixel(data: &mut [u8], x: i32, y: i32, argb: [u8; 4]) {
    if x < 0 || y < 0 || x >= SIZE || y >= SIZE {
        return;
    }
    let idx = ((y * SIZE + x) * 4) as usize;
    data[idx..idx + 4].copy_from_slice(&argb);
}

pub fn idle_icon() -> Icon {
    let mut data = blank_argb();
    let center = SIZE as f64 / 2.0;
    let radius = center - 2.0;
    const FACE: [u8; 4] = [255, 84, 110, 122];
    const RIM: [u8; 4] = [255, 38, 50, 56];
    const HAND: [u8; 4] = [255, 236, 239, 241];

    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as f64 + 0.5 - center;
            let dy = y as f64 + 0.5 - center;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist <= radius {
                set_pixel(&mut data, x, y, if dist >= radius - 1.6 { RIM } else { FACE });
            }
        }
    }

    // clock hands pointing to 12 and 3, so it reads as a clock at a glance
    let c = center as i32;
    for i in 0..(radius as i32 - 4) {
        set_pixel(&mut data, c, c - i, HAND);
    }
    for i in 0..((radius as i32 - 4) * 6 / 10) {
        set_pixel(&mut data, c + i, c, HAND);
    }

    Icon { width: SIZE, height: SIZE, data }
}

pub fn running_icon() -> Icon {
    let mut data = blank_argb();
    const GLASS: [u8; 4] = [255, 255, 152, 0];
    const FRAME: [u8; 4] = [255, 66, 66, 66];

    let margin = 6.0;
    let top = margin;
    let bottom = SIZE as f64 - margin;
    let left = margin;
    let right = SIZE as f64 - margin;
    let mid = SIZE as f64 / 2.0;
    let max_half = (right - left) / 2.0;
    let half_range = mid - top;

    for y in 0..SIZE {
        let yf = y as f64 + 0.5;

        if yf <= top + 1.5 || yf >= bottom - 1.5 {
            for x in 0..SIZE {
                let xf = x as f64 + 0.5;
                if xf >= left && xf <= right {
                    set_pixel(&mut data, x, y, FRAME);
                }
            }
            continue;
        }

        // hourglass silhouette: wide at top/bottom, pinched to a thin waist in the middle
        let frac = ((yf - mid).abs() / half_range).clamp(0.0, 1.0);
        let half_width = 1.0f64.max(frac * max_half);
        for x in 0..SIZE {
            let xf = x as f64 + 0.5;
            if (xf - mid).abs() <= half_width {
                set_pixel(&mut data, x, y, GLASS);
            }
        }
    }

    Icon { width: SIZE, height: SIZE, data }
}
