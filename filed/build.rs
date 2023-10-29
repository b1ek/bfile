
use std::{fs, path::PathBuf, ffi::OsStr, process::Command};

use css_minify::optimizations::{Minifier, Level};

fn asset_path(asset: &PathBuf) -> PathBuf {
    let mut path = asset.components().take(1).collect::<PathBuf>();
    path.push(asset.components().last().unwrap());
    path
}

fn extfilter(valid: String, x: Option<&OsStr>) -> bool {
    if x.is_none() {
        return false
    }
    let ext = x.clone().unwrap().to_str().unwrap().to_string();
    ext == valid
}

fn system(cmd: &str, args: &[&str]) -> String {
    let out = Command::new(cmd)
        .args(args)
        .output()
        .unwrap();

    if out.stderr.len() != 0 {
        panic!("Got this while running {cmd} with \"{}\": {}", args.join(" "), String::from_utf8(out.stderr).unwrap())
    }

    String::from_utf8(out.stdout).unwrap()
}

fn main() {

    println!("cargo:rerun-if-changed=static/assets");

    let assets = fs::read_dir("static/assets").unwrap();
    let assets = assets
        .map(|x| x.unwrap().path())
        .filter(|x| x.extension().is_some())
        .collect::<Vec<PathBuf>>();
    
    let styles = assets
        .iter()
        .filter(|x| {
            extfilter("css".into(), x.extension())
        })
        .collect::<Vec<&PathBuf>>();

    let scripts = assets
        .iter()
        .filter(|x| {
            extfilter("js".into(), x.extension())
        })
        .collect::<Vec<&PathBuf>>();

    styles.iter().for_each(|asset| {
        let bundled = Minifier::default().minify(
            String::from_utf8(fs::read(asset).unwrap()).unwrap().as_str(),
            Level::Zero
        ).unwrap();
        
        fs::write(asset_path(asset), bundled).unwrap();
    });

    scripts.iter().for_each(|asset| {
        Command::new("uglifyjs")
            .arg("-c")
            .arg(asset)
            .arg("-o")
            .arg(asset_path(asset))
            .spawn()
            .unwrap();
    });

    let commit = system("git", &["rev-parse", "HEAD"]);
    let branch = system("git", &["rev-parse", "--abbrev-ref", "HEAD"]);

    println!("cargo:rustc-env=COMMIT_HASH={commit}");
    println!("cargo:rustc-env=COMMIT_BRANCH={branch}");
}