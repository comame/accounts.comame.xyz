fn trim_index(uri_path: &mut String) -> bool {
    let c = uri_path.as_str();
    let len_of_index_html = "index.html".len();

    if c.len() < len_of_index_html {
        return false;
    }

    if &c[(c.len() - len_of_index_html)..(c.len())] == "index.html" {
        *uri_path = String::from(&c[0..(c.len() - len_of_index_html)]);
        return true;
    }

    false
}

fn trim_slash(uri_path: &mut String) -> bool {
    let chars = uri_path.as_str();
    if &chars[chars.len() - 1..chars.len()] == "/" {
        uri_path.remove(uri_path.len() - 1);
        return true;
    }

    false
}

pub fn trim(uri_path: &mut String) -> bool {
    if uri_path == "/" {
        return false;
    }

    if uri_path == "" {
        *uri_path = String::from("/");
        return true;
    }

    let trimed_by_index = trim_index(uri_path);
    let trimed_by_slash = trim_slash(uri_path);

    if uri_path == "" {
        *uri_path = String::from("/");
        return true;
    }

    trimed_by_index || trimed_by_slash
}

#[cfg(test)]
mod tests {
    use super::*;

    type Testcase = (&'static str, &'static str, bool);

    fn testcases() -> Vec<Testcase> {
        vec![
            ("/", "/", false),
            ("", "/", true),
            ("/foo", "/foo", false),
            ("/index.html", "/", true),
            ("/foo/", "/foo", true),
            ("/foo/index.html", "/foo", true),
        ]
    }

    fn test(testcase: Testcase) {
        let mut path = String::from(testcase.0);
        assert_eq!(trim(&mut path), testcase.2);
        assert_eq!(path, testcase.1);
    }

    #[test]
    fn root_ends_with_slash() {
        test(testcases()[0]);
    }

    #[test]
    fn root_empty_string() {
        test(testcases()[1]);
    }

    #[test]
    fn random_uri() {
        test(testcases()[2]);
    }

    #[test]
    fn root_ends_with_index_html() {
        test(testcases()[3]);
    }

    #[test]
    fn ends_with_slash() {
        test(testcases()[4]);
    }

    #[test]
    fn ends_with_slash_and_index_html() {
        test(testcases()[5]);
    }
}
