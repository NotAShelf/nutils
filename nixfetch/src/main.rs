use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "nixfetch",
    about = "Add a file to /nix/store and output its path"
)]
struct Opt {
    /// Print this help message
    #[structopt(short = "h", long = "help")]
    help: bool,

    /// Set the file's executable bit
    #[structopt(short = "x", long = "executable")]
    executable: bool,

    /// Unpack the file (url can also be a flakeref)
    #[structopt(long = "unpack")]
    unpack: bool,

    /// The URL of the file to fetch
    url: String,
}

pub fn fetch_file(url: &str, unpack: bool, executable: bool) -> Result<PathBuf, String> {
    let mut command = Command::new("nix");
    if unpack {
        command.arg("flake").arg("prefetch").arg("--json");
    } else {
        command.arg("store").arg("prefetch-file").arg("--json");
        if executable {
            command.arg("--executable");
        }
    }
    command.arg(url);

    let output = command
        .output()
        .map_err(|e| format!("Error executing command: {}", e))?;
    if !output.status.success() {
        return Err(format!(
            "Error: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    let storepath = String::from_utf8_lossy(&output.stdout).trim().to_string();

    Ok(PathBuf::from(storepath))
}

fn main() {
    let opt = Opt::from_args();

    if opt.help {
        Opt::clap().print_help().unwrap();
        return;
    }

    if opt.url.is_empty() {
        eprintln!("Error: No URL provided.");
        Opt::clap().print_help().unwrap();
        std::process::exit(125);
    }

    if opt.executable && opt.unpack {
        eprintln!("Error: -x/--executable and --unpack cannot be used together.");
        Opt::clap().print_help().unwrap();
        std::process::exit(125);
    }

    let storepath = fetch_file(&opt.url, opt.unpack, opt.executable).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    println!("{}", storepath.display());
}
