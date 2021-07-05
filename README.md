# A JSON completeness checker for Rust

This project is a personal playground for coming up with a solution for the following problem:
* A client is developed against an API, possibly a third-party one
* The API changes - and more data is added
* Without an update to the API, data gets discarded and the client will never know that the API has changed

Typically the APIs speak JSON, and an object mapper is used, like `serde-json`.
This create already protects against the first case, of having a class that takes in more data than is provided by the API.
However, there is no alerting when the case above happens.

I aspire to turn this code into a plugin, if only for the sake of learning.
