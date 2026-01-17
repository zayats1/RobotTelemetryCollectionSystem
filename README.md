# Robot telemetry collection system

## How to run
The system is in an early development stage, so not suitable for deployments yet.

1. install dependencies:  

        1) rustup
        2) rustup toolchain install stable
        3) cargo install trunk
        4) rustup target add wasm32-unknown-unknown
    and tools  
        1) Bruno,RustRover or Vscode + plugins,some browser

2. clone project
3. open terminal and split it into two tabs
4. first tab  
     ```bash
      cd server
      cargo run
     ```
5. second tab
    ```bash
      cd client_frontend
      trunk serve
     ```
   open link in the browser 

# Contributing
If you have a problem, open a issue. Know, how to fix it open a PR.
And if you are  a teacher, grade it nice, because I use rust, btw!





