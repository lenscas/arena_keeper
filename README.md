# arena_keeper

A simple HTML game build using Rust and Yew with a touch of Javascript.

## Install
1. Follow the instalation instructions from [Yew](https://github.com/DenisKolodin/yew/tree/0.4.0).
2. Install Yarn and Nodejs.
3. Go to the root of the project and run `yarn run gen species`. This will turn the various files describing the species into rust code.
4. While still in the root of the project also run `yarn run gen routes`. This will create the various files needed to properly track which windows are open. 

## Run
Just like how you would run the examples from yew.
You use `cargo web start` to start a server and a watcher that will automatically compile your code on changes.

## Create new species
 Every specie is a folder inside /species containing the following files/folders:
 /images -> A folder containing every image your specie may have.
 names -> A text file containing the various names a character from your specie can get.
 description ->A text file containing 1 line description of your specie.
 
 When you have all the files simply run `yarn run gen species` again inside the root folder of the project to generate the correct rust code for your specie. 

## Create new windows
 If you want to create a new dragable popup window there is an easy command to get you started. Just run `yarn run gen window $windowName` where `$windowName` is the name of your new window. This will create a new folder in `src/pages/` containing everything you need to start.
 
 The reason its done this way is because Yew does not have an easy way to create container elements. Thus automatically generating the boilerplate needed seemed like a reasonable workaround until a better alternative is found.
