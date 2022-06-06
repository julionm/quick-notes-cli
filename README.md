# quick-notes-cli (Qkn)


## how to run

- Clone this repo // Download the code
- Install cargo on your OS
- Run `cargo build --release`
- Run `cargo install --path .` on the root folder

## what i want it to do

- make possible i can call from anywhere in the terminal `qkn add "Create Issue about something"`  
and it'll be added to my todo list
- i want to consult all entries using `qkn list`
- make possible to remove any item i want passing some index `qkn remove 1`
- remove all of the items `qkn reset`

## steps to create the functionalities

- First things first
  - [x] test the use of multiple commands
  - [x] study about file management
  - [ ] improve documentation
  - [ ] make it more readable using iterators
  - [x] pass the logic to the lib file
  - [ ] Write the `help` command text
  - [x] Fix the `remove` command to receive numbers greater than 9

- Issues
  - [x] Change the creation path for the file

- Optimizations & Improvements
  - [ ] Create an abstraction to use the file better - *DEPRECATED*
  - [x] Remove all warnings from the compiler

- [x] Add
  - I need to check which command was passed
  - Create a file to store these texts
  - Add the second value passed to the file
- [x] List
  - List all notes with numbers to help when removing
- [x] Remove
  - Enumerate all lines inside the file using iterators
  - Remove the exact line following the next arg
- [ ] Reset
  - Open the file and simply remove all the content inside
