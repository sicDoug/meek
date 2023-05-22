# meek

## Simple streaming ChatGPT CLI written in Rust

This is very much a work in progress.

### Installing
1. Clone this project:
```bash
git clone https://github.com/sicDoug/meek.git
```

2. Enter new the directory:
```bash
cd meek/
```

3. Build from source and install:
```bash
cargo install --path .
```

4. Edit your bashrc file to export your API-key:
```bash
echo OPENAI_API_KEY="<YOUR KEY HERE>" >> ~/.bashrc
```

5. Restart your terminal or run:
```bash
source ~/.bashrc
```

6. Test it out:
```bash
meek Hello there.
```

### Usage

All non-flag arguments will be collected to a prompt:
```bash
meek Teach me how to boil an egg
```

Inclue a file:
```bash
meek Critique my code -i script.js
```

When run for the first time a config file should be created in ~/.meek/
Inside it you can change stuff like model, temprerature and the color of the output text.

The chat history will be used until either cleared (with -c), or the terminal is closed.

Run with --help for full list of commands.
