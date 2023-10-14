
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

fn main() {
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
    })
}