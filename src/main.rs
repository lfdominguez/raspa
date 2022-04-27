mod float_iterator;

const BRUSH_WIDTH: f64 = 6.0;
const DEFORMATION_FACTOR: f64 = 2.0;
const DELAY: u32 = 3;

fn get_circle_fpoints(radius: f64, start: f64, end: f64) -> Vec<(f64, f64, f64)> {
    const angle_step: f64 = 5.0;
    const range_step: f64 = (std::f64::consts::PI * 2.0 / 360.0) * angle_step;

    let mut points: Vec<(f64, f64, f64)> = Vec::new();

    for angle in float_iterator::FloatIterator::new_with_step(start, end, range_step)
    {
        points.push((
            angle.cos() * radius * DEFORMATION_FACTOR,
            angle.sin() * radius,
            angle
        ));
    }

    return points;
}

fn get_key_points(console_rows: i32, console_columns: i32) -> Vec<Vec<(f64, f64)>> {
    const half_brush_deformed: f64 = (BRUSH_WIDTH * DEFORMATION_FACTOR) / 2.0;

    let mut points: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut step: f64 = 0.0;

    while ((BRUSH_WIDTH / 2.0) * 3.0 + (step - 1.0) * BRUSH_WIDTH) < console_rows as f64{
        points.push(vec![
            (
                half_brush_deformed * 2.0,
                (BRUSH_WIDTH / 2.0) * 2.0 + step * BRUSH_WIDTH
            ),
            (
                console_columns as f64 - half_brush_deformed * 2.0,
                (BRUSH_WIDTH / 2.0) * 1.0 + step * BRUSH_WIDTH
            )
        ]);
        points.push(vec![
            (
                console_columns as f64 - half_brush_deformed * 2.0,
                (BRUSH_WIDTH / 2.0) * 3.0 + step * BRUSH_WIDTH
            ),
            (
                half_brush_deformed * 2.0,
                (BRUSH_WIDTH / 2.0) * 2.0 + step * BRUSH_WIDTH
            )
        ]);

        step += 1.0;
    }

    return points;
}

fn get_line_points(start_x: f64, start_y: f64, end_x: f64, end_y: f64) -> Vec<(f64, f64, f64)>{
    let mut points: Vec<(f64, f64, f64)> = Vec::new();

    let y_step = (end_y - start_y) / (end_x - start_x).abs();
    let x_direction = if end_x > start_x { 1 } else { -1 };
    let angle = -((end_y - start_x) / start_y - end_y).atan();

    for step in 0..((end_x - start_x).abs() as i64) {
        points.push((
            step as f64 * x_direction as f64 + start_x,
            start_y + y_step * (step as f64),
            angle
        ));
    }

    return points;
}

fn get_zig_zag_path(console_rows: i32, console_columns: i32) -> Vec<(f64, f64, f64)>{
    let mut circle_points_left = get_circle_fpoints(
        ((BRUSH_WIDTH / 2.0) as f64).floor(),
        std::f64::consts::FRAC_PI_2,
        (std::f64::consts::PI * 3.0) / 2.0,
    );

    let circle_points_right = get_circle_fpoints(
        ((BRUSH_WIDTH / 2.0) as f64).floor(),
        (std::f64::consts::PI * 3.0) / 2.0,
        (std::f64::consts::PI * 5.0) / 2.0,
    );

    circle_points_left.reverse();

    let key_points = get_key_points(console_rows, console_columns);

    let mut points: Vec<(f64, f64, f64)> = Vec::new();

    for step in 0..key_points.len() {
        let mut line_points = get_line_points(key_points[step][0].0, key_points[step][0].1, key_points[step][1].0, key_points[step][1].1);

        let mut turn_points = if step % 2 == 0 {
            circle_points_right.clone()
        } else {
            circle_points_left.clone()
        }.iter().map(|point| {
            (
                point.0 + key_points[step][1].0,
                point.1 + key_points[step][1].1 + BRUSH_WIDTH / 2.0,
                point.2
            )
        }).collect();

        points.append(&mut points.clone());
        points.append(&mut line_points);
        points.append(&mut turn_points);
    }

    return points;

}

fn get_rectangular_path(closest_start_point: (f64, f64, f64), console_rows: i32, console_columns: i32) -> Vec<(f64, f64, f64)> {
    let mut points: Vec<(f64, f64, f64)> = Vec::new();

    const vertical_margin: f64 = BRUSH_WIDTH / 2.0;
    const horizontal_margin: f64 = (BRUSH_WIDTH * DEFORMATION_FACTOR) / 2.0;

    let start_point = (closest_start_point.0, console_rows - vertical_margin as i32);
    
    for x in float_iterator::FloatIterator::new_with_step(start_point.0, -2.0, -1.0) {
        points.push((
            x,
            console_rows as f64 - vertical_margin + 1.0,
            std::f64::consts::FRAC_PI_2
        ));
    }

    let mut angle_points: Vec<(f64, f64, f64)> = get_circle_fpoints((BRUSH_WIDTH / 2.0).floor(), 0.0, std::f64::consts::FRAC_PI_2)
        .iter().map(|point| {
            (
                point.0,
                point.1 + console_rows as f64 - vertical_margin * 2.0,
                point.2
            )
        }).collect();
    angle_points.reverse();

    points.append(&mut angle_points);

    for y in float_iterator::FloatIterator::new_with_step(console_rows as f64 - vertical_margin - 3.0, -1.0, -1.0) {
        points.push((
            horizontal_margin - 1.0,
            y,
            std::f64::consts::PI
        ));
        points.push((
            horizontal_margin - 1.0,
            y,
            std::f64::consts::PI
        ));
    }

    let mut angle_points_2 = get_circle_fpoints ((BRUSH_WIDTH / 2.0).floor(), std::f64::consts::PI, (std::f64::consts::PI * 3.0) / 2.0);
    angle_points_2.reverse();

    let mut angle_points_2: Vec<(f64, f64, f64)> = angle_points_2.iter().map(|point| {
        (
            point.0 + horizontal_margin * 2.0,
            point.1,
            point.2
        )
    }).collect();

    points.append(&mut angle_points_2);

    for x in float_iterator::FloatIterator::new_with_step(horizontal_margin + 3.0, console_columns as f64, 1.0) {
        points.push((
            x,
            vertical_margin - 1.0,
            std::f64::consts::FRAC_PI_2
        ));
    }

    let mut angle_points_3 = get_circle_fpoints ((BRUSH_WIDTH / 2.0).floor(), std::f64::consts::PI, (std::f64::consts::PI * 3.0) / 2.0);
    angle_points_3.reverse();
    let mut angle_points_3: Vec<(f64, f64, f64)> = angle_points_3.iter().map(|point| {
        (
            point.0 + console_columns as f64,
            point.1 + vertical_margin * 2.0,
            point.2
        )
    }).collect();

    points.append(&mut angle_points_3);

    for y in float_iterator::FloatIterator::new_with_step(vertical_margin + 3.0, console_rows as f64, 1.0) {
        points.push((
            console_columns as f64 - horizontal_margin,
            y,
            std::f64::consts::PI
        ));
        points.push((
            console_columns as f64 - horizontal_margin,
            y,
            std::f64::consts::PI
        ));
    }

    let mut angle_points_4: Vec<(f64, f64, f64)> = get_circle_fpoints ((BRUSH_WIDTH / 2.0).floor(), (std::f64::consts::PI * 3.0) / 2.0, (std::f64::consts::PI * 4.0) / 2.0).iter().map(|point| {
        (
            point.0 + console_columns as f64 - horizontal_margin * 2.0,
            point.1 + console_rows as f64,
            point.2
        )
    }).collect();
    angle_points_4.reverse();

    points.append(&mut angle_points_4);

    for x in float_iterator::FloatIterator::new_with_step(console_columns as f64 - horizontal_margin - 3.0, start_point.0, -1.0) {
        points.push((
            x,
            console_rows as f64 - vertical_margin,
            std::f64::consts::FRAC_PI_2
        ));
    }

    return points;
}

fn get_brush_points(x: f64, y: f64, angle: f64) -> Vec<(f64, f64)> {
    let mut new_x: f64;
    let mut new_y: f64;

    let mut points: Vec<(f64, f64)> = Vec::new();

    const half_brush_width: f64 = BRUSH_WIDTH / 2.0;
    let opposite_angle = angle + std::f64::consts::PI;

    for step in float_iterator::FloatIterator::new_with_step(0.0, half_brush_width * DEFORMATION_FACTOR, 1.0) {
        new_x = x + angle.cos() * ((half_brush_width / (half_brush_width * DEFORMATION_FACTOR)) * step) * DEFORMATION_FACTOR;
        new_y = y + angle.sin() * ((half_brush_width / (half_brush_width * DEFORMATION_FACTOR)) * step);

        points.push((new_x, new_y));

        new_x = x + opposite_angle.cos() * ((half_brush_width / (half_brush_width * DEFORMATION_FACTOR)) * step) * DEFORMATION_FACTOR;
        new_y = y + opposite_angle.sin() * ((half_brush_width / (half_brush_width * DEFORMATION_FACTOR)) * step);

        points.push((new_x, new_y));
    }

    return points;
}

fn draw_string_at(x: u16, y: u16, str: &str) {
    print!("{}{}", termion::cursor::Goto(x, y), str);
}

fn start_drawing(speed: u32) {
    let consola_size = termion::terminal_size().unwrap();

    let console_rows = consola_size.1 as u16;
    let console_columns = consola_size.0 as u16;

    let ms_per_frame: u32 = 1000 / speed;
    let zig_zag_path = get_zig_zag_path(console_rows.into(), console_columns.into());
    let mut rectangular_path = get_rectangular_path(*zig_zag_path.last().unwrap(), consola_size.0.into(), consola_size.1.into());

    let mut final_path = zig_zag_path.clone();
    final_path.append(&mut rectangular_path);

    for index in 0..final_path.len() {
        let point = final_path[index];

        let brush_points = get_brush_points(point.0, point.1, point.2);

        std::thread::sleep(std::time::Duration::from_millis((index as u32 * ms_per_frame).into()));
        for point in brush_points.iter() {
            if point.1 < console_rows as f64 && point.0 < console_columns as f64 {
                draw_string_at(point.0 as u16 + 1, point.1 as u16 + 1, "#");
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(((index as u32 + DELAY) * ms_per_frame).into()));
        for point in brush_points.iter() {
            if point.1 < console_rows as f64 && point.0 < console_columns as f64 {
                draw_string_at(point.0 as u16, point.1 as u16, " ");
            }
        }
    }

    draw_string_at(1, 1, "\x1Bc");
}

fn main() {
    start_drawing(320);
}
