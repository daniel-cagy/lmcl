# LMCL
![Rust](https://img.shields.io/badge/language-Rust-orange)
![Build](https://img.shields.io/badge/build-cargo-darkgreen)
![License](https://img.shields.io/badge/license-MIT-red)
![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Status](https://img.shields.io/badge/status-unstable-black)


LMCL (Lite Markdown Coding Language) is a simple markdown language that compiles to HTML. Its compiler, called 'hallo', is being developed in Rust.

My goal with this project was to create a markdown language that's more familiar to programming languages in general. It's also a learning project to explore Rust, and how to manipulate strings and files.

This is an early-stage version of the 'hallo' compiler, and syntax changes might occur.

## Code example
- LMCL code:
```
// This is a comment
// Declaring an h1 element
let title main_title = "This is an h1 title!";
// Placing an h2 subtitle
place subtitle = "This is an h2 subtitle!";
// Storing content
store info = "Welcome!";
// Reusing it in a tag
let paragraph intro = info;
```
- HTML code:
```html
<!DOCTYPE html>
<html>
<head><meta charset="UTF-8"></head>
<body>
<h1 id="main_title">This is an h1 title!</h1>
<h2>This is an h2 subtitle!</h2>
<p id="intro">Welcome!</p>
</body>
</html>
```

## Syntax
### Elements:
In LMCL, you can place elements in the HTML page, or store it to use later in an attribution, ending the line with a ';'. All values are resolved at compile time, meaning there's no JavaScript involved yet. In this version, you can only attribute an element to a string or the value of another element already stored.
### Tag and class
In the current version of LMCL, Tags and classes are used to place elements in the HTML code. For placing elements, tags are mandatory, while classes are optional. There are 8 Tags available in this version:
- 'paragraph', or 'p'
- 'title', or 'h1'
- 'subtitle', or 'h2'
- 'h3'
- 'h4'
- 'h5'
- 'h6'
- 'div'

Using a tag that doesn't exist or isn't available in this version is invalid and will raise an error. Classes, however, can be created freely.
### id
In the current version of LMCL, ids are used to store elements. You can create ids freely, but you cannot use the same id more than once.
### Commands
#### Comments
You can comment your code by using "//" at the beginning of a line, which is ignored by the compiler. Here's an example:
```
// This is a comment
```
#### let
The 'let' command is used to declare an element, therefore placing the element in the HTML page and storing it. You can declare an element by entering:
```
// If it has tag and class:
let tag.class id = "content";
```
```
// Or, if it only has a tag:
let tag id = "content";
```
With 'let', the element can be seen in the HTML page, and can also be atributed to other elements:
```
// id2 has same value or content as id1
let tag id1 = "content";
let tag id2 = id1;
```
#### place
The 'place' command is used only to place the element in the HTML page. Since the element isn't stored, it doesn't have an Id. You can place an element by entering:
```
// If it has tag and class:
place tag.class = "content";
```
```
// Or, if it only has a tag:
place tag = "content";
```
#### store
The 'store' command is used to only store the string to use it later. Since it isn't placed on the HTML file, it doesn't have a tag or class. You can store an element by entering:
```
// Stored values can later be used by referencing their id
store id = "content";
place tag = id;
```
## Requirements
Make sure you have the Rust compiler installed on your system. To download the latest version, go to https://www.rust-lang.org/learn/get-started, and install the Rustup tool.

## Installation
The simplest method to install the project is to, once you're in the repository page:
- Click on the green "Code" button, then choose "Download ZIP"
- After downloading, extract the ZIP file on your computer
- Open the extracted folder in your terminal

You can also use Git to clone the project, if you're already familiar with it.

## Running the project
Now, you can easily run the compiler by entering in the terminal:
```
cargo run -- file
```
This will compile the file.lmcl to file.html in the same directory. If your '.lmcl' file is located outside the project folder, include the full or relative path.

## Contact
If you experience any problems using the program, feel free to contact me by my email (cagydaniel@gmail.com).

## Future releases
I'm planning to implement for the next versions some improvements such as:
- Support to simple and static logic operations
- Support to linking to CSS and JavaScript files
- Support for other tags and data types

## License
This project is licensed under the MIT License. See the [License](LICENSE.txt) file for more details.