#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;
    use rocket::serde::json::json;
    use qirust::helper::generate_svg_string;

    #[test]
    fn test_get_svg() {
        let rocket = rocket();
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/svg?data=test").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string(),
            Some(json!({ "svg": generate_svg_string("test") }).to_string())
        )
    }

    #[test]
    fn test_get_svg_post() {
        let rocket = rocket();
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client
            .post("/svg")
            .header(ContentType::JSON)
            .body(json!({ "content": "test" }).to_string())
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string(),
            Some(json!({ "svg": generate_svg_string("test") }).to_string())
        )
    }
}
