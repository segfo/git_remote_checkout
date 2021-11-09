use clap::*;
pub fn init_args<'a>() -> CmdArgs<'a> {
    let args = app_from_crate!()
        .arg(
            Arg::with_name("branch_name")
                .required(true)
                .help("Specify the path of the search target file or directory"),
        )
        .get_matches();
    CmdArgs::new(args)
}

pub struct CmdArgs<'a> {
    args: clap::ArgMatches<'a>,
}
impl<'a> CmdArgs<'a> {
    fn new(args: clap::ArgMatches<'a>) -> Self {
        CmdArgs { args: args }
    }
    pub fn branch_name(&self) -> &str {
        self.args.value_of("branch_name").unwrap()
    }
}
