# Asteroids

This is a reboot of the famous video game from 1979. The goal of this project is to learn programming in different languages and having fun.

<img src="./assets/asteroids-0.png" width="600">
<img src="./assets/asteroids-1.png" width="600">

So far, there are 2 versions of it. One written in TypeScript with p5js (https://p5js.org) while the other is in Java with Processing (https://processing.org).
I'm planning to write another version in Rust and draw directly to the canvas instead of using an external library like Processing or p5js. As I only started learning Rust recently, this could take some time :-).

Test the game at: https://olimungo.github.io/asteroids/

## TypeScript

### Requirements

-   node and npm

### Setup

Clone this project, move the the _typescript_ folder and do:

```sh
npm i
```

### Run locally

```sh
npm start
```

## Java

### Requirements

-   install Processing: https://processing.org/download

On macOS, you can install it with brew:

```sh
brew install --cask processing
```

### Run locally

You can either start Processing and open the sketch _Asteroids.pde_ located into the _java/Asteroids_ folder or install the _processing-java_ command-line.

In order to install the _processing-java_ command, select the item "Install processing-java" in the _Tools_ menu from the Processing IDE.

To execute the sketch using the Processing IDE, just click on the big "Play" button.

To start it at the command-line, use the following command at the root of this project:

```sh
processing-java --force --sketch=`pwd`/java/Asteroids --output=`pwd`/java/Asteroids/out --run
```

To start it in VS Code, use the keyboard shortcut CMD-SHIFT-b on macOS or CTRL-SHIFT-b on Windows and Linux.

Have fun!
