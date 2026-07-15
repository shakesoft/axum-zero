use sailfish::TemplateSimple;

#[derive(TemplateSimple)]  // automatically implement `TemplateSimple` trait
#[template(path = "hello.stpl")]  // specify the path to template
pub struct HelloTemplate {
    // data to be passed to the template
    pub messages: Vec<String>,
}