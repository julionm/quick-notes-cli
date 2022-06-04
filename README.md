# quick-notes-cli


## what i want it to do

- make possible i can call from anywhere in the terminal `qkn add "Create Issue about something"`  
and it'll be added to my todo list
- i want to consult all entries using `qkn list`
- make possible to remove any item i want passing some index `qkn remove 1`
- remove all of the items `qkn reset`

## steps to create the functionalities

- First things first
  - [ ] test the use of multiple commands
  - [ ] study about file management
  - [ ] improve documentation
  - [ ] make it more readable using iterators
  - [ ] pass the logic to the lib file

- Add
  - I need to check which command was passed
  - Create a file to store these texts
  - Add the second value passed to the file
- Remove
  - Enumerate all lines inside the file using iterators
  - Remove the exact line following the next arg
- Reset
  - Open the file and simply remove all the content inside
