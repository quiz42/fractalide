#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::fs;

mod contract_capnp {
    include!("path.rs");
    include!("generic_text.rs");
}
use contract_capnp::path;
use contract_capnp::generic_text;

use std::process::Command;
use std::process;
use std::path::Path;
use std::env;

component! {
    fvm,
    inputs(path: path, contract: generic_text, input: generic_text),
    inputs_array(),
    outputs(output: any),
    outputs_array(),
    option(),
    acc(),
    fn run(&mut self) -> Result<()>{

        let path_ip = try!(self.ports.recv("path"));
        let path = try!(path_ip.get_reader());
        let path: path::Reader = try!(path.get_root());
        let path = try!(path.get_path());

        let contract_ip = try!(self.ports.recv("contract"));
        let contract = try!(contract_ip.get_reader());
        let contract: generic_text::Reader = try!(contract.get_root());
        let f_contract = try!(contract.get_text());

        let input_ip = try!(self.ports.recv("input"));
        let input = try!(input_ip.get_reader());
        let input: generic_text::Reader = try!(input.get_root());
        let input = try!(input.get_text());

        let pstr = env::current_exe().unwrap();
        let parent_dir = Path::new(&pstr).parent();
        let capnp_path = try!(parent_dir.and_then(|s| s.to_str()).map(|s| {format!("{}/../bootstrap/capnp", s)}).ok_or(result::Error::Misc("cannot create capnp path".into())));

        let mut child = try!(Command::new(capnp_path)
            .arg("encode")
            .arg(path)
            .arg(f_contract)
            .stdin(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .spawn()
            );

        if let Some(ref mut stdin) = child.stdin {
            try!(stdin.write_all(input.as_bytes()));
        } else {
            unreachable!();
        }
        let output = try!(child.wait_with_output());

        if !output.status.success() {
            return Err(result::Error::Misc("capnp encode command doesn't work".into()));
        }

        let send_ip = IP { vec : output.stdout };
        let _ = self.ports.send("output", send_ip);
        Ok(())
    }
}
