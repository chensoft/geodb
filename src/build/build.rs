mod country;
mod dataset;

#[macro_use] extern crate indexmap;

fn main() -> anyhow::Result<()> {
    println!("cargo::rerun-if-changed=datasets");

    country::country_code()?;

    Ok(())
}