mod country;

#[macro_use] extern crate indexmap;

fn main() -> anyhow::Result<()> {
    println!("cargo::rerun-if-changed=datasets");

    country::generate()?;

    Ok(())
}