// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<&str> for Status {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lower_cased = value.to_lowercase();
        let status = match lower_cased.as_str() {
            "todo" => Self::ToDo,
            "inprogress" => Self::InProgress,
            "done" => Self::Done,
            _ => return Err("No matching".to_string()),
        };

        Ok(status)
    }
}

impl TryFrom<String> for Status {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let lower_cased = value.to_lowercase();
        let status = match lower_cased.as_str() {
            "todo" => Self::ToDo,
            "inprogress" => Self::InProgress,
            "done" => Self::Done,
            _ => return Err("No matching".to_string()),
        };

        Ok(status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
