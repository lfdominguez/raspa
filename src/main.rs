use std::io::{self, Write};

const BRUSH_WIDTH: u16 = 8;
#[derive(Debug, PartialEq, Clone, Copy)]
struct IPoint {
    x: i32,
    y: i32,
}

impl IPoint {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &IPoint) -> f32 {
        let IPoint { x, y } = *other - *self;
        ((x * x + y * y) as f32).sqrt()
    }
}
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
        let length = self.to.dist(&self.from);

        let dx = x as f32 / length;
        let dy = y as f32 / length;
        dedupe(
            (0..length as i32 + 1).map(move |i| {
                self.from + IPoint::new((i as f32 * dx) as i32, (i as f32 * dy) as i32)
            }),
        )
    }

    fn from_angle(origin: IPoint, angle: f32, length: f32) -> Self {
        let x = angle.cos() * length;
        let y = angle.sin() * length;
        Self::new(origin, origin + IPoint::new(x as i32, y as i32))
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
            Some((false, r))
        };

        *l = Some(r);
        result
    })
    .filter_map(|(same, r)| if same { None } else { Some(r) })
}

impl Canvas {
    fn new() -> Self {
        let (cols, rows) = termion::terminal_size().unwrap();
        Canvas { rows, cols }
    }
    fn draw_point<P: Into<IPoint>>(&self, p: P, is_there: bool) {
        let IPoint { x, y } = p.into();
        self._draw_string_at(x + 1, y + 1, if is_there { "#" } else { " " });
    }

    fn draw_segment(&self, seg: &ISegment, is_there: bool) {
        for point in seg.get_line_points() {
            self.draw_point(point, is_there);
        }
        io::stdout().flush().unwrap();
    }

    fn _draw_string_at(&self, x: i32, y: i32, str: &str) {
        print!("{}{}", termion::cursor::Goto(x as u16, y as u16), str);
    }
}

fn frange(from: f32, to: f32, step: f32) -> impl Iterator<Item = f32> {
    let range = to - from;
    let steps = (range / step) as i32;
    assert!(steps >= 0);
    (0..=steps.abs()).map(move |it| it as f32 * step + from)
}

fn function_to_curve<F: Fn(f32) -> f32>(f: F, from: f32, to: f32) -> impl Iterator<Item = IPoint> {
    frange(from, to, (to - from).signum()).map(move |it| IPoint::new(it as i32, f(it) as i32))
}

fn main() {
    let canvas = Canvas::new();
    // let o = IPoint::new(0, BRUSH_WIDTH);
    let curve = (0..canvas.rows - BRUSH_WIDTH as u16)
        .step_by((BRUSH_WIDTH - BRUSH_WIDTH % 2 - 2) as usize)
        .flat_map(|it| {
            let (l, r) = if it % 2 == 0 {
                (0, canvas.cols - 2 * BRUSH_WIDTH - 1)
            } else {
                (canvas.cols - 1, 2 * BRUSH_WIDTH)
            };

            function_to_curve(
                move |i| (i / 80.0).sin() * i / 20.0 + it as f32,
                l.into(),
                r.into(),
            )
        })
        .map(|p| (p, 90.0_f32))
        .chain(
            [
                (
                    ISegment::new(IPoint::new(0, canvas.rows.into()), IPoint::new(0, 0)),
                    0.0,
                ),
                (
                    ISegment::new(IPoint::new(0, 0), IPoint::new(canvas.cols.into(), 0)),
                    90.0,
                ),
                (
                    ISegment::new(
                        IPoint::new(canvas.cols.into(), 0),
                        IPoint::new(canvas.cols.into(), canvas.rows.into()),
                    ),
                    -180.0,
                ),
                (
                    ISegment::new(
                        IPoint::new(canvas.cols.into(), canvas.rows.into()),
                        IPoint::new(0, canvas.rows.into()),
                    ),
                    -90.0,
                ),
            ]
            .into_iter()
            .flat_map(|(s, a)| s.get_line_points().map(move |p| (p, a))),
        )
        .map(|(p, a)| ISegment::from_angle(p, a.to_radians(), BRUSH_WIDTH as f32));
    for s in dedupe(curve) {
        canvas.draw_segment(&s, true);
        std::thread::sleep(std::time::Duration::from_millis(1000 / 90));
        canvas.draw_segment(&s, false);
    }
}
