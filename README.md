# Ngrok Tunnel Checker

Ngrok Tunnel Checker is a command-line tool to check if Ngrok tunnels are running. It utilizes the Ngrok API to retrieve information about active tunnels and displays them.

## Installation and Usage

### Prerequisites

- Rust programming language installed. You can install Rust from [here](https://www.rust-lang.org/tools/install).
- Ngrok installed and running locally. You can download Ngrok from [here](https://ngrok.com/download) and follow the installation instructions.

### Windows

1. **Install Rust:**
   - Download and install Rust from the official website: [rust-lang.org](https://www.rust-lang.org/tools/install).

2. **Build the Program:**
   - Open a terminal and navigate to the project directory.
   - Run the following commands:
     ```sh
     cargo build --release
     ```

3. **Run the Program:**
   - After building the program, navigate to the target directory:
     ```sh
     cd target\release
     ```
   - Run the executable:
     ```sh
     .\ngrok-tunnel-checker.exe
     ```

### MacOS

1. **Install Rust:**
   - Open a terminal and install Rust using Homebrew:
     ```sh
     brew install rustup
     rustup-init
     ```

2. **Build the Program:**
   - Clone the repository and navigate to the project directory.
   ```sh
   git clone https://github.com/LightJkd/Ngrok-Tunnel-Checker.git
   cd Ngrok-Tunnel-Checker
   ```
   - Run the following commands:
     ```sh
     cargo build --release
     ```

3. **Run the Program:**
   - After building the program, navigate to the target directory:
     ```sh
     cd target/release
     ```
   - Run the executable:
     ```sh
     ./ngrok-tunnel-checker
     ```

### Linux

1. **Install Rust:**
   - Open a terminal and install Rust:
     ```sh
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     source $HOME/.cargo/env
     ```

2. **Build the Program:**
   - Clone the repository and navigate to the project directory.
```sh
   git clone https://github.com/LightJkd/Ngrok-Tunnel-Checker.git
   cd Ngrok-Tunnel-Checker
   ```
   - Run the following commands:
     ```sh
     cargo build --release
     ```

3. **Run the Program:**
   - After building the program, navigate to the target directory:
     ```sh
     cd target/release
     ```
   - Run the executable:
     ```sh
     ./ngrok-tunnel-checker
     ```

## How It Works

The program sends a request to the Ngrok API endpoint (`http://127.0.0.1:4040/api/tunnels`) to retrieve information about active tunnels. It then parses the JSON response and displays the tunnels found.

### Code Snippet

```rust
async fn get_ngrok_tunnels() -> Result<Vec<NgrokTunnel>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:4040/api/tunnels")
        .send()
        .await?
        .json::<NgrokApiResponse>()
        .await?;
    Ok(response.tunnels)
}

```

### License
* This project is licensed under the MIT License.”


