mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let arg_matches = args::get_args();
    match arg_matches.subcommand() {
        Some(("print", sm)) => {
            let filepath = sm.get_one::<String>("filepath").unwrap();
            commands::print(filepath);
        }
        Some(("encode", sm)) => {
            let filepath = sm.get_one::<String>("filepath").unwrap();
            let chunk_type = sm.get_one::<String>("chunk type").unwrap();
            let message = sm.get_one::<String>("message").unwrap();

            commands::encode(filepath, chunk_type, message);
        }
        Some(("decode", sm)) => {
            let filepath = sm.get_one::<String>("filepath").unwrap();
            let chunk_type = sm.get_one::<String>("chunk type").unwrap();

            commands::decode(filepath, chunk_type);
        }
        Some(("remove", sm)) => {
            let filepath = sm.get_one::<String>("filepath").unwrap();
            let chunk_type = sm.get_one::<String>("chunk type").unwrap();

            commands::remove(filepath, chunk_type);
        }
        _ => (),
    }
    Ok(())
}
