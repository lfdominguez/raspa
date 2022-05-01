const BRUSH_WIDTH: i32 = 6;
const DEFORMATION_FACTOR: i32 = 2;
const DELAY: i32 = 3;

const ANGLE_STEP: f32 = 5.0;
const RANGE_STEP: f32 = (std::f32::consts::PI / 180.0) * ANGLE_STEP;

const HALF_BRUSH_DEFORMED: i32 = (BRUSH_WIDTH * DEFORMATION_FACTOR) / 2;
const HALF_BRUSH_WIDTH: i32 = BRUSH_WIDTH / 2;

const VERTICAL_MARGIN: i32 = BRUSH_WIDTH / 2;
const HORIZONTAL_MARGIN: i32 = (BRUSH_WIDTH * DEFORMATION_FACTOR) / 2;

#[derive(Clone, Debug)]
struct FPoint {
    x: f32,
    y: f32,
    angle: f32,
}
impl FPoint {
    fn new(x: f32, y: f32, angle: f32) -> Self {
        Self { x, y, angle }
    }
}

#[derive(Clone, Debug, Copy)]
struct IPoint {
    x: i32,
    y: i32,
}

impl IPoint {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug, Copy)]
struct ISegment {
    from: IPoint,
    to: IPoint,
}

impl ISegment {
    fn new(from: IPoint, to: IPoint) -> Self {
        ISegment { from, to }
    }
}

fn frange(from: f32, to: f32, step: f32) -> impl Iterator<Item = f32> {
    let range = to - from;
    let steps = (range / step) as i64;
    assert!(steps >= 0);
    (0..steps).into_iter().map(move |x| x as f32 * step + to)
}

fn get_circle_fpoints(radius: f32, start: f32, end: f32) -> impl Iterator<Item = FPoint> {
    frange(start, end, RANGE_STEP).map(move |angle| FPoint {
        x: angle.cos() * radius * DEFORMATION_FACTOR as f32,
        y: angle.sin() * radius,
        angle,
    })
}

fn get_key_points(console_rows: i32, console_columns: i32) -> impl Iterator<Item = ISegment> {
    let iterations = (console_rows - (HALF_BRUSH_WIDTH * 3 - BRUSH_WIDTH)) / BRUSH_WIDTH;

    (0..iterations + 1).into_iter().flat_map(move |step| {
        [
            ISegment {
                from: IPoint {
                    x: HALF_BRUSH_DEFORMED * 2,
                    y: HALF_BRUSH_WIDTH * 2 + step * BRUSH_WIDTH,
                },
                to: IPoint {
                    x: console_columns - HALF_BRUSH_DEFORMED * 2,
                    y: HALF_BRUSH_WIDTH * 1 + step * BRUSH_WIDTH,
                },
            },
            ISegment {
                from: IPoint {
                    x: console_columns - HALF_BRUSH_DEFORMED * 2,
                    y: HALF_BRUSH_WIDTH * 3 + step * BRUSH_WIDTH,
                },
                to: IPoint {
                    x: HALF_BRUSH_DEFORMED * 2,
                    y: HALF_BRUSH_WIDTH * 2 + step * BRUSH_WIDTH,
                },
            },
        ]
    })
}

fn get_line_points(seg: ISegment) -> impl Iterator<Item = FPoint> {
    let y_step = (seg.to.y - seg.from.y) as f32 / ((seg.to.x - seg.from.x) as f32).abs();

    let x_direction = if seg.to.x > seg.from.x { 1.0 } else { -1.0 };
    let angle =
        -((seg.to.y as f32 - seg.from.x as f32) / (seg.from.y as f32 - seg.to.y as f32)).atan();

    (0..(seg.to.x - seg.from.x))
        .into_iter()
        .map(move |step| FPoint {
            x: (step as f32) * x_direction + seg.from.x as f32,
            y: (seg.from.y as f32) + y_step * (step as f32),
            angle,
        })
}

fn get_zig_zag_path(console_rows: i32, console_columns: i32) -> impl Iterator<Item = FPoint> {
    let circle_points_left: Vec<_> = get_circle_fpoints(
        HALF_BRUSH_WIDTH as f32,
        std::f32::consts::FRAC_PI_2,
        (std::f32::consts::PI * 3.0) / 2.0,
    )
    .collect();

    let circle_points_right: Vec<_> = get_circle_fpoints(
        HALF_BRUSH_WIDTH as f32,
        (std::f32::consts::PI * 3.0) / 2.0,
        (std::f32::consts::PI * 5.0) / 2.0,
    )
    .collect();

    let key_points: Vec<_> = get_key_points(console_rows, console_columns).collect();
    println!(
        "{:?}",
        get_line_points(ISegment::new(IPoint::new(0, 1), IPoint::new(10, 10))).collect::<Vec<_>>()
    );
    key_points
        .into_iter()
        .enumerate()
        .flat_map(move |(step, s)| {
            let line_points = get_line_points(s);

            let circle_points = if step % 2 == 0 {
                circle_points_right.clone()
            } else {
                circle_points_left.clone()
            };

            let turn_points = circle_points.into_iter().map(move |point| FPoint {
                x: (point.x + s.to.x as f32),
                y: (point.y + (s.to.y + BRUSH_WIDTH) as f32 / 2.0) as f32,
                angle: point.angle,
            });
            line_points.chain(turn_points)
        })
}

fn get_rectangular_path(
    closest_start_point: FPoint,
    console_rows: i32,
    console_columns: i32,
) -> impl Iterator<Item = FPoint> {
    let start_point = IPoint {
        x: closest_start_point.x as i32,
        y: (console_rows - VERTICAL_MARGIN),
    };

    let points = frange(start_point.x as f32, -2.0, -1.0).map(move |x| FPoint {
        x,
        y: console_rows as f32 - VERTICAL_MARGIN as f32 + 1.0,
        angle: std::f32::consts::FRAC_PI_2,
    });

    let mut angle_points: Vec<_> =
        get_circle_fpoints((BRUSH_WIDTH / 2) as f32, 0.0, std::f32::consts::FRAC_PI_2)
            .map(|FPoint { x, y, angle }| FPoint {
                x,
                y: y + (console_rows - VERTICAL_MARGIN * 2) as f32,
                angle,
            })
            .collect();
    angle_points.reverse();

    let vertical_points =
        frange((console_rows - VERTICAL_MARGIN - 3) as f32, -1.0, -1.0).flat_map(|y| {
            [
                FPoint {
                    x: HORIZONTAL_MARGIN as f32 - 1.0,
                    y,
                    angle: std::f32::consts::PI,
                },
                FPoint {
                    x: HORIZONTAL_MARGIN as f32 - 1.0,
                    y,
                    angle: std::f32::consts::PI,
                },
            ]
        });

    let circle_points_2: Vec<_> = get_circle_fpoints(
        (BRUSH_WIDTH / 2) as f32,
        std::f32::consts::PI,
        (std::f32::consts::PI * 3.0) / 2.0,
    )
    .collect();

    let angle_points_2 = circle_points_2
        .into_iter()
        .rev()
        .map(|FPoint { x, y, angle }| FPoint {
            x: x + (HORIZONTAL_MARGIN * 2) as f32,
            y,
            angle,
        });

    let more_points = frange((HORIZONTAL_MARGIN + 3) as f32, console_columns as f32, 1.0)
        .map(|x| FPoint::new(x, (VERTICAL_MARGIN - 1) as f32, std::f32::consts::FRAC_PI_2));

    let angle_points_3: Vec<_> = get_circle_fpoints(
        (BRUSH_WIDTH / 2) as f32,
        std::f32::consts::PI,
        (std::f32::consts::PI * 3.0) / 2.0,
    )
    .collect();

    let another_set_of_points: Vec<_> = angle_points_3
        .iter()
        .rev()
        .map(|point| {
            FPoint::new(
                point.x + console_columns as f32,
                point.y + (VERTICAL_MARGIN * 2) as f32,
                point.angle,
            )
        })
        .collect();

    let extra = frange((VERTICAL_MARGIN + 3) as f32, console_rows as f32, 1.0).flat_map(move |y| {
        [
            FPoint::new(
                (console_columns - HORIZONTAL_MARGIN) as f32,
                y,
                std::f32::consts::PI,
            ),
            FPoint::new(
                (console_columns - HORIZONTAL_MARGIN) as f32,
                y,
                std::f32::consts::PI,
            ),
        ]
    });

    let angle_points_4: Vec<_> = get_circle_fpoints(
        (BRUSH_WIDTH / 2) as f32,
        (std::f32::consts::PI * 3.0) / 2.0,
        (std::f32::consts::PI * 4.0) / 2.0,
    )
    .into_iter()
    .map(|point| {
        FPoint::new(
            point.x + (console_columns - HORIZONTAL_MARGIN * 2) as f32,
            point.y + console_rows as f32,
            point.angle,
        )
    })
    .collect();

    points
        .chain(angle_points)
        .chain(vertical_points)
        .chain(angle_points_2)
        .chain(more_points)
        .chain(another_set_of_points)
        .chain(extra)
        .chain(angle_points_4.into_iter().rev())
        .chain(
            frange(
                (console_columns - HORIZONTAL_MARGIN - 3) as f32,
                start_point.x as f32,
                -1.0,
            )
            .map(move |x| {
                FPoint::new(
                    x,
                    (console_rows - VERTICAL_MARGIN) as f32,
                    std::f32::consts::FRAC_PI_2,
                )
            }),
        )
}

fn get_brush_points(FPoint { x, y, angle }: FPoint) -> impl Iterator<Item = IPoint> {
    let opposite_angle = angle + std::f32::consts::PI;

    (0..HALF_BRUSH_WIDTH * DEFORMATION_FACTOR).flat_map(move |step| {
        [
            IPoint::new(
                (x + angle.cos()
                    * ((HALF_BRUSH_WIDTH / (HALF_BRUSH_WIDTH * DEFORMATION_FACTOR)) * step) as f32
                    * DEFORMATION_FACTOR as f32) as i32,
                (y + angle.sin()
                    * ((HALF_BRUSH_WIDTH / (HALF_BRUSH_WIDTH * DEFORMATION_FACTOR)) * step) as f32)
                    as i32,
            ),
            IPoint::new(
                (x + opposite_angle.cos()
                    * (((HALF_BRUSH_WIDTH / (HALF_BRUSH_WIDTH * DEFORMATION_FACTOR)) * step)
                        * DEFORMATION_FACTOR) as f32) as i32,
                (y + opposite_angle.sin()
                    * ((HALF_BRUSH_WIDTH / (HALF_BRUSH_WIDTH * DEFORMATION_FACTOR)) * step) as f32)
                    as i32,
            ),
        ]
    })
}

fn draw_string_at(x: i32, y: i32, str: &str) {
    print!("{}{}", termion::cursor::Goto(x as u16, y as u16), str);
}

fn draw_point(point: FPoint, period: i32, limit: IPoint) {
    let brush_points: Vec<_> = get_brush_points(point).collect();

    let IPoint {
        x: console_rows,
        y: console_columns,
    } = limit;

    std::thread::sleep(std::time::Duration::from_millis(period as u64));

    for IPoint { x, y } in brush_points.iter() {
        if x < &console_rows && y < &console_columns {
            draw_string_at(x + 1, y + 1, "#");
        }
    }

    // std::thread::sleep(std::time::Duration::from_millis((DELAY * period) as u64));
    //
    // for IPoint { x, y } in brush_points.iter() {
    //     if x < &console_rows && y < &console_columns {
    //         draw_string_at(x + 1, y + 1, " ");
    //     }
    // }
}

fn start_drawing(speed: i32) {
    let (console_rows, console_columns) = termion::terminal_size().unwrap();

    let console_limits = IPoint::new(console_rows.into(), console_columns.into());
    //
    let ms_per_frame = 1000 / speed;
    let path = get_zig_zag_path(console_rows.into(), console_columns.into());

    for point in path {
        draw_point(point, ms_per_frame, console_limits);
    }

    // let path = get_rectangular_path(
    //     FPoint::new(10.0, console_columns as f32 - 20.0, 0.0),
    //     console_rows as i32,
    //     console_columns as i32,
    // );
    //
    // for point in path {
    //     draw_point(point, ms_per_frame, console_limits);
    // }

    // draw_string_at(1, 1, "\x1Bc");
}

fn main() {
    start_drawing(320);
}
