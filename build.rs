mod gen;

use gen::*;

#[macro_use] extern crate indexmap;
#[macro_use] extern crate handlebars;

fn main() -> anyhow::Result<()> {
    println!("cargo::rerun-if-changed=res");

    std::fs::create_dir_all("out")?;

    continent::generate()?;
    country::generate()?;
    currency::generate()?;
    language::generate()?;
    subdivision::generate()?;

    Ok(())
}