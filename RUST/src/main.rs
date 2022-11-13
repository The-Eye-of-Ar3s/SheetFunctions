mod web;

fn main() {
    println!("HELLO WORLD")
}

#[cfg(test)]
mod tests {
    use crate::web::{ENCODEURL, FILTERXML, WEBSERVICE};
    use pretty_assertions::{assert_str_eq};

    #[test]
    #[allow(non_snake_case)]
    fn WEB_URLENCODE() {
        assert_str_eq!(ENCODEURL("'!@#$%^&*()-=_+[]\\{}|;':\",./<>?`~".to_owned()).unwrap(), "%27%21%40%23%24%25%5E%26%2A%28%29-%3D_%2B%5B%5D%5C%7B%7D%7C%3B%27%3A%22%2C.%2F%3C%3E%3F%60%7E".to_owned());
    }

    #[test]
    #[allow(non_snake_case)]
    fn WEB_FILTERXML() {
        assert_str_eq!(FILTERXML("<root>hello</root>".to_owned(), "/root".to_owned()).unwrap(),"hello");
    }

    #[test]
    #[allow(non_snake_case)]
    fn WEB_WEBSERVICE() {
        assert_str_eq!(WEBSERVICE("https://jsonplaceholder.typicode.com/todos/1".to_owned()).unwrap(), r#"{
  "userId": 1,
  "id": 1,
  "title": "delectus aut autem",
  "completed": false
}"#.to_owned())
    }


}