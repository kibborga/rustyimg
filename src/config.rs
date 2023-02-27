pub mod config {

    use clap::{arg, Command, Parser};
    use clap::*;
    use std::collections::HashMap;

    #[derive(Parser, Debug)]

    // #[clap(author="Author Name", version, about="A Very simple Package Hunter")]
    #[command(author, version, about, long_about = None)]


    pub struct ConfigOptions {
        pub action: String,
        pub src_file: String,
        pub grayscale: bool,
        //pub autocontrast: bool,
        pub verbose: bool,
        pub suffix: String,
        pub overwrite: bool,
        pub force: bool,
        pub error: bool,
        pub invert: bool,
        pub fliph: bool,
        pub flipv: bool,
        pub noexif: bool,
    }

    lazy_static! {
        pub static ref OPTS: HashMap<&'static str, String> = {
            let args = parse_cmdline_args().get_matches();
            let mut opts = HashMap::new();
            if let Some(c) = args.get_one::<String>("COMMAND") {
                opts.insert("action", c.to_string());
            } else {
                println!("Command not specified");
                println!("Possible commands: ");
                println!("  - convert-heic: Converts HEIC to JPG");
                println!("  - process: Applie all transformations");
                println!("  - set-date: Sets the EXIF and file date to the date specified with -dt=YYYY-MM-DD");
                println!("  - print-exit: Prints the EXIF data");
                println!("  - fix-jpeg-ext: Renames *.JPEG to JPG");

                opts.insert("error", String::from("true"));
            }
            if let Some(c) = args.get_one::<String>("INPUT") {
                opts.insert("src_file", c.to_string());
            }
            if let Some(c) = args.get_one::<bool>("verbose") {
                opts.insert(
                    "verbose",
                    if *c {
                        String::from("true")
                    } else {
                        String::from("false")
                    },
                );
            }

            if let Some(c) = args.get_one::<String>("suffix") {
                opts.insert("suffix", c.to_string());
            }

            if let Some(c) = args.get_one::<bool>("overwrite") {
                opts.insert(
                    "overwrite",
                    if *c {
                        String::from("true")
                    } else {
                        String::from("false")
                    },
                );
            }

            if let Some(c) = args.get_one::<bool>("force") {
                opts.insert(
                    "force",
                    if *c {
                        String::from("true")
                    } else {
                        String::from("false")
                    },
                );
            }

            if let Some(c) = args.get_one::<bool>("debug") {
                opts.insert(
                    "debug",
                    if *c {
                        String::from("true")
                    } else {
                        String::from("false")
                    },
                );
            }

            if let Some(c) = args.get_one::<String>("date") {
                opts.insert("date", c.to_string());
            }

            if let Some(c) = args.get_one::<String>("artist") {
                opts.insert("artist", c.to_string());
            }

            if let Some(c) = args.get_one::<bool>("fliph") {
                opts.insert(
                    "fliph",
                    if *c {
                        String::from("true")
                    } else {
                        String::from("false")
                    },
                );
            }

            if let Some(c) = args.get_one::<bool>("flipv") {
                opts.insert(
                    "flipv",
                    if *c {
                        String::from("true")
                    } else {
                        String::from("false")
                    },
                );
            }


            if let Some(c) = args.get_one::<bool>("invert") {
                opts.insert(
                    "invert",
                    if *c {
                        String::from("true")
                    } else {
                        String::from("false")
                    },
                );
            }

            if let Some(c) = args.get_one::<bool>("noexif") {
                opts.insert(
                    "noexif",
                    if *c {
                        String::from("true")
                    } else {
                        String::from("false")
                    },
                );
            }

            if let Some(c) = args.get_one::<bool>("grayscale") {
                opts.insert(
                    "grayscale",
                    if *c {
                        String::from("true")
                    } else {
                        String::from("false")
                    },
                );
            }

            // if let Some(c) = args.get_one::<bool>("autocontrast") {
            //     opts.insert(
            //         "autocontrast",
            //         if *c {
            //             String::from("true")
            //         } else {
            //             String::from("false")
            //         },
            //     );
            // }

            opts
        };
    }


    pub fn option(name: &str, default: &str) -> String {
        return OPTS.get(name).unwrap_or(&String::from(default)).to_string();
    }

    pub fn get() -> ConfigOptions {
        let opts = ConfigOptions {
            action: option("action", ""),
            src_file: option("src_file", ""),
            verbose: option("verbose", "false") == "true",
            suffix: option("suffix", ""),
            overwrite: option("overwrite", "false") == "true",
            force: option("force", "false") == "true",
            error: option("error", "false") == "true",
            invert: option("invert", "false") == "true",
            fliph: option("fliph", "false") == "true",
            flipv: option("flipv", "false") == "true",
            noexif: option("noexif", "false") == "true",
            grayscale: option("grayscale", "false") == "true",
            //autocontrast: option("autocontrast", "false") == "true",
        };
        return opts;
    }

    /**
     * Parses the command line arguments
     */
    fn parse_cmdline_args() -> Command {
        return Command::new("prog")
            .propagate_version(true)
            .args(&[
                arg!(<COMMAND> "action to perform"),
                arg!(-s --suffix <SUFFIX> "suffix to append to the output file name"),
                arg!(-o --overwrite "overwrite the original file"),
                arg!(-f --force "force grayscale action"),
                arg!(-g --grayscale "convert to grayscale"),
                // arg!(-c --autocontrast "apply auto-contrast to the image"),
                arg!(-i --invert "invert image"),
                arg!(-x --fliph "flip image horizontally"),
                arg!(-y --flipv "flip image vertically"),
                arg!(-n --noexif "do not preserve the EXIF data"),
                arg!(-t --date <DATE> "date to set to the file with set-date action"),
                arg!(-a --artist <ARTIST> "artist to set to the file with set-artist action"),
                arg!(-v --verbose "turns on verbose mode"),
                arg!(-d --debug "print debug messages"),
                arg!(<INPUT> "input file or directory"),
            ])
            .version("0.1.0")
            .author(
                "Chavdar Yordanov"
            )
            .about("A Rusty image manipulation library");
    }

}

pub use config::*;
