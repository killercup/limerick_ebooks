extern crate handlebars;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_json as json;
extern crate comrak;

use std::fs::File;
use std::io::Read;

use comrak::{ComrakOptions, markdown_to_html};
use handlebars::Handlebars;

quick_main!(|| -> Result<()> {
    let source_path = "./limericks-20170503.md";
    let template_path = "./template.hbs";
    let output_path = "dist/index.rust.html";

    let limericks = read(source_path)?;
    let limericks: Vec<String> = limericks.split("\n---\n").map(md).collect();

    let data = json!({
        "title": "Rust Limericks",
        "author": "Andre 'lloqig' Bogus",
        "limericks": limericks,
    });

    let handlebars = Handlebars::new();
    let mut template = open(template_path)?;
    let mut output_file = create(output_path)?;
    handlebars.template_renderw2(&mut template, &data, &mut output_file)?;

    Ok(())
});

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Handlebars(::handlebars::TemplateRenderError);
    }
}

fn open(path: &str) -> Result<File> {
    File::open(path).chain_err(|| format!("Can't open `{}`", path))
}

fn create(path: &str) -> Result<File> {
    File::create(path).chain_err(|| format!("Can't create `{}`", path))
}

fn read(path: &str) -> Result<String> {
    let mut result = String::new();
    let mut file = open(path)?;
    file.read_to_string(&mut result)?;
    Ok(result)
}

fn md(input: &str) -> String {
    let mut md_options = ComrakOptions::default();
    md_options.hardbreaks = true;

    markdown_to_html(input.trim(), &md_options)
}
