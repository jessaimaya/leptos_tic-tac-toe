
pub enum HeadingSize {
    L,
    M,
    S,
    XS,
}

stylance::import_crate_style!(styles,  "./src/components/headings/headings.module.css");
pub fn heading_component(size: HeadingSize) -> &'static str {
   match size {
       HeadingSize::L => styles::large,
       HeadingSize::M =>  styles::medium,
       HeadingSize::S =>  styles::small,
       HeadingSize::XS => styles::xsmall,
   }
}