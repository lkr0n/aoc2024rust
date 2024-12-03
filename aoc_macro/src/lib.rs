use reqwest;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

const AOC_SESSION_COOKIE: &str = "AOC_SESSION_COOKIE";

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("Need env var '{0}' to be defined")]
    Env(&'static str),

    #[error("Failed to read '{0}'")]
    Read(&'static str),

    #[error("Failed to write '{0}'")]
    Write(&'static str),

    #[error("Failed to build http client: {0:?}")]
    ClientBuilder(reqwest::Error),

    #[error("Failed to fetch input: {0:?}")]
    Get(reqwest::Error),

    #[error("Failed to parse url: {0}'")]
    UrlParse(String),

    #[error("Decoding URL into utf8: {0:?}")]
    Utf8(reqwest::Error)
}

pub fn ensure_filename(day_str: &str, filename: &'static str) 
    -> Result<(), AocError> {

    if std::fs::exists(filename).unwrap() {
        return Ok(());
    }

    // extract the day number from the day_str.
    // It is expected that day_str is of the format dayXX
    // where XX is any integer in [1, 25] 
    let day = day_str.strip_prefix("day").unwrap();

    // figure out the URL from which we can download the challenge input
    let url = format!("https://adventofcode.com/2024/day/{day}/input");
    let reqwest_url = url.parse::<reqwest::Url>()
        .map_err(|_| AocError::UrlParse(url.clone()))?;

    // setup a cookie jar containing the session hash for auth with 
    // adventofcode.com
    let session_hash = std::env::var(AOC_SESSION_COOKIE)
        .map_err(|_| AocError::Env(AOC_SESSION_COOKIE))?;
    let cookie = format!("session={}; Domain=.adventofcode.com;", session_hash);

    let jar = reqwest::cookie::Jar::default();
    jar.add_cookie_str(&cookie, &reqwest_url);

    // fetch the input data from adventofcode.com
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .cookie_provider(jar.into())
        .build().map_err(AocError::ClientBuilder)?;

    let data = client.get(url).send().map_err(AocError::Get)?
        .text().map_err(AocError::Utf8)?;

    std::fs::write(filename, data).map_err(|_| AocError::Write(filename))?;

    Ok(())
}

#[macro_export]
macro_rules! execute {
    ($module:ident) => {
        {
            fn execute() -> Result<(), aoc_macro::AocError> {
                let filename = concat!("input/", stringify!($module), ".txt");

                let day_str = stringify!($module);
                aoc_macro::ensure_filename(day_str, filename)?;

                let input = std::fs::read_to_string(filename)
                    .map_err(|_| aoc_macro::AocError::Read(filename))?;

                let answer = $module::part1(&input);
                println!("{} part1: {}", day_str, answer);

                let answer = $module::part2(&input);
                println!("{} part2: {}", day_str, answer);
                Ok(())
            };

            if let Err(error) = execute() {
                eprintln!("ERROR: {:?}", error);
            }
        }
    }
}
