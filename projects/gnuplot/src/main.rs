use gnuplot::{Figure, Caption, Color};
use gnuplot::Coordinate::*;
use gnuplot::PlotOption::*;
use gnuplot::ArrowheadType::Filled;

fn main() {

    let x = [0u32, 1, 2];
    let y = [3u32, 4, 5];
    let mut fg = Figure::new();
    let options = [ArrowType(Filled)];
    fg.axes2d()
        .lines(&x, &y, &[Caption("A line"), Color("black")])
        .arrow(Axis(0.0), Axis(4.0), Axis(2.0), Axis(4.0), &options)
        .points(&y, &x, &[Caption("Points"), Color("red")]);
    fg.show();
}
