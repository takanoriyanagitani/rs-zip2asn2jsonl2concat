use std::process::ExitCode;

use std::io;

use std::fs::File;

use rs_zip2asn2jsonl2concat::zipfile2jsonl2zcat2stdout;

fn env_val_by_key(key: &'static str) -> impl FnMut() -> Result<String, io::Error> {
    move || {
        std::env::var(key)
            .map_err(|e| format!("env var {key} missing: {e}"))
            .map_err(io::Error::other)
    }
}

fn env2input_zip_filename() -> Result<String, io::Error> {
    env_val_by_key("ENV_INPUT_ZIP_FILENAME")()
}

fn sub() -> Result<(), io::Error> {
    let izipname: String = env2input_zip_filename()?;
    let f: File = File::open(izipname)?;
    zipfile2jsonl2zcat2stdout(f)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
