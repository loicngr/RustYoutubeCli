# RustYoutubeCli
A little Twitch cli (Command-line interface) app, write in Rust 

### Env vars (.env)

#### Add this lines in your .env file :
    YOUTUBE_API_KEY=

### Get user last video
- > cargo run lastvideo-user={channel_id}

### Windows Build
- > cargo build --release --target=x86_64-pc-windows-msvc