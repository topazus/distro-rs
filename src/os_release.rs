use std::io::BufRead;
use std::iter::FromIterator;

macro_rules! parse_os_release_line {
    ($line:expr, { $($regex:expr => $value:expr),+ }) => {
        {
            $(
                if let Some(caps) = $regex.captures($line) {
                    $value = caps.get(1).unwrap().as_str().to_string();
                    continue;
                }
            )+
        }
    };
}

/// Contents of the `/etc/os-release` file, as a data structure.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OsRelease {
    /// The URL where bugs should be reported for this OS.
    pub bug_report_url: String,
    /// The homepage of this OS.
    pub home_url: String,
    /// Identifier of the original upstream OS that this release is a derivative of.
    ///
    /// **IE:** `debian`
    pub id_like: String,
    /// An identifier which describes this release, such as `ubuntu`.
    ///
    /// **IE:** `ubuntu`
    pub id: String,
    /// The name of this release, without the version string.
    ///
    /// **IE:** `Ubuntu`
    pub name: String,
    /// The name of this release, with th eversion stirng.
    ///
    /// **IE:** `Ubuntu 18.04 LTS`
    pub pretty_name: String,
    /// The URL describing this OS's privacy policy.
    pub privacy_policy_url: String,
    /// The URL for seeking support with this OS release.
    pub support_url: String,
    /// The codename of this version.
    ///
    /// **IE:** `bionic`
    pub version_codename: String,
    /// The version of this OS release, with additional details about the release.
    ///
    /// **IE:** `18.04 LTS (Bionic Beaver)`
    pub version_id: String,
    /// The version of this OS release.
    ///
    /// **IE:** `18.04`
    pub version: String,
    /// Additional keys not covered by the API.
    pub extra: std::collections::BTreeMap<String, String>,
}

impl OsRelease {
    /// Attempt to parse the contents of `/etc/os-release`.
    pub fn new() -> std::io::Result<OsRelease> {
        let file = std::io::BufReader::new(std::fs::File::open("/etc/os-release")?);
        let lines = file.lines().flat_map(|line| line);
        Ok(OsRelease::from_iter(lines))
    }

    /// Attempt to parse any `/etc/os-release`-like file.
    pub fn from_file(path: &str) -> std::io::Result<OsRelease> {
        let file = std::io::BufReader::new(std::fs::File::open(path)?);
        Ok(OsRelease::from_iter(file.lines().flat_map(|line| line)))
    }
}

impl FromIterator<String> for OsRelease {
    fn from_iter<I: IntoIterator<Item = String>>(lines: I) -> Self {
        let mut os_release = Self::default();

        for line in lines {
            parse_os_release_line!(&line, {
            regex::Regex::new(r#"^NAME="?([^"]+)"?$"#).unwrap() => os_release.name,
            regex::Regex::new(r#"^VERSION="?([^"]+)"?$"#).unwrap() => os_release.version,
            regex::Regex::new(r#"^ID="?([^"]+)"?$"#).unwrap() => os_release.id,
            regex::Regex::new(r#"^ID_LIKE="?([^"]+)"?$"#).unwrap() => os_release.id_like,
            regex::Regex::new(r#"^PRETTY_NAME="?([^"]+)"?$"#).unwrap() => os_release.pretty_name,
            regex::Regex::new(r#"^VERSION_ID="?([^"]+)"?$"#).unwrap() => os_release.version_id,
            regex::Regex::new(r#"^HOME_URL="?([^"]+)"?$"#).unwrap() => os_release.home_url,
            regex::Regex::new(r#"^SUPPORT_URL="?([^"]+)"?$"#).unwrap() => os_release.support_url,
            regex::Regex::new(r#"^BUG_REPORT_URL="?([^"]+)"?$"#).unwrap() => os_release.bug_report_url,
            regex::Regex::new(r#"^PRIVACY_POLICY_URL="?([^"]+)"?$"#).unwrap() => os_release.privacy_policy_url,
            regex::Regex::new(r#"^VERSION_CODENAME="?([^"]+)"?$"#).unwrap() => os_release.version_codename
            });
            let re = regex::Regex::new(r#"(\w+)="?([^"]+)"?"#).unwrap();
            if let Some(cap) = re.captures(&line) {
                os_release
                    .extra
                    .insert(cap[1].to_owned().to_string(), String::from(&cap[2]));
            }
        }
        os_release
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE: &str = r#"NAME="Pop!_OS"
VERSION="18.04 LTS"
ID=ubuntu
ID_LIKE=debian
PRETTY_NAME="Pop!_OS 18.04 LTS"
VERSION_ID="18.04"
HOME_URL="https://system76.com/pop"
SUPPORT_URL="http://support.system76.com"
BUG_REPORT_URL="https://github.com/pop-os/pop/issues"
PRIVACY_POLICY_URL="https://system76.com/privacy"
VERSION_CODENAME=bionic
EXTRA_KEY=thing
ANOTHER_KEY="#;
    #[test]
    fn os_release() {
        let os_release = OsRelease::from_iter(EXAMPLE.lines().map(|x| x.to_owned()));

        assert_eq!(
            os_release,
            OsRelease {
                name: "Pop!_OS".into(),
                version: "18.04 LTS".into(),
                id: "ubuntu".into(),
                id_like: "debian".into(),
                pretty_name: "Pop!_OS 18.04 LTS".into(),
                version_id: "18.04".into(),
                home_url: "https://system76.com/pop".into(),
                support_url: "http://support.system76.com".into(),
                bug_report_url: "https://github.com/pop-os/pop/issues".into(),
                privacy_policy_url: "https://system76.com/privacy".into(),
                version_codename: "bionic".into(),
                extra: {
                    let mut map = std::collections::BTreeMap::new();
                    map.insert("EXTRA_KEY".to_owned(), "thing".to_owned());
                    map
                }
            }
        )
    }
}
