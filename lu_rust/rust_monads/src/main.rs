#[derive(Debug)]
struct User {
    id: i32,
    name : String, 
}

fn find_user(id : i32) -> Option<User>{
    if id == 1{
        Some(User{id: 1, name: "John".to_string()})
    } else {
        None
    }
}

fn get_company(user: &User) -> Option<String> {
    if user.id == 1 {
        Some(String::from("Acme Corp"))
    } else {
        None
    }
}

fn main() {
    let user_company = find_user(1)
        .as_ref()
        .and_then(get_company);

    match user_company {
        Some(company) => println!("User works at: {}", company),
        None => println!("Company not found"),
    }

    #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_user() {
        assert!(find_user(1).is_some());
        assert!(find_user(2).is_none());
    }

    #[test]
    fn test_get_company() {
        let user = User { id: 1, name: String::from("Alice") };
        assert_eq!(get_company(&user), Some(String::from("Acme Corp")));
    }

    #[test]
    fn test_monad_chain() {
        let result = find_user(1)
            .as_ref()
            .and_then(get_company);
        assert_eq!(result, Some(String::from("Acme Corp")));

        let result = find_user(2)
            .as_ref()
            .and_then(get_company);
        assert_eq!(result, None);
    }
}

}
