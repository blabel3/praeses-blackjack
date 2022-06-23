# praeses-blackjack-rust
Blackjack cli in Rust


Topics:

    Pre-requisites (commandline ability)
    Installation
    Getting started
    Additional features implemented
    Badges (devops tests, builds, deployments)
    Demo link if that becomes a thing, otherwise, recording of game.


# Praeses Blackjack

*Created in a week for part of the software engineer interview process at [Praeses](https://praeses.com/)* 

Website that lets people generate unique album covers for their spotify playlists. Currently spotify's automatic playlist covers are just some of the album arts stitched together, and the creator has to go and make an image themself if they want something memorable for it. Our app would make an album cover that fits the playlist given the moods of the songs automatically. It will use infortmation from the Spotify API like the lyric keywords, key, bpm, dancability, and song titles to determine images, shapes and colors to use. It will also use image APIs like Unsplash or Artbreeder to find cool images to include in the playlist cover.

# Table of Contents

- [Installing](#Installing)
- [Running](#Running)
- [Contributing](#Contributing)
- [Demo](#Demo)

# Installing

### Pre-Requisites

- This project uses Node: install [Node.js](https://nodejs.org/en/download/) (this will also install npm!)

- Know how to run things on the commandline

Once Node is installed, open up your command line and navigate to this project folder. Node uses npm (the Node Package Manager) to install dependencies. Our dependencies are stuff like Express which lets us run a web server, or Typescript which helps us write better code. To install all the dependencies for our project, run this command:

```console
$ npm install
```

It will make a folder called node_modules with all the code from our dependencies. You only need to run npm install when we add new dependencies or remove old dependencies. If there's an error where it can't find some module, it's a good idea to run `npm install` again.

# Running

All of the scripts necessary to run the project are listed in the [package.json](package.json) file under "scripts". To run the project, we're going to use two of those scripts:

```console
$ npm run build
```

This takes all of our source code in src/ and builds it, converting the typescript into well-optimized javascript for us.

Then, to start the web server, run this command:

```console
$ npm run start
```

This will start the web server, and show a link to where it is running locally. Then all you have to do is click it and you're set, hooray! The terminal window will show any errors on the server, and you can test everything in the browser.

# Contributing

To make edits to the code, first, make sure you're in a new branch! The repo is set up to not allow commits directly on main so that none of us accidentally break everything.

Then you can make your changes! Using `npm run dev` is a good way to test locally, since it will restart the server every time a change is made to the code. This way you don't have to stop the server, build again, and restart the server.

Then, you can make your edits (probably to files in src/ and test/) and commit + push them. Then, on github make a Pull Request to get them merged in.

It'll automatically test the code in the PR, and lets people have a chance to look at the code, add in anything that's missing, etc.

# Demo

Will paste in demo later

