
use std::{fs, path::PathBuf, ffi::OsStr, process::Command, error::Error};

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

fn system(cmd: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let out = Command::new(cmd)
        .args(args)
        .output()
        ?;

    if out.stderr.len() != 0 {
        panic!("Got this while running {cmd} with \"{}\": {}", args.join(" "), String::from_utf8(out.stderr).unwrap())
    }

    Ok(String::from_utf8(out.stdout)?)
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
            .arg(asset)
            .arg("-o")
            .arg(asset_path(asset))
            .arg("-c")
            .spawn()
            .unwrap();
    });

    let commit = system("git", &["rev-parse", "HEAD"]).map_err(|x| x.to_string());
    let branch = system("git", &["rev-parse", "--abbrev-ref", "HEAD"]).map_err(|x| x.to_string());

    match commit {
        Err(err) => panic!("Can't get commit: {}", err),
        Ok(commit) => println!("cargo:rustc-env=COMMIT_HASH={commit}")
    }

    match branch {
        Err(err) => panic!("Can't get commit: {}", err),
        Ok(branch) => println!("cargo:rustc-env=COMMIT_BRANCH={branch}")
    }
}