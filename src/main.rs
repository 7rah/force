use plotters::prelude::*;
use std::fmt::Display;
use structopt::StructOpt;

#[derive(Clone, Copy)]
struct Eval {
    M: f64,
    G: f64,
    t: f64,
    group: Group,
}

#[derive(Clone, Copy)]
struct Group {
    x: f64,
    vx: f64,

    y: f64,
    vy: f64,

    step: u64,
}

#[derive(StructOpt)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long)]
    step: u64,
    #[structopt(short, long)]
    G: f64,
    #[structopt(short, long)]
    M: f64,
    #[structopt(short, long)]
    x: f64,
    #[structopt(short, long)]
    y:f64,
    #[structopt(short, long)]
    vx:f64,
    #[structopt(short, long)]
    vy:f64,
    #[structopt(short, long)]
    t: f64,
    #[structopt(short, long)]
    buf: usize,
    #[structopt(short, long)]
    img_size: u32,
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "step:{} ({}, {})", self.step, self.x, self.y)
    }
}

impl Iterator for Eval {
    type Item = Group;

    fn next(&mut self) -> Option<Self::Item> {
        let Eval { G, M, t, group } = *self;
        let Group {
            mut x,
            mut y,
            mut vx,
            mut vy,
            mut step,
        } = group;

        x = x + vx * t;
        y = y + vy * t;
        let r = (x.powi(2) + y.powi(2)).sqrt();

        let ax = -(x / r.powi(3));
        let ay = -(y / r.powi(3));

        vx = vx + ax * t;
        vy = vy + ay * t;

        step += 1;

        self.group = Group { x, vx, y, vy, step };
        Some(Group { ..self.group })
    }
}

impl Eval {
    pub fn new(G: f64, M: f64, t: f64, x: f64, y: f64, vx: f64, vy: f64) -> Self {
        Self {
            M,
            G,
            t,
            group: Group {
                x,
                vx,
                y,
                vy,
                step: 0,
            },
        }
    }
}

fn max(x: f64, y: f64) -> f64 {
    if x > y {
        x
    } else {
        y
    }
}

fn min(x: f64, y: f64) -> f64 {
    if x < y {
        x
    } else {
        y
    }
}

fn get_coord(eval: Eval, step: u64) -> ((f64, f64), (f64, f64)) {
    let (mut max_x, mut max_y) = (0f64, 0f64);
    let (mut min_x, mut min_y) = (0f64, 0f64);
    for i in eval {
        max_x = max(max_x, i.x);
        max_y = max(max_y, i.y);
        min_x = min(min_x, i.x);
        min_y = min(min_y, i.y);

        if i.step == step {
            break;
        }
    }
    ((max_x, max_y), (min_x, min_y))
}

fn calc_size(img_size:u32,eval: Eval, step: u64) -> ((u32, u32), ((f64, f64), (f64, f64))) {
    let ((max_x, max_y), (min_x, min_y)) = get_coord(eval, step);
    let x = max_x - min_x;
    let y = max_y - min_y;
    let max = max(x, y);
    let width = ((img_size as f64) * (x / max)).ceil() as u32;
    let height = ((img_size as f64) * (y / max)).ceil() as u32;
    ((width, height), ((min_x, max_x), (min_y, max_y)))
}

fn main() {
    let opt = Opt::from_args();
    let eval = Eval::new(opt.G, opt.M, opt.t,opt.x, opt.y , opt.vx, opt.vy);
    

    let ((width, height), ((min_x, max_x), (min_y, max_y))) = calc_size(opt.img_size,eval, opt.step);
    let path = format!("output/STEP({})-t({})-x({},{})-v({},{})-G({})-M({}).png",opt.step, opt.t,opt.x, opt.y , opt.vx, opt.vy,opt.G, opt.M,);
    let root = BitMapBackend::new(&path, (width, height)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(0)
        .y_label_area_size(0)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)
        .unwrap();

    //chart.configure_mesh().draw().unwrap();
    chart
        .draw_series(PointSeries::of_element(
            vec![(0.0, 0.0)],
            8,
            &BLUE,
            &|c, s, st| return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
        ))
        .unwrap();

    let len = opt.buf;
    let mut buf = vec![(0f64, 0f64); len];
    for i in eval {
        let pos = (i.step % len as u64) as usize;
        buf[pos] = (i.x, i.y);
        if pos == 0 {
            chart
                .draw_series(PointSeries::of_element(
                    buf.clone(),
                    1,
                    &RED,
                    &|c, s, st| return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
                ))
                .unwrap();
        }

        if i.step > opt.step {
            break;
        }
    }
}
