# WARQUEST

Links:
* https://www.reddit.com/r/roguelikedev/ seems to be centered around libtcod
* There are rust bindings for libtcod https://tomassedovic.github.io/roguelike-tutorial/part-1-graphics.html
* Game programming patterns: https://gameprogrammingpatterns.com/introduction.html
* Another tutorial, using RLTK: https://bfnightly.bracketproductions.com/chapter_1.html
* Rust Rogue Like Toolkit: https://github.com/amethyst/bracket-lib
* Well regarded tutorial in Java: https://trystans.blogspot.com/2016/01/roguelike-tutorial-00-table-of-contents.html?m=1
* Rust terminal library: https://github.com/redox-os/termion
* Another, possibly more featureful: https://github.com/crossterm-rs/crossterm
* Rexpaint: For drawing ascii stuff: https://www.gridsagegames.com/rexpaint/downloads.html
* Ansi colors: https://www.ditig.com/publications/256-colors-cheat-sheet

Going with Crossterm for now.

* Roguelike tutorial in rust, own library: https://www.gridbugs.org/roguelike-tutorial-2020-part-1/
* Useful unicode: https://github.com/globalcitizen/zomia/blob/master/USEFUL-UNICODE.md


TODO:
  - [x] Get a character on the screen.
  - [x] Move character around the screen.
  - [x] Implement a world structure.
  - [x] Create bounds for character movement.
  - [x] Use cardinal directions for movement function.
  - [x] BUILD A WALL
  - [x] Set static world size
  - [x] Unit test character movement
  - [x] Read world from file
  - [x] Colored/Solid tiles
  - [ ] Better error handling with terminal cleanup
  - [x] Reserved Space for UI
  - [x] Print messages to screen in the UI
