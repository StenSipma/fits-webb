use yew::prelude::*;
use gloo_file::File;
use fits::BasicFits;
use fits::parsing::header::Keyword;

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
                    <p> {"Axes: "} {h.axes} </p>
                </div>

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

fn view_keyword_table(kw: &Keyword) -> Html {
    match kw {
        Keyword::Value(k, v, _c) => html!(<tr><td>{k}</td> <td>{v}</td></tr>),
        Keyword::History(v) => html!(<tr><td>{"HISTORY"}</td> <td>{v}</td></tr>),
        Keyword::Comment(v) => html!(<tr><td>{"COMMENT"}</td> <td>{v}</td></tr>),
        Keyword::Continue(k, v, _c) => html!(<tr><td>{k}</td> <td>{v}</td></tr>),
    }

}
