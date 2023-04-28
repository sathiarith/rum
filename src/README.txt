For the Rust Universal Machine, I did not have a partner to work with but had the assistance of the TAs.
Will in particular provided an in-depth explanation of the UM and how to plan it out. The gist of the rum
program is the main calling on to the parse() function which takes care of identifying and carrying out the operations.
Halt instruction would be responsible for exiting out of the program. The RUM itself is a struct that holds the registers, 
memory (mapped and unmapped), and a program counter. Vectors were used as the data structures to store the memory and registers. 
In terms of problems, the implementation can read most of the um/umz files the umbin lab package except for codex and advent. 
These would end up stalling on stdin. I tried to use the std::Instant but then released the program forcefully exits by halt. 
I ended up using a physical timer to keep track of the time for sandmark. The time to complete was 4 minutes and 1 second.
I spent about 6-7 hours a day for 6 days on the assignment. 
The initial planning was prior to design documentation submission which was a couple day's work. 
The primary activity that dragged this was debugging. I had greatest difficulty was keeping track of how the registers 
maintained data and mapping to memory. There was a lot of trial and error.