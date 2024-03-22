// use flexi_logger::{FileSpec, Logger, WriteMode};

pub fn init_logger(debug: bool) {
    let log_level = if debug {
        log::Level::max()
    } else {
        log::Level::Warn
    };

    let init_result = simple_logger::init_with_level(log_level);

    match init_result {
        Ok(_) => {}
        Err(e) if debug => {
            // This usually just means that the logger has already been initialized.
            eprintln!("Error initializing logger: {}", e);
        }
        Err(_) => {}
    }
}
