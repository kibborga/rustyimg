pub mod rustyexif {

    use std::process::Command;
    use json;
    pub use crate::config::*;

    use crate::log;
    use crate::debug;

    pub struct ExifField {
        pub name: String,
        pub value: String,
    }

    pub fn read_exif_from_file(path: &str) -> Vec<ExifField> {
        let filter = ["SourceFile", "ExifToolVersion", "FileName", "Directory", "FileAccessDate", "FileInodeChangeDate", "FileModifyDate", "FilePermissions", "FileSize", "FileType", "FileTypeExtension", "MIMEType"];
        match Command::new("exiftool").arg("-j").arg(path).output() {
            Ok(output) => {
                let json = String::from_utf8_lossy(&output.stdout).to_string();
                let parsed = json::parse(&json).unwrap();
                let mut fields: Vec<ExifField> = Vec::new();

                for item in parsed.members() {
                    for (key, value) in item.entries() {
                        if filter.contains(&key) {
                            continue;
                        }
                        let field = ExifField {
                            name: key.to_string(),
                            value: value.to_string(),
                        };
                        fields.push(field);
                    }
                }
                log!(" > Read exif data");
                return fields;
            }
            Err(e) => {
                println!("Error reading exif data: {:?}", e);
                return Vec::new();
            }
        };
    }

    pub fn write_exif_to_file(path: &str, fields: Vec<ExifField>) -> u8
    {

        log!(" > Writing exif data to {}", path);

        let mut args = vec!["-overwrite_original".to_string()];
        for field in fields {
            if field.name != "SourceFile" {
                args.push(format!("-{}={}", field.name, field.value));
                debug!(" > exif {}={}", field.name, field.value);
            }
        }
        args.push(path.to_string());

        match Command::new("exiftool").args(args).output() {
            Ok(_) => {
                log!(" > EXIF data written to {}", path);
                return 1;
            }
            Err(e) => {
                println!("Error writing exif data: {:?}", e);
                return 0;
            }
        };
    }


}

pub use rustyexif::*;
