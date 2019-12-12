extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use chrono::naive::NaiveDateTime;
use chrono::{DateTime, Utc};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
// use chrono::serde::{};
use serde::{de, Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;
use std::string::String;

#[derive(Deserialize, Debug)]
pub struct Bug {
    html_url: String,
    state: String,
    title: String,
    // updated_at: String,
    #[serde(deserialize_with = "deserialize_from_str")]
    updated_at: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_from_str")]
    created_at: DateTime<Utc>,
    number: i64,
}

// You can use this deserializer for any type that implements FromStr
// and the FromStr::Err implements Display
fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,      // Required for S::from_str...
    S::Err: Display, // Required for .map_err(de::Error::custom)
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(de::Error::custom)
}

#[derive(Deserialize, Debug)]
pub struct Bugs {
    bugs: Vec<Bug>,
}

pub fn load_bugs(file_name: &str) -> Result<Vec<Bug>, Box<Error>> {
    let path = &Path::new(file_name);
    let mut inf = File::open(path)?;
    let mut result = String::new();
    inf.read_to_string(&mut result)?;

    let bugs: Vec<Bug> = serde_json::from_str(result.as_str())?;

    Ok(bugs)
}

pub fn bugscan(bugdir: &str) -> Result<std::vec::Vec<Bug>, Box<Error>> {
    let p = Path::new(bugdir);

    let mut v = Vec::new();

    if p.exists() {
        for fr in p.read_dir()? {
            let f = fr?;
            let fname = f
                .file_name()
                .into_string()
                .map_err(|e| format!("bad filename: {:?}", f))?;

            match Path::new(bugdir).join(fname.clone()).to_str() {
                Some(p) => {
                    let mut nbugs = load_bugs(p)?;
                    println!("issue count {}, {}", fname, nbugs.len());
                    v.append(&mut nbugs);
                }
                None => (),
            }
        }
    }

    Ok(v)
}

fn main() {
    let bugs = bugscan("issue_pages");
    match bugs {
        Ok(bugs) => {
            println!("bugs: {:?}", bugs.len());
            for b in &bugs {
                println!("state: {}", b.state);
            }

            let path = &Path::new("out.txt");
            let mut inf = File::create(path).unwrap();
            for b in bugs {
                inf.write(
                    &format!(
                        "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                        // replace " with "" in title.
                        b.title.replace("\"", "\"\""), b.number, b.html_url, b.created_at, b.updated_at
                    )
                    .as_bytes(),
                )
                .unwrap();
            }
        }
        Err(e) => println!("error: {:?}", e),
    }
}

/*{
  "url": "https://api.github.com/repos/NixOS/nix/issues/3262",
  "repository_url": "https://api.github.com/repos/NixOS/nix",
  "labels_url": "https://api.github.com/repos/NixOS/nix/issues/3262/labels{/name}",
  "comments_url": "https://api.github.com/repos/NixOS/nix/issues/3262/comments",
  "events_url": "https://api.github.com/repos/NixOS/nix/issues/3262/events",
  "html_url": "https://github.com/NixOS/nix/pull/3262",
  "id": 536465567,
  "node_id": "MDExOlB1bGxSZXF1ZXN0MzUxOTkwNjM4",
  "number": 3262,
  "title": "Content-addressed paths",
  "user": {
    "login": "regnat",
    "id": 7226587,
    "node_id": "MDQ6VXNlcjcyMjY1ODc=",
    "avatar_url": "https://avatars2.githubusercontent.com/u/7226587?v=4",
    "gravatar_id": "",
    "url": "https://api.github.com/users/regnat",
    "html_url": "https://github.com/regnat",
    "followers_url": "https://api.github.com/users/regnat/followers",
    "following_url": "https://api.github.com/users/regnat/following{/other_user}",
    "gists_url": "https://api.github.com/users/regnat/gists{/gist_id}",
    "starred_url": "https://api.github.com/users/regnat/starred{/owner}{/repo}",
    "subscriptions_url": "https://api.github.com/users/regnat/subscriptions",
    "organizations_url": "https://api.github.com/users/regnat/orgs",
    "repos_url": "https://api.github.com/users/regnat/repos",
    "events_url": "https://api.github.com/users/regnat/events{/privacy}",
    "received_events_url": "https://api.github.com/users/regnat/received_events",
    "type": "User",
    "site_admin": false
  },
  "labels": [

  ],
  "state": "open",
  "locked": false,
  "assignee": null,
  "assignees": [

  ],
  "milestone": null,
  "comments": 0,
  "created_at": "2019-12-11T16:02:19Z",
  "updated_at": "2019-12-11T16:06:50Z",
  "closed_at": null,
  "author_association": "CONTRIBUTOR",
  "pull_request": {
    "url": "https://api.github.com/repos/NixOS/nix/pulls/3262",
    "html_url": "https://github.com/NixOS/nix/pull/3262",
    "diff_url": "https://github.com/NixOS/nix/pull/3262.diff",
    "patch_url": "https://github.com/NixOS/nix/pull/3262.patch"
  },
  "body": "(draft) companion MR for NixOS/rfcs#62\r\n\r\nMakes derivations with `contentAddressed = true` content-addressed in the store.\r\n\r\nYou probably don't want to try this in an existing Nix installation since it requires a change in the database schema (and there's no guaranty that it won't corrupt the store ;) )"
}*/
