use fastmurmur3::hash;

use url::Url;
use serde::Deserialize;

use structopt::StructOpt;
use std::path::PathBuf;
use std::fs::File;
use std::io;

#[allow(non_snake_case)]
#[derive(Debug, StructOpt)]
#[structopt(name = "add_hashid_by_domain", about = "Add hash id to URLs based on domain", rename_all="verbatim")]
struct Opt
{
    #[structopt(parse(from_os_str))]
    /// optional file with on entry per line [default: STDIN]
    input: Option<PathBuf>,
/*
    #[structopt(parse(from_os_str), long, short)]
    /// file to save PNG plot to
    output: PathBuf,
*/
}

#[derive(Debug, Deserialize)]
struct UrlEntry
{
    url: String,
}

fn main()
//fn main() -> Result<(), Box<dyn Error>>
{
    let opt = Opt::from_args();

    let input: Box<dyn std::io::Read + 'static> =
        if let Some(path) = &opt.input
        {
            Box::new(File::open(&path).unwrap())
        }
        else
        {
            Box::new(io::stdin())
        };

    csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(input)
        .deserialize::<UrlEntry>()
        .fold((), |_i,entry|
        {
            if let Ok(url_entry) = entry
            {
                if let Ok(url) = Url::parse(&url_entry.url)
                {
                    if let Some(domain) = url.domain()
                    {
                        let h = hash(domain.as_bytes());
                        println!("{h}\t{domain}\t{url}");
                    }
                }
            }
        })
}
