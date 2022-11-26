use plotters::prelude::{IntoDrawingArea, ChartBuilder, Rectangle};
use plotters::style::{RGBColor, Color, HSLColor, BLACK};
use tensor::Tensor;
use web_sys::console;
use yew::prelude::*;
use gloo_file::File;
use fits::BasicFits;
use fits::parsing::header::Keyword;
use plotters_canvas::CanvasBackend;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub file: File,
    pub data: Vec<u8>,
}

#[function_component(FitsElement)]
pub fn fits_element(props: &Props) -> Html {
    let Props{ file, data } = props.clone();

    if let Some(fits) = BasicFits::from_bytes(data) {
        let h = fits.header;

        // TODO: We are cloning a lot here, maybe find a better solution.
        let dataclone = fits.data.clone();
        let shape = h.axes.clone();
        let onclick = Callback::from(move |_| {
            plot_data("data_canvas", dataclone.clone(), shape.clone()).unwrap();
        });
        html! {
            <>
                <h2 class="delim">{"General Info"}</h2>
                <div class="file_info">
                    <h3>{file.name()}</h3>
                    <h4> {"Filetype: "} {file.raw_mime_type()} {",  Size: "} {file.size()} {" Bytes"}</h4>
                </div>

                <div>
                    <p> {"Bitpix: "} {format!("{:?}", h.bitpix)} </p>
                    <p> {"Simple: "} {h.simple} </p>
                    <p> {"Naxis: "} {h.naxis} </p>
                    <p> {"Axes: "} {h.axes.clone()} </p>
                </div>

                <h2 class="delim">{"Data"}</h2>
                if fits.data.len() > 0 && h.naxis == 2 {
                    <div><button {onclick}>{"Draw"}</button></div>
                    <canvas id="data_canvas" width={h.axes[0].to_string()} height={h.axes[1].to_string()} ></canvas>
                } else {
                    <p>{"No data to display"}</p>
                }

                <h2 class="delim">{"Keywords"}</h2>
                <table class="keywords">
                    <tr> <th>{"Keyword"}</th> <th>{"Value"}</th> </tr>
                    {for h.keywords.iter().map(view_keyword_table)}
                </table>

            </>
        }
    } else {
        html! {
            <>
                <h3>{file.name()}</h3>
                <h4> {"Filetype: "} {file.raw_mime_type()} {",  Size: "} {file.size()} {" Bytes"}</h4>
                <p>{"Not a valid FITS file, so not reading..."}</p>
            </>
        }
    }
}

// fn plot_data(canvas_id: &str, data: Tensor<f64>, shape: Vec<usize>) -> Result<impl Fn((i32, i32)) -> Option<(f32, f32)>, Box<dyn std::error::Error>> {
fn plot_data(canvas_id: &str, data: Tensor<f64>, shape: Vec<usize>) -> Result<(), Box<dyn std::error::Error>> {

    let data = data.log10();
    let max = data.iter().fold(-1./0., |acc, x| {f64::max(acc,*x)});
    let min = data.iter().fold(1./0., |acc, x| {f64::min(acc,*x)});
    console::log_1(&format!("{} {}", min, max).into());

    let canvas = CanvasBackend::new(canvas_id).expect("Cannot find canvas");
    let root = canvas.into_drawing_area();
    root.fill(&BLACK)?;

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0..shape[0], 0..shape[1])?;

    chart.configure_mesh().draw()?;

    // Scale data to 0-1, as a greyscale value.
    let scaled_data = (data - min) / (max - min);
    let max = scaled_data.iter().fold(-1./0., |acc, x| {f64::max(acc,*x)});
    let min = scaled_data.iter().fold(1./0., |acc, x| {f64::min(acc,*x)});
    console::log_1(&format!("{} {}", min, max).into());

    chart.draw_series(
            scaled_data.iter_2d(shape.clone())
            .map(|((x, y), t)| {
                Rectangle::new( [(x, y), (x+1, y+1)], HSLColor(0.0, 0.0, *t).filled())
            })
            )?;
    root.present()?;

    // Ok(chart.into_coord_trans())
    Ok(())
}

fn view_keyword_table(kw: &Keyword) -> Html {
    match kw {
        Keyword::Value(k, v, c) => html!(<tr><td>{k}</td> <td title={c.to_string()}>{v}</td></tr>),
        Keyword::History(v) => html!(<tr><td>{"HISTORY"}</td> <td>{v}</td></tr>),
        Keyword::Comment(v) => html!(<tr><td>{"COMMENT"}</td> <td>{v}</td></tr>),
        Keyword::Continue(k, v, c) => html!(<tr><td>{k}</td> <td title={c.to_string()}>{v}</td></tr>),
    }

}
