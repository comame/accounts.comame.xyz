use std::collections::HashSet;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Scopes {
    value: HashSet<String>,
}

impl Scopes {
    pub fn parse(str: &str) -> Self {
        let scopes: Vec<&str> = str.split(' ').collect();
        let mut set = HashSet::new();

        for scope in scopes {
            set.insert(scope.to_string());
        }

        Self { value: set }
    }

    pub fn has(&self, target: &str) -> bool {
        self.value.contains(target)
    }

    pub fn within(&self, target: &str) -> bool {
        let target = Self::parse(target);
        self.value.iter().all(|v| target.value.contains(v))
    }
}

impl Display for Scopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for (i, scope) in self.value.iter().enumerate() {
            str.push_str(scope);
            if i < self.value.len() - 1 {
                str.push(' ');
            }
        }
        write!(f, "{str}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Scopes::parse("openid code"), Scopes::parse("code openid"));
    }

    #[test]
    fn test_has() {
        assert!(Scopes::parse("openid code").has("openid"));
        assert!(Scopes::parse("code openid").has("code"));
    }

    #[test]
    fn test_within() {
        assert!(Scopes::parse("openid").within("openid"));
        assert!(Scopes::parse("openid").within("openid email"));
        assert!(Scopes::parse("openid").within("openid email profile"));
    }
}
