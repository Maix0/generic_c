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
extern crate walkdir;

mod clap_mod;

use eyre::{Result, WrapErr};
use grep_regex::RegexMatcher;
use input_file::InputFile;
use regex::{Regex, RegexSet};
mod input_file;

use clap::Parser;
use std::collections::HashSet;
use std::fmt::Write;
use std::path::{Path, PathBuf};

fn open_input_file(p: impl AsRef<std::path::Path>) -> Result<input_file::InputFile> {
    toml::from_str::<input_file::InputFile>(
        &std::fs::read_to_string(p.as_ref())
            .wrap_err(eyre!("Couldn't open file {}", p.as_ref().display()))?,
    )
    .wrap_err(eyre!("File {} isn't valid toml", p.as_ref().display()))
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let args = clap_mod::Cli::parse();
    let mut data = open_input_file(args.input_file)?;
    check_all_def(&mut data)?;
    for (name, def) in &data.definition {
        apply_transformation(
            def,
            data.create.get(name).map_or(&[], std::vec::Vec::as_slice),
            &args.output_dir,
        )
        .wrap_err(eyre!("Transform with name '{name}' failed to apply"))?;
        println!("Apllied all transform for definition '{name}'");
    }
    Ok(())
}
fn check_all_def(data: &mut InputFile) -> Result<()> {
    {
        data.definition.iter_mut().try_for_each(|(k, v)| {
            check_definition(v).wrap_err(eyre!("Definition {k} is incorrect:"))
        })?;
        let mut out_err = String::new();
        for missing_def in data
            .create
            .keys()
            .filter(|n| data.definition.get(n.as_str()).is_none())
        {
            writeln!(&mut out_err, "Definition {missing_def} is missing")?;
        }
        if !out_err.is_empty() {
            out_err.pop();
            return Err(eyre!("{out_err}"));
        }
    }
    for (name, def) in &mut data.definition {
        check_transformations(
            def,
            data.create
                .get_mut(name)
                .map_or(&mut [], std::vec::Vec::as_mut_slice),
        )
        .wrap_err(eyre!(
            "Transform with name '{name}' doesn't have valid schema"
        ))?;
    }
    Ok(())
}

fn check_definition(def: &mut input_file::Definition) -> eyre::Result<()> {
    def.replace
        .insert("__TEMPLATE__".to_string(), input_file::Kind::Builtin);
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
    create: &mut [input_file::Create],
) -> eyre::Result<()> {
    let keys = def
        .replace
        .keys()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    create.iter_mut().try_for_each(|c| {
        c.replace.insert("__TEMPLATE__".to_string(), String::new());
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

fn regex_path(
    regex_set: &RegexSet,
    regexs: &[(&String, Regex)],
    path: &Path,
    c: &input_file::Create,
) -> Result<std::path::PathBuf> {
    let idx = regex_set
        .matches(path.to_str().ok_or(eyre!("out path isn't UTF-8"))?)
        .iter()
        .next();
    Ok(match idx {
        None => path.to_path_buf(),
        Some(i) => {
            let (name, regex) = &regexs[i];
            std::path::PathBuf::from(
                regex
                    .replace_all(
                        path.to_str().ok_or(eyre!("out path isn't UTF-8"))?,
                        c.replace[name.as_str()].as_str(),
                    )
                    .into_owned(),
            )
        }
    })
}

fn apply_transformation(
    def: &input_file::Definition,
    create: &[input_file::Create],
    out_path: impl AsRef<Path>,
) -> eyre::Result<()> {
    let out_path = out_path.as_ref();
    let regex_set = regex::RegexSet::new(def.replace.keys().map(|k| regex_syntax::escape(k)))
        .wrap_err("Error with regexes for a definition")?;
    let regexs = def
        .replace
        .keys()
        .map(|k| {
            (
                k,
                regex::Regex::new(regex_syntax::escape(k).as_str()).unwrap(),
            )
        })
        .collect::<Vec<_>>();
    for c in create {
        let mut out_source = out_path.join(
            regex_path(&regex_set, &regexs, &c.sources_output, c)
                .wrap_err("Error with the source regex for output")?,
        );
        let mut out_header = out_path.join(
            regex_path(&regex_set, &regexs, &c.headers_output, c)
                .wrap_err("Error with the header regex for output")?,
        );
        std::fs::create_dir_all(&out_source)
            .wrap_err("Error when creating the source directory")?;
        std::fs::create_dir_all(&out_header)
            .wrap_err("Error when creating the header directory")?;
        let header_files = def
            .headers
            .iter()
            .map(|p| regex_path(&regex_set, &regexs, p.as_path(), c).map(|r| (p, r)))
            .collect::<Result<Vec<(&PathBuf, PathBuf)>>>()?;
        let source_files = def
            .sources
            .iter()
            .map(|p| regex_path(&regex_set, &regexs, p.as_path(), c).map(|r| (p, r)))
            .collect::<Result<Vec<(&PathBuf, PathBuf)>>>()?;
        for (to_copy, out_path) in header_files {
            out_header.push(out_path.file_name().unwrap());
            std::fs::copy(to_copy, &out_header)?;
            out_header.pop();
        }
        for (to_copy, out_path) in source_files {
            out_source.push(out_path.file_name().unwrap());
            std::fs::copy(to_copy, &out_source)?;
            out_source.pop();
        }

        let out_paths = vec![out_header.to_str().unwrap(), out_source.to_str().unwrap()];

        regexs.iter().try_for_each(|(k, r)| {
            fastmod::Fastmod::run_fast(
                r,
                &RegexMatcher::new_line_matcher(r.as_str())?,
                c.replace[k.as_str()].as_str(),
                out_paths.clone(),
                None,
                false,
                false,
            )
            .map_err(|e| eyre!(e.to_string()))
        })?;
    }
    Ok(())
}
