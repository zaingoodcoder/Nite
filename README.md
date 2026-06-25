NITE

Fast Python project scaffolding built with Rust.

NITE is a lightweight CLI tool that bootstraps a new Python project in seconds. Instead of manually creating folders, initializing Git, and setting up a virtual environment, NITE does it all with a single command.

Features
Creates a new project directory
Creates a Python virtual environment
Initializes a Git repository
Generates a README.md
Generates a .gitignore
Generates a main.py
Fast and lightweight
Written in Rust
Installation
Build from source
git clone https://github.com/<your-username>/nite.git
cd nite
cargo install --path .

After installation, you can use nite from any terminal.

Usage

Create a new project:

nite new my-app

This generates:

my-app/
├── README.md
├── .gitignore
├── main.py
├── venv/
└── .git/
Example
nite new blog

Output:

Creating project...
Creating README.md
Creating .gitignore
Initializing Git...
Creating virtual environment...

Project "blog" created successfully.
Roadmap

Project templates (Flask, FastAPI, Django, Pygame)

Interactive mode

Configuration file

Colored output and progress spinners

Custom project templates

Automatic dependency installation

Publish to crates.io

Contributing

Contributions, feature requests, and bug reports are welcome.

If you'd like to improve NITE, feel free to open an issue or submit a pull request.

License

This project is licensed under the MIT License.
