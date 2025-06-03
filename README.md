
![img.png](gh_media/main-image.png)
# Development

Install [Rust](https://www.rust-lang.org/tools/install)
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | s
```

Clone project
```sh
git clone https://github.com/valaises/rust-chat.git
cd rust-chat
```

Serve project for web
```bash
dx serve --platform web --package core --port 8008
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

