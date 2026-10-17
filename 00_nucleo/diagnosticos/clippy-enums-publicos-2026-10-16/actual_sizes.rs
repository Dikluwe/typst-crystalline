use typst_core::entities::{tag::Tag,element_info::ElementInfo,show::{Transformation,ShowRule},style::Styles};
fn main(){println!("{{\"tag\":{},\"element_info\":{},\"transformation\":{},\"show_rule\":{},\"styles\":{}}}",std::mem::size_of::<Tag>(),std::mem::size_of::<ElementInfo>(),std::mem::size_of::<Transformation>(),std::mem::size_of::<ShowRule>(),std::mem::size_of::<Styles>());}
