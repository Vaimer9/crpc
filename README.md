<h1 align="center">Command Line Rich Presence🚀</h1>
<div align="center">
<p align="center"><img src="images/Frame 1.svg"></p>
<p align="center">A Blazingly Fast Rich Presence Command Line Application made in rust</p>
</div>

## Installation
1. Install the [binary](https://github.com/Vaimer9/crpc/releases/tag/Production)
2. Run `./crpc init` to initialize the application
3. Go to your config folder and edit `config.json`
4. Run `./crpc --help` to get to know about other commands

## Build Locally
1. Install rustup and all its components
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Clone Repository and build
```sh
git clone https://github.com/Vaimer9/crpc
cd crpc
cargo build --release
```
3. Make your way to the binary in case you are not faminliar with Rust
```
cd target/release
```
4, Run the binary
```
./crpc --help
```

## QuickStart
You just have to run `crpc init` to make the config folder, the code is written to be compatible with both windows and macOS however tests have not been conducted yet. 
To get Rich Presence in your Discord Profile you first need to make a Discord application follow these instructions
1. Visit their [dev portal](https://discord.com/developers/applications)
2. Log In
3. Make a new application with the button at the top right
4. When giving the application a name keep in mind this will be on your profile so choose wisely, but you can always change it later
5. In the general information tab of your application scroll a bit down and copy the application ID of the application
Now that you have your Discord Application set up lets move the next part, which is setting up the `config.json` file.
1. Make your way to the config directories of your OS (Will specify the exact directories soon)
2. Go to crpc directory
3. Open `config.json` in the editor of your choice
4. Paste the application ID in the id slot
5. Here you can customize and tinker with the configuration
6. Once done save the file
7. Then run `crpc` to get the Rich Presence activated


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

