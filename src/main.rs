extern crate handlebars;
extern crate structopt;
#[macro_use] extern crate structopt_derive;
#[macro_use] extern crate error_chain;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json as json;
extern crate serde_yaml as yaml;
extern crate comrak;

use std::fs::File;
use std::io::Read;

use structopt::StructOpt;
use comrak::{ComrakOptions, markdown_to_html};
use handlebars::Handlebars;

/// Display limericks gloriously!
#[derive(StructOpt, Debug)]
#[structopt(name = "limerick-render", author = "Idea and limericking by @llogiq, coding by @killercup.")]
struct Cli {
    /// Limerick sauce (markdown snippets divided by `---`)
    #[structopt(long = "limericks", default_value = "./limericks-20170503.md")]
    source_path: String,
    /// Your awesome template (with the right incantations)
    #[structopt(long = "template", default_value = "./template.hbs")]
    template_path: String,
    /// Where to put gloriously rendered limerick awesomesauce
    #[structopt(long = "output", default_value = "./dist/index.rust.html")]
    output_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Metadata {
    title: String,
    author: String,
}

quick_main!(|| -> Result<()> {
    let cli = Cli::from_args();

    let limerick_data = read(&cli.source_path)?;
    let mut limericks = limerick_data.split("\n---\n").skip_while(|x| x.is_empty());

    let header = limericks.next()
        .chain_err(|| "No limericks. Not even a header. :(")?;
    let header: Metadata = yaml::from_str(header)
        .chain_err(|| "Error parsing metadata header. \
            Make sure your file starts with a valid YAML block.")?;

    let limericks: Vec<String> = limericks.map(md).collect();

    let data = json!({
        "metadata": header,
        "limericks": limericks,
    });

    let handlebars = Handlebars::new();
    let mut template = open(&cli.template_path)?;
    let mut output_file = create(&cli.output_path)?;
    handlebars.template_renderw2(&mut template, &data, &mut output_file)?;

    Ok(())
});

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Handlebars(::handlebars::TemplateRenderError);
        Yaml(::yaml::Error);
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
