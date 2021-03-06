<div id="top"></div>

<!-- VARIABLES TO GLOBAL SEARCH + REPLACE
- snake                                      // repo_name
- Snake with Rust & WASM                     // project_title
- A starter template for a Rust CLI program  // project_description
- MylesWritesCode                            // github_username
- MylesCodesEmoji                            // twitter_handle
- myles-berueda                              // linkedin_username
- me@themapletree.io                         // email
-->

<!-- [![Forks][forks-shield]][forks-url] -->
<!-- [![Stargazers][stars-shield]][stars-url] -->
<!-- [![Issues][issues-shield]][issues-url] -->

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/MylesWritesCode/snake">
    <img src=".meta/logo.png" alt="Logo">
  </a>

<h3 align="center">Snake with Rust & WASM</h3>

  <p align="center">
    A starter template for a Rust CLI program
    <br />
    <a href="https://github.com/MylesWritesCode/snake"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/MylesWritesCode/snake">View Demo</a>
    ·
    <a href="https://github.com/MylesWritesCode/snake/issues">Report Bug</a>
    ·
    <a href="https://github.com/MylesWritesCode/snake/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

[![Product Name Screen Shot][product-screenshot]](https://example.com)

This is the famous snake game with the game logic built in Rust and compiled to
WASM via [wasm-pack](https://github.com/rustwasm/wasm-pack). The web rendering
implementation will depend on the branch that you're looking at with the
differences between the two listed below.

<p align="right">(<a href="#top">back to top</a>)</p>

### Built With

- [Rust](https://rust-lang.org)
- [Remix](https://remix.run/)
- [Turborepo](https://turborepo.org/)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

Install the pre-requisites linked below then follow the instrucations in usage.

### Prerequisites

- [Rust](https://rust-lang.org)
- [NodeJS](https://nodejs.org/en/)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->

## Usage

1. Clone the repo

```
git clone https://github.com/MylesWritesCode/snake.git
```

2. Install deps

```
yarn install
```

3. Build the project

```
yarn build
```

4. Run the server

```
yarn dev
# or
yarn start
```

### Only building the WASM

1. `cd` into `packages/snake-game`

```
# This totally works 👍
wasm-pack build --target web
```

## Branches

- `main` should be the JS implementation, where Rust and WASM is used for
  everything except for presentation, which will be taken care of on the JS side.
- `with-mostly-rust` is the Rust implementation, where I use mostly Rust for both
  the game logic and the rendering implementation.

<p align="right">(<a href="#top">back to top</a>)</p>

## Rust + WASM + React notes

- Rust + WASM should take care of game state including ticks. I'm thinking that
  we create an array of enums that map to each grid element.

  ```
  enum PositionType {
    Food,
    Snake
  }
  ```

- When a position is updated, we send observable data to the frontend with the
  index and the enum. We assume a PositionType of None all squares in the grid.
  ```
  struct BoardPositionEvent {
    position: usize,
    variant: Option<PositionType>
  }
  ```
- In theory, we only need to send three of these structs over, containing the
  head, tail, and food.
- On the React side, we should listen for these updates from WASM. When we get an
  index, we update that position using the enum type to either color the block,
  or display food "🍕".
- The React side should also listen for `has_lost`, which will emit when we have
  a lost game state.
- Screw it, we'll let Rust also take care of the movement by listening to keydown
  events.

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

**Myles Berueda**

[![Twitter][twitter-shield]][twitter-url]

[![LinkedIn][linkedin-shield]][linkedin-url]

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->

<!-- ## Acknowledgments -->

<!-- - []() -->
<!-- - []() -->
<!-- - []() -->

<!-- <p align="right">(<a href="#top">back to top</a>)</p> -->

<!-- MARKDOWN LINKS & IMAGES -->

[contributors-shield]: https://img.shields.io/github/contributors/MylesWritesCode/snake.svg?style=for-the-badge
[contributors-url]: https://github.com/MylesWritesCode/snake/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/MylesWritesCode/snake.svg?style=for-the-badge
[forks-url]: https://github.com/MylesWritesCode/snake/network/members
[stars-shield]: https://img.shields.io/github/stars/MylesWritesCode/snake.svg?style=for-the-badge
[stars-url]: https://github.com/MylesWritesCode/snake/stargazers
[issues-shield]: https://img.shields.io/github/issues/MylesWritesCode/snake.svg?style=for-the-badge
[issues-url]: https://github.com/MylesWritesCode/snake/issues
[license-shield]: https://img.shields.io/github/license/MylesWritesCode/snake.svg?style=for-the-badge
[license-url]: https://github.com/MylesWritesCode/snake/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/myles-berueda
[twitter-shield]: https://img.shields.io/twitter/follow/MylesCodesEmoji?style=for-the-badge
[twitter-url]: https://twitter.com/MylesCodesEmoji
[product-screenshot]: .meta/screenshot.png
