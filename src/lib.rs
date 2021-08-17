
use std::fs::File;
use std::io::LineWriter;

struct AutoCfg {
    config_dir: &str
}

impl AutoCfg {

    fn new() -> AutoCfg {
        AutoCfg {
            config_dir: ".",
        }
    }

    fn write(&self) -> Result<()> {

        let mut stream = File::create(self.config_dir);

        let mut output = LineWriter::new(stream);

        output.write_all("#define FOO_BAR_H 1\n");

        output.write_all(format!("#define PACKAGE_NAME \"{}\"\n", "example"));

        output.write_all(format!("#define VERSION \"{}\"\n", "1.0.0"));

        let _status = output.flush();

        Ok(())
    }

}