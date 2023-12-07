#[macro_use]
extern crate serde;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate paste;
extern crate color_eyre;
#[macro_use]
extern crate eyre;

extern crate toml;

use eyre::{Result, WrapErr};
use input_file::InputFile;
mod input_file;

use std::collections::HashSet;
use std::fmt::Write;

fn open_input_file(p: impl AsRef<std::path::Path>) -> Result<input_file::InputFile> {
    toml::from_str::<input_file::InputFile>(
        &std::fs::read_to_string(p.as_ref())
            .wrap_err(eyre!("Couldn't open file {}", p.as_ref().display()))?,
    )
    .wrap_err(eyre!("File {} isn't valid toml", p.as_ref().display()))
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let data = open_input_file("./input.toml")?;
    check_all_def(&data)?;

    println!("{data:?}");
    Ok(())
}
fn check_all_def(data: &InputFile) -> Result<()> {
    {
        data.definition.iter().try_for_each(|(k, v)| {
            check_definition(v).wrap_err(eyre!("Definition {k} is incorrect:"))
        })?;
        let mut out_err = String::new();
        for missing_def in data
            .create
            .keys()
            .filter(|n| data.definition.get(n.as_str()).is_none())
        {
            writeln!(&mut out_err, "Definition {} is missing", missing_def)?;
        }
        if !out_err.is_empty() {
            out_err.pop();
            return Err(eyre!("{out_err}"));
        }
    }
    for (name, def) in &data.definition {
        check_transformations(
            def,
            data.create.get(name).map(|v| v.as_slice()).unwrap_or(&[]),
        )
        .wrap_err(eyre!(
            "Transform with name '{name}' doesn't have valid schema"
        ))?;
    }
    for (name, def) in &data.definition {
        apply_transformation(
            def,
            data.create.get(name).map(|v| v.as_slice()).unwrap_or(&[]),
        )
        .wrap_err(eyre!("Transform with name '{name}' failed to apply"))?;
    }
    Ok(())
}

fn check_definition(def: &input_file::Definition) -> eyre::Result<()> {
    let missing_files = def
        .sources
        .iter()
        .chain(def.headers.iter())
        .filter(|f| !f.exists())
        .collect::<Vec<_>>();
    if !missing_files.is_empty() {
        let mut s = String::with_capacity(128);
        for err in missing_files {
            writeln!(&mut s, "File {} is missing", err.display())?;
        }
        s.pop();
        return Err(eyre!("{s}"));
    }
    Ok(())
}
fn check_transformations(
    def: &input_file::Definition,
    create: &[input_file::Create],
) -> eyre::Result<()> {
    let keys = def
        .replace
        .keys()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    create.iter().try_for_each(|c| {
        let c_keys = c.replace.keys().map(String::as_str).collect::<HashSet<_>>();
        if c_keys != keys {
            if c_keys.is_subset(&keys) {
                let mut s = String::from("Keys '");
                for v in keys.difference(&c_keys) {
                    write!(&mut s, "{v}, ")?;
                }
                s.pop();
                s.pop();
                write!(
                    &mut s,
                    "' are missing from the replace section in the transformation"
                )?;
                return Err(eyre!("{s}"));
            } else if c_keys.is_superset(&keys) {
                let mut s = String::from("Keys '");
                for v in c_keys.difference(&keys) {
                    write!(&mut s, "{v}, ")?;
                }
                s.pop();
                s.pop();
                write!(&mut s, "' are missing from the definition")?;
                return Err(eyre!("{s}"));
            }
        }
        Ok(())
    })?;

    Ok(())
}
fn apply_transformation(
    def: &input_file::Definition,
    create: &[input_file::Create],
) -> eyre::Result<()> {
    let regexs = def
        .replace
        .keys()
        .map(|k| (k, regex::Regex::new(regex_syntax::escape(&k).as_str())))
        .collect::<Vec<_>>();
    let regex_set =
        regex::RegexSet::new(def.replace.keys().map(|k| regex_syntax::escape(k)));
    for c in create {
        let out_source = {c.headers_output.display();}; //.iter()//.map(|p| p);
    }
    Ok(())
}
