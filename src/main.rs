use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
};

use cubic_bezier::{
    bernstein::Bezier,
    handle::Handle,
};
use hypermelon::{self, prelude::Elem};
use poloto::*;
use vector2d::Vector2D;

fn main() {
    let mut bezier = Bezier::new(1000, 4);

    bezier.push_handle(Handle::new(
        Vector2D::new(-1.0, 0.0),
        Vector2D::new(0.0, 0.0),
        Vector2D::new(1.5, 0.5),
    ));
    bezier.push_handle(Handle::new(
        Vector2D::new(1.0, 3.0),
        Vector2D::new(3.0, 3.0),
        Vector2D::new(3.0, 1.0),
    ));
    bezier.push_handle(Handle::mirrored(
        Vector2D::new(5.0, 1.0),
        Vector2D::new(5.0, 0.0),
    ));
    bezier.push_handle(Handle::aligned(
        Vector2D::new(7.0, -2.5),
        Vector2D::new(8.0, 0.0),
        2.0,
    ));
    bezier.knot_insert(0.5);

    let scatter_points = bezier.all_part_point_dbg();

    let points = bezier.calculate();
    let _ = plot(&points, &scatter_points, String::from("output.svg"));
}

fn plot(
    points: &Vec<Vector2D<f64>>,
    scatter: &Vec<Vector2D<f64>>,
    filename: String,
) -> Result<(), Box<dyn Error>> {
    let cleartheme = poloto::render::Theme::light()
        .append(".poloto_background{fill:white;}")
        .append(".poloto_imgs.poloto_ticks{stroke:white;stroke-width:0px;}")
        .append(".poloto_text{fill: white;stroke-width:0px;}")
        .append(".poloto_scatter.poloto_plot{fill:black;stroke-width:3px;}")
        .append(".poloto_line.poloto_imgs.poloto_plot{stroke:black;stroke-width:2px}");

    let plots = poloto::plots!(
        build::plot("").line((0..points.len()).map(|x| { (points[x].x, points[x].y) })),
        build::plot("").scatter((0..scatter.len()).map(|x| { (scatter[x].x, scatter[x].y) }))
    );

    let plot = poloto::frame_build()
        .data(poloto::plots!(build::origin(), plots))
        .build_and_label(("", "", ""))
        .append_to(poloto::header().append(cleartheme))
        .render_string()?;

    fs::remove_file(&filename).unwrap_or(());
    let mut file = OpenOptions::new().write(true).create(true).open(filename)?;

    file.write_all(plot.as_bytes())?;

    Ok(())
}
