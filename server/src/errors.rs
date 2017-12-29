#![allow(missing_docs)]

use iron::error::HttpError;

error_chain! {
    errors {
        CouldntStartServer {
            description("Failed to start server")
        }
        InvalidEnvVar(var: &'static str) {
            description("Invalid environment variable value")
            display("The environment variable {} had an invalid or missing value.", var)
        }
    }
    foreign_links {
        Http(HttpError);
    }
}
