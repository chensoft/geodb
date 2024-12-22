mod continent;
mod country;
mod currency;
mod language;

#[macro_use] extern crate indexmap;
#[macro_use] extern crate handlebars;

fn main() -> anyhow::Result<()> {
    println!("cargo::rerun-if-changed=datasets");

    continent::generate()?;
    country::generate()?;
    currency::generate()?;
    language::generate()?;

    Ok(())
}