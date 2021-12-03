<h1 align="center">Command Line Rich Presence🚀</h1>
<p align="center"><img src="images/Frame 1.svg"></p>
<p>
    A Blazingly Fast Rich Presence Command Line Application made in rust
</p>

<details>
<summary>Installation</summary>

1. Install the [binary](https://github.com/Vaimer9/crpc/releases/tag/Production)
2. Run `$ ./crpc init` to initialize the application
3. Go to your config folder and edit `config.json`
4. Run `$ ./crpc --help` to get to know about other commands
</details>

<details>
<summary>Build Locally</summary>

1. Install Rustc
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Clone and Build
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/Vaimer9/crpc
cd crpc
cargo build --release
```
</details>

### JSON Example
```json
{
	"id": "",
	"status": "",
	"details": "",
	"large": true,
	"small": false,
	"want_buttons": false,
	"button_numbers": 1,
	"large_image": "",
	"small_image": "",
	"large_tool": "",
	"small_tool": "",
	"buttons": ["", "", "", ""]
}
```
I have tried my best to keep the JSON as simple as possible.
