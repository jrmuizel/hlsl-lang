use std::env;

use anyhow::bail;
use argh::FromArgs;

mod dxc;

#[derive(Debug, FromArgs)]
/// hlsl-lang task runner
struct Opts {
    #[argh(subcommand)]
    command: Command,
}

#[derive(Debug, FromArgs)]
#[argh(subcommand)]
enum Command {
    GenerateTests(GenerateTestsOpts),
}

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "gen-tests")]
/// Generate DXC test files
struct GenerateTestsOpts {}

fn generate_tests(_opts: &Opts, _gen_test_opts: &GenerateTestsOpts) -> anyhow::Result<()> {
    let base_dir = env::current_dir().expect("failed to read current dir");

    let mut base_dir = base_dir.as_path();

    while !base_dir.join("data").exists() {
        if let Some(parent) = base_dir.parent() {
            base_dir = parent;
        } else {
            bail!("failed finding root directory");
        }
    }

    eprintln!("Found base directory: {}", base_dir.display());

    let tests = dxc::discover_tests(base_dir);

    tests.write_entry(
        &base_dir.join("lang-pp/tests/dxc.rs"),
        "#![cfg(feature = \"full\")]\n",
    )?;
    tests.write_entry(&base_dir.join("lang/tests/dxc.rs"), "")?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = argh::from_env();
    match &opts.command {
        Command::GenerateTests(ref generate_tests_opts) => {
            generate_tests(&opts, generate_tests_opts)
        }
    }
}
