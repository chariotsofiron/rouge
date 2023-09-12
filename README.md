# Rouge

A simple, asynchronous Reddit API wrapper.

# Usage

```rust
use rouge::client::{Auth, Client};
use rouge::models::{flatten, me::Me, user::Comments};

let user_agent = "macos:mybot:0.0.1 (by /u/myusername)";
let client = Client::new(user_agent, Auth::None).await?;
let req = Comments { username: "spez".to_string() };
let resp = client.request(req).await.unwrap();
let comments = flatten(resp);
println!("{:?}", comments);
```