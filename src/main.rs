use std::{
    collections::HashMap,
    io::Write,
    path::{Path, PathBuf},
};

use clap::Parser;
use tera::{from_value, to_value, Function, Result, Tera, Value};

#[derive(Parser, Debug)]
#[clap(author, about, version)]
struct Cli {
    /// A bash string of a glob, should be single quoted to avoid premature splitting
    files_glob: String,
    /// Custom flags to pass to pandoc
    #[clap(short, long)]
    custom_pandoc_flags: Vec<String>,
    /// Output to PDF instead of stdout
    #[clap(short, long, default_value = "false")]
    output_pdf: bool, 
    /// Search for import files relative to this directory
    /// If not specified, will search relative to the current directory
    #[clap(short, long)]
    root_dir: Option<PathBuf>
}

fn convert_to_pdf(args: &Cli, content: &str) {
    let mut binding = std::process::Command::new("pandoc");
    let mut cmd_builder = binding
        .arg("-f")
        .arg("markdown")
        .arg("-o")
        .arg("output.pdf");

    for flag in &args.custom_pandoc_flags {
        cmd_builder = cmd_builder.arg(flag);
    }
    
    let mut child = cmd_builder
        .stdin(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();

    if !output.status.success() {
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("Success!");
    }
}


/// Get the depth of a path, i.e. the number of path components
fn get_path_depth(path: &Path) -> usize {
    path.iter().count()
}

/// Sort by path depth (shortest path first), then break by alphabetical order
fn sort_paths_by_depth(paths: &mut [PathBuf]) {
    paths.sort_by(|a, b| {
        let a_depth = get_path_depth(a);
        let b_depth = get_path_depth(b);
        if a_depth == b_depth {
            a.cmp(b)
        } else {
            a_depth.cmp(&b_depth)
        }
    });
}

/// Given the name to a file, search recursively within a root directory using globs to find it.
///  If there is only one file with that name inside that directory, return it. Otherwiwse, error.
fn find_file_by_name(root_dir: &Path, file_name: &str) -> Result<PathBuf> {
    let mut found_files = Vec::new();
    for entry in glob::glob(root_dir.join("**").join(file_name).to_str().unwrap()).unwrap() {
        let path = entry.unwrap();
        found_files.push(path);
    }

    match found_files.len() {
        1 => Ok(found_files[0].clone()),
        0 => Err(format!("Found zero files with name {file_name} below {root_dir:?}").into()),
        x => Err(format!("Found {x} files with name {file_name} below {root_dir:?}").into()),
    }
}


/// A Tera function that embeds the contents of a file into the template
fn embed_file(root_dir: PathBuf) -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
        match args.get("path") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(file_name) => {
                    let file_path = find_file_by_name(&root_dir, &file_name)?;
                    let content = std::fs::read_to_string(file_path)?;
                    Ok(to_value(content)?)
                }
                Err(_) => Err("oops".into()),
            },
            None => Err("oops".into()),
        }
    })
}

fn main() {
    let args = Cli::parse();

    let mut tera = Tera::default();

    let ref_dir = if let Some(root_dir) = &args.root_dir {
        root_dir.to_owned()
    } else {
        std::env::current_dir().unwrap()
    };

    tera.register_function("embed_file", embed_file(ref_dir));

    let mut file_names: Vec<_> = glob::glob(&args.files_glob)
        .unwrap()
        .map(|file| {
            let path = file.unwrap();
            let path_str = path.to_str().unwrap();
            tera.add_template_file(path_str, Some(path_str)).unwrap();
            path
        })
        .collect();

    sort_paths_by_depth(&mut file_names);

    // Now, render each file and concat their contents

    let content = file_names.into_iter().map(|file_name| {
        let rendered = tera
            .render(file_name.to_str().unwrap(), &tera::Context::new())
            .unwrap();
        rendered
    }).collect::<Vec<_>>().join("\n\n * * * \n\n");

    // Finally, use pandoc to convert the file to PDF

    if args.output_pdf {
        convert_to_pdf(&args, &content);
    } else {
        println!("{}", content);
    }
}
