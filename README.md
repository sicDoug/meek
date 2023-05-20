# meek

## ChatGPT CLI that streams the response written in Rust

### Please note that this is a work in progress

### Installing
1. Clone this project
```
git clone https://github.com/sicDoug/meek.git
```

2. Enter the directory
```
cd meek/
```

3. Build from source
```
cargo build --release
```

4. Extract the executable
```
sudo mv target/release/meek /usr/local/bin/
```

5. Delete the cloned directory if you want
```
rm -rf ../meek/
```

6. Edit your bashrc file to export your API-key
```
echo OPENAI_API_KEY="<YOUR KEY HERE>" >> ~/.bashrc
```

7. Restart your terminal or run
```
source ~/.bashrc
```

8. Test it out
```
meek Hello there.
```

To update just repeat steps 1-4
