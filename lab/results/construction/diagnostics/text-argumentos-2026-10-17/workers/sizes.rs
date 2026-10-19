use std::mem::{align_of,size_of};
use typst_core::entities::{style::{Style,Styles},style_chain::StyleDelta,layout_types::Length};
fn main() {
 println!("{{\"length\":{},\"option_length\":{},\"option_f64\":{},\"style\":{},\"styles\":{},\"style_delta\":{},\"style_delta_alignment\":{}}}",size_of::<Length>(),size_of::<Option<Length>>(),size_of::<Option<f64>>(),size_of::<Style>(),size_of::<Styles>(),size_of::<StyleDelta>(),align_of::<StyleDelta>());
}
