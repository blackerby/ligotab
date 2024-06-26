use std::process;

#[derive(PartialEq)]
pub enum Format {
    Markdown,
    Org,
    Confluence,
}

impl From<String> for Format {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "markdown" => Self::Markdown,
            "org" => Self::Org,
            "confluence" => Self::Confluence,
            _ => {
                eprintln!("Unsupported output format: {value}");
                process::exit(3);
            }
        }
    }
}
