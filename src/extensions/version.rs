use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.major != other.major {
            self.major.cmp(&other.major)
        } else if self.minor != other.minor {
            self.minor.cmp(&other.minor)
        } else {
            self.patch.cmp(&other.patch)
        }
    }
}

pub fn parse(version: &str) -> Version {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        eprintln!("Invalid version format: {version}");
        panic!("Version must be in the format 'major.minor.patch'");
    }
    let major = parts[0].parse().unwrap();
    let minor = parts[1].parse().unwrap();
    let patch = parts[2].parse().unwrap();
    Version {
        major,
        minor,
        patch,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::cmp::Ordering;

    #[rstest]
    #[case("1.2.3", "1.2.4", Ordering::Less)]
    #[case("1.2.4", "1.2.3", Ordering::Greater)]
    #[case("1.2.3", "1.2.3", Ordering::Equal)]
    #[case("1.2.3", "1.3.3", Ordering::Less)]
    #[case("1.3.3", "1.2.3", Ordering::Greater)]
    #[case("1.2.3", "2.2.3", Ordering::Less)]
    #[case("2.2.3", "1.2.3", Ordering::Greater)]
    #[case("1.2.3", "2.3.4", Ordering::Less)]
    #[case("2.3.4", "1.2.3", Ordering::Greater)]
    #[case("1.2.3", "2.3.3", Ordering::Less)]
    #[case("2.3.3", "1.2.3", Ordering::Greater)]
    #[case("1.2.3", "1.3.4", Ordering::Less)]
    fn test_ordering(
        #[case] right_version: &str,
        #[case] left_version: &str,
        #[case] expected: Ordering,
    ) {
        let right = parse(right_version);
        let left = parse(left_version);
        assert_eq!(right.cmp(&left), expected);
    }
}
