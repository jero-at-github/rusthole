# Rusthole
Application to send files between computers implemented in Rust.

# Motivation
This application is inspired by the fantastic application "Magic Wormhole".
The intention is to create a similar but very basic application implemented in Rust for learning purposes.

# Structure
The project compiles into: 
- A client binary, used to send and received files between 2 computers in a direct connection (P2P).
- A synchronization server, which mission is to match a sender and a receiver through a secret phrase.

# Crates
Following crates have been used in the project:
- tokio
- clap
- serde_json
- rand

# Deployment usage
1) Executes the server: `server.exe`
2) Executes the client in sender mode: `rusthole.exe send ./my_folder/file_to_send.extension`
3) Executes the client in receiver mode: `rusthole.exe receive [secret_phrase]`
