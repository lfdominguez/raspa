const BRUSH_WIDTH: i32 = 6;
// const DEFORMATION_FACTOR: i32 = 2;
// const DELAY: i32 = 3;
//
// const ANGLE_STEP: f32 = 5.0;
// const RANGE_STEP: f32 = (std::f32::consts::PI / 180.0) * ANGLE_STEP;
//
// const HALF_BRUSH_DEFORMED: i32 = (BRUSH_WIDTH * DEFORMATION_FACTOR) / 2;
// const HALF_BRUSH_WIDTH: i32 = BRUSH_WIDTH / 2;
//
// const VERTICAL_MARGIN: i32 = BRUSH_WIDTH / 2;
// const HORIZONTAL_MARGIN: i32 = (BRUSH_WIDTH * DEFORMATION_FACTOR) / 2;
//
// #[derive(Clone, Debug)]
// struct FPoint {
//     x: f32,
//     y: f32,
//     angle: f32,
// }
// impl FPoint {
//     fn new(x: f32, y: f32, angle: f32) -> Self {
//         Self { x, y, angle }
//     }
// }
//
#[derive(Debug, PartialEq, Clone, Copy)]
struct IPoint {
    x: i32,
    y: i32,
}

impl IPoint {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// impl From<FPoint> for IPoint {
//     fn from(FPoint { x, y, .. }: FPoint) -> IPoint {
//         IPoint::new(x as i32, y as i32)
//     }
// }
//
impl std::ops::Add for IPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for IPoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug)]
struct ISegment {
    from: IPoint,
    to: IPoint,
}

impl ISegment {
    fn new<P1: Into<IPoint>, P2: Into<IPoint>>(from: P1, to: P2) -> Self {
        ISegment {
            from: from.into(),
            to: to.into(),
        }
    }

    fn get_line_points(self) -> impl Iterator<Item = IPoint> {
        let IPoint { x, y } = self.to - self.from;
        let length = ((x * x + y * y) as f32).sqrt();
        let dx = x as f32 / length;
        let dy = y as f32 / length;
        dedupe(
            (0..length as i32 + 1).map(move |i| {
                self.from + IPoint::new((i as f32 * dx) as i32, (i as f32 * dy) as i32)
            }),
        )
    }

    fn from_angle<Point: Into<IPoint>>(origin: Point, angle: f32, length: f32) -> Self {
        let p = origin.into();
        let x = angle.cos() * length;
        let y = angle.sin() * length;
        Self::new(p, p + IPoint::new(x as i32, y as i32))
    }
}

struct Canvas {
    rows: u16,
    cols: u16,
}

fn dedupe<T: std::cmp::PartialEq + Copy, It: Iterator<Item = T>>(
    iter: It,
) -> impl Iterator<Item = T> {
    iter.scan(None, |l, r| {
        let result = if let Some(old) = l {
            Some((*old == r, r))
        } else {
            Some((true, r))
        };

        *l = Some(r);
        result
    })
    .filter_map(|(same, r)| if same { None } else { Some(r) })
}

impl Canvas {
    fn new() -> Self {
        let (rows, cols) = termion::terminal_size().unwrap();
        Canvas { rows, cols }
    }
    fn draw_point<P: Into<IPoint>>(&self, p: P, is_there: bool) {
        let IPoint { x, y } = p.into();
        if x < self.cols as i32 && y < self.rows as i32 {
            self._draw_string_at(x + 1, y + 1, if is_there { "#" } else { " " });
        }
    }

    fn draw_segment(&self, seg: ISegment, is_there: bool) {
        for point in seg.get_line_points() {
            self.draw_point(point, is_there);
        }
    }

    fn _draw_string_at(&self, x: i32, y: i32, str: &str) {
        print!("{}{}", termion::cursor::Goto(x as u16, y as u16), str);
    }
}

fn frange(from: f32, to: f32, step: f32) -> impl Iterator<Item = f32> {
    let range = to - from;
    let steps = (range / step) as i64;
    assert!(steps >= 0);
    (0..steps).into_iter().map(move |x| x as f32 * step + to)
}

// fn function_to_curve<F: Fn(f32) -> f32>(f: F, from: i32, to: i32) -> impl Iterator<Item = IPoint> {
//     (from..=to).map(move |it| IPoint::new(it, f(it as f32) as i32))
// }

fn main() {
    // start_drawing(320);
    // draw_segment(ISegment::new(IPoint::new(4, 10), IPoint::new(10, 3)));
    let canvas = Canvas::new();
    let o = IPoint::new(10, 10);
    for s in frange(0.0, 90.0_f32.to_radians(), 5.0_f32.to_radians())
        .map(|a| ISegment::from_angle(o, a, BRUSH_WIDTH as f32))
    {
        std::thread::sleep(std::time::Duration::from_millis(50));
        canvas.draw_segment(s, true)
    }
}
