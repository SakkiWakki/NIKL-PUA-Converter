use std::{env, fs, path::Path};

use hypua::to_ipf_string;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (in_dir, out_dir) = (&args[1], &args[2]);
    fs::create_dir_all(out_dir)?;

    for entry in fs::read_dir(in_dir)? {
        let entry = entry?;
        let path = entry.path();
        // Parse only xml files for the dataset ()
        if path.extension().and_then(|s| s.to_str()) == Some("xml") {
            let out_path = Path::new(out_dir).join(path.file_name().unwrap());
            let text = fs::read_to_string(&path)?;
            let converted = to_ipf_string(&text);
            fs::write(out_path, converted.as_ref())?;
        }
    }
    Ok(())
}