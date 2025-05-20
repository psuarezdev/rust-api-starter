# Rust Api Starter With Axum

## TODO:
* Add request validation
  ```rs
  use serde::Deserialize;
  use validator::{Validate};
  
  #[derive(Debug, Deserialize, Validate)]
  pub struct LoginRequest {
      #[validate(email(message = "Email must be valid"))]
      pub email: String,
  
      #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
      pub password: String,
  }
  
  #[derive(Debug, Deserialize, Validate)]
  pub struct RegisterRequest {
      #[validate(length(min = 1, message = "First name cannot be empty"))]
      pub first_name: String,
  
      #[validate(length(min = 1, message = "Last name cannot be empty"))]
      pub last_name: String,
  
      #[validate(email(message = "Email must be valid"))]
      pub email: String,
  
      #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
      pub password: String,
  }

  // validator = { version = "0.16", features = ["derive"] }
  /*
  let user = NewUser { ... };
  match user.validate() {
      Ok(_) => println!("Datos válidos"),
      Err(errors) => println!("Errores: {:?}", errors),
  }
  */
  ```
* Improve logs
