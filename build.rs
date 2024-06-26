use std::io::Write;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=lib");

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let mut file = std::fs::File::create(format!("{out_dir}/mods.rs")).unwrap();

    let mut problems = vec![];

    for entry in walkdir::WalkDir::new("src").into_iter().flatten() {
        let name = entry.file_name().to_string_lossy();
        if name.starts_with('p') {
            let number = name.split(".rs").next().context("Invalid file name")?;

            let path = entry
                .path()
                .canonicalize()
                .context("Invalid path")?
                .to_str()
                .context("Invalid path")?
                .to_string();

            let module = quote::format_ident!("{}", number);

            let code = quote::quote! {
                #[path = #path]
                mod #module;
            }
            .to_string();

            file.write_all(code.as_bytes())?;

            problems.push(number.to_string());
        }
    }

    problems.sort();

    let len = problems.len();
    let problems = problems
        .into_iter()
        .map(|p| {
            let ident = quote::format_ident!("{p}");
            quote::quote! {
                (#p, || Box::new(#ident::solution()))
            }
        })
        .collect::<Vec<_>>();

    let code = quote::quote! {
        pub const SOLUTIONS: [(&str, fn() -> Box<dyn std::fmt::Display>); #len] = [
            #(#problems),*
        ];
    };

    file.write_all(code.to_string().as_bytes())?;

    Ok(())
}
