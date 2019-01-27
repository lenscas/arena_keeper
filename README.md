# arena_keeper

A simple HTML game build using Rust and Yew with a touch of Javascript.

## Install
1. Follow the instalation instructions from [Yew](https://github.com/DenisKolodin/yew/tree/0.4.0).
2. Install Yarn and Nodejs.
3. Go to the root of the project and run `yarn run gen_species`. This will turn the various files describing the species into rust code.

## Run
Just like how you would run the examples from yew.
You use `cargo web start` to start a server and a watcher that will automatically compile your code on changes.

## Create new species
 Every specie is a folder inside /species containing the following files/folders:
 /images -> A folder containing every image your specie may have.
 names -> A text file containing the various names a character from your specie can get.
 description ->A text file containing 1 line description of your specie.
 
 When you have all the files simply run `yarn run gen_species` again inside the root folder of the project to generate the correct rust code for your specie. 
