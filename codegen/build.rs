mod continent;
mod country;
mod currency;

#[macro_use] extern crate indexmap;

fn main() -> anyhow::Result<()> {
    println!("cargo::rerun-if-changed=datasets");

    continent::generate()?;
    country::generate()?;
    currency::generate()?;

    Ok(())
}