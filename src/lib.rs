
use log::{trace,debug};
use std::fs::File;
use std::io::Write;
use std::io::LineWriter;

struct Definition {
    name: String,
    defined: bool,
    value: String,
    optional: bool,
}

pub struct AutoCfg {
    config_dir: String,
    defines: Vec<Definition>,
}

impl AutoCfg {

    pub fn new() -> AutoCfg {
        let mut result = AutoCfg {
            config_dir: String::from("config.h"),
            defines: Vec::<Definition>::new(),
        };
        result.init();
        result
    }

    fn init(&mut self) {
        self.set_quoted("PACKAGE_NAME", "example");
        self.set_quoted("VERSION", "1.0.0")
    }

    pub fn set(&mut self, name: &str, value: &str) {
        trace!("set: name = {} value = {}", name, value);
        // TODO - save in cache ...
        self.defines.push(Definition{
            name: String::from(name),
            defined: true,
            value: String::from(value),
            optional: false,
        });
    }

    pub fn set_quoted(&mut self, name: &str, value: &str) {
        let new_value = format!("\"{}\"", value);
        self.set(name, &new_value);
    }

    pub fn write(&self) -> Result<(), std::io::Error> {

        let mut stream = File::create(&self.config_dir)?;

        let mut output = LineWriter::new(stream);

        for def in &self.defines {
            let rec = format!("#define {} {}\n", def.name, def.value);
            output.write_all(rec.as_bytes())?;
        }


        output.flush()?;

        Ok(())
    }

}
