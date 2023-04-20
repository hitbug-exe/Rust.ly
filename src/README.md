# Rust.ly

Rust.ly is a lightweight, high-preformance URL shortener written in Rust. The tool generates a shorter version of a long URL and redirects to the long URL when the short URL is requested. With the help of rocket.rs, this URL shortener provides a reliable and robust web framework for handling HTTP requests and responses. By incorporating an LFU cache, it can store frequently accessed URLs, resulting in faster response times and an overall smoother user experience.

Caching is a commonly used technique in high concurrency systems, and this project showcases the effectiveness of the LFU cache policy in improving performance. The implementation of this cache policy, along with other performance optimizations, makes this URL shortener a great example of how Rust can be used to optimize existing utilities.

# Installation

1. Clone this repository to your local machine using the following command:
 
   `git clone https://www.github.com/hitbug-exe/Rust-ly`

   
2. Navigate to the root directory of the project using the following command:

  `cd Rust-ly`

3. Run cargo build to build the project.

  `cargo build`


# Usage

To run the code, use the command cargo run. This starts the server, which listens on http://localhost:8000.

To store a new URL, use the following curl command:

  `curl --data "url=https://www.example.com" https://localhost:8000/`

This command generates a unique ID for the long URL and returns it in the response. To retrieve the long URL, use the ID in the short URL http://localhost:8000/{ID}. This will redirect to the long URL.

# References

https://github.com/archer884/harsh/

https://rocket.rs/v0.5-rc/guide/

# License

Rust.ly is released under the MIT License. Feel free to use, modify, and distribute it as you see fit.