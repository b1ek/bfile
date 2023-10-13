use std::{fs, path::PathBuf};
use css_minify::optimizations::{Minifier, Level};

fn main() {
    let assets = fs::read_dir("static/assets").unwrap();
    let assets = assets
        .map(|x| x.unwrap().path())
        .filter(|x| x.extension().is_some())
        .filter(|x| {
            let ext = x.extension().clone().unwrap().to_str().unwrap().to_string();
            ext == "css"
        })
        .collect::<Vec<PathBuf>>();
    
    assets.iter().for_each(|asset| {
        let bundled = Minifier::default().minify(
            String::from_utf8(fs::read(asset).unwrap()).unwrap().as_str(),
            Level::Zero
        ).unwrap();
        
        fs::write({
            let mut path = asset.components().take(1).collect::<PathBuf>();
            path.push(asset.components().last().unwrap());
            println!("{:?}", path);
            path
        }, bundled).unwrap();
    })
}