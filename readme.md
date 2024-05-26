# Draft Master

## Table of Contents

- [Introduction](#introduction)
- [Project Structure](#project-structure)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
  - [Controls](#controls)
  - [Statistics](#statistics)
- [Animation Assets](#animation-assets)
- [Algorithm](#algorithm)
- [Physics and Vehicle Dynamics](#physics-and-vehicle-dynamics)
- [Optional Features](#optional-features)
- [Authors](#authors)
- [Contributing](#contributing)
- [License](#license)

## Introduction
**DraftMaster** is a simple and efficient technical drawing software designed to facilitate the creation of technical drawings and the management of widgets in an SDL2 interface. This project uses SDL2 to provide an interactive user interface with buttons for various drawing actions.

## Fonctionnalités
* **Integrated Widgets** : Buttons for drawing lines, polylines, axes, rectangles, polygons, points, and circles.

* **Dual Viewport** : Separation between the drawing area and the widget area for better organization.

* **Event Handling** : Detection of mouse clicks and key presses for dynamic interactions.

## Requirements

- Rust programming language
- SDL2 library for Rust

## Installation

1. Ensure you have Rust installed on your system. If not, download and install it from [here](https://www.rust-lang.org/tools/install).
2. Install the SDL2 library. Instructions can be found [here](https://github.com/Rust-SDL2/rust-sdl2#user-content-requirements).

Clone this repository and navigate into the project directory:

```sh
git clone https://github.com/mouhasup/DraftMaster.git
cd DraftMaster
```

## Build the project:

```sh
cargo build
```

## Usage

Run the program:

```sh
cargo run
```

## Project Structure

The project is organized into multiple files for better structure:

**main.rs**

The main file initializes SDL2, creates the window and viewports, and handles the main rendering and event loop

**events.rs**

This module handles SDL2 events, particularly interactions with buttons and the drawing area.

**buttons.rs**

This module contains the Button structure and functions to create and manage buttons.


## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your improvements.

## Authors

- Mouhameth Alioune Diouf
-
-

## License

This project is licensed under the **[M.A.D]** License.

**_Feel free to explore, modify, and enhance the project as you see fit. Happy coding !!!_**
