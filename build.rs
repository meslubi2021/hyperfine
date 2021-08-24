use std::fs;

use clap::Shell;

include!("src/app.rs");

fn main() {
    let min_version = "1.39";

    match version_check::is_min_version(min_version) {
        Some(true) => {}
        // rustc version too small or can't figure it out
        _ => {
            eprintln!("'hyperfine' requires rustc >= {}", min_version);
            std::process::exit(1);
        }
    }

    let var = std::env::var_os("SHELL_COMPLETIONS_DIR").or_else(|| std::env::var_os("OUT_DIR"));
    let outdir = match var {
        None => return,
        Some(outdir) => outdir,
    };
    fs::create_dir_all(&outdir).unwrap();

    let mut app = build_app();
    app.gen_completions("hyperfine", Shell::Bash, &outdir);
    app.gen_completions("hyperfine", Shell::Fish, &outdir);
    app.gen_completions("hyperfine", Shell::Zsh, &outdir);
    app.gen_completions("hyperfine", Shell::PowerShell, &outdir);
}
