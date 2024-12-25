mod continent;
mod country;
mod currency;
mod language;
mod subdivision;

#[macro_use] extern crate indexmap;
#[macro_use] extern crate handlebars;

fn main() -> anyhow::Result<()> {
    println!("cargo::rerun-if-changed=csv");

    continent::generate()?;
    country::generate()?;
    currency::generate()?;
    language::generate()?;
    subdivision::generate()?;

    Ok(())
}