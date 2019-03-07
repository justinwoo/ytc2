#![warn(rust_2018_idioms, clippy::all)]

use std::env;
use std::process::Command;

struct Target {
    title: String, // hello world
    href: String,  // /watch?v=myCode
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let page_url = args.get(1).expect(EXPECT_YOUTUBE_PATH_URL_MESSAGE);

    ensure_sqlite_db();

    let pick_xsl = env::var_os("PICK_XSL")
        .map(|x| x.into_string().unwrap())
        .unwrap_or_else(|| "pick.xsl".to_string());

    let targets = get_targets(page_url, &pick_xsl);

    for target in targets {
        download_target(target);
    }

    println!("done");
}

const EXPECT_YOUTUBE_PATH_URL_MESSAGE: &str = "
You must pass a single argument for what page will be used to fetch targets.
";

fn ensure_sqlite_db() {
    run_command(&format!("sqlite3 ~/.ytc2db '{}'", ENSURE_SQLITE_DB_QUERY));
}

const ENSURE_SQLITE_DB_QUERY: &str = "
create table if not exists downloads (link text primary key unique, title text, created datetime);
";

fn get_targets(page_url: &str, pick_xsl: &str) -> Vec<Target> {
    let output = run_command(&format!(
        r#"
      content=$(curl {}| hxnormalize -x | hxselect '.yt-uix-tile-link') \
      && xml="<results>$content</results>" \
      && echo $xml | xsltproc {} -
    "#,
        page_url, pick_xsl
    ));

    let mut targets = Vec::new();

    for pair in output.split('\n') {
        let parts: Vec<&str> = pair.split("||||||||||").collect();
        if parts.len() != 2 {
            continue;
        }
        let title_split: Vec<&str> = parts[0].splitn(2, ':').collect();
        let href_split: Vec<&str> = parts[1].splitn(2, ':').collect();

        if title_split.len() != 2 || href_split.len() != 2 {
            print!("Warning: could not properly split for {}", pair);
            continue;
        }

        targets.push(Target {
            title: title_split[1].trim().to_string(),
            href: href_split[1].trim().to_string(),
        });
    }

    targets
}

fn run_command(command: &str) -> String {
    let command_ref = &command;
    let attempt = Command::new("bash")
        .arg("-c")
        .arg(command_ref)
        .output()
        .expect("Failed to launch bash command");

    if attempt.status.success() {
        let result: String = String::from_utf8(attempt.stdout)
            .unwrap_or_else(|_| panic!("Invalid output from command {}", command));
        result.trim().to_string()
    } else {
        panic!("Command failed: {}", command)
    }
}

fn download_target(target: Target) {
    let href = format!("https://www.youtube.com{}", target.href);
    let cmd = &format!(
        r#"sqlite3 ~/.ytc2db "select exists(select 1 from downloads where link = '{}')""#,
        href
    );
    let result = run_command(cmd);

    if result == "1" {
        println!("Already downloaded {}", target.title);
        return;
    }

    println!("Downloading {} from {}", target.title, target.href);

    let mut child = Command::new("youtube-dl")
        .arg(&href)
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .spawn()
        .expect("Failed to launch youtube-dl");

    let exit = child.wait().expect("youtube-dl child failed");

    if exit.success() {
        run_command(&format!(
            r#"sqlite3 ~/.ytc2db "insert or ignore into downloads (link, title, created) values ('{}', '{}', datetime('now'));""#,
                             href, target.title));
        println!("Downloaded {}", target.title);
    } else {
        println!("Failed to download {} from {}", target.title, target.href)
    }
}
