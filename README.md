## PAUSE!
I just realized I know nothing. Be right back after reading some books.


## File server
###### Description
This project aims to build simple client server application who's purpose is to perform general file operations remotely similar to ~~S~~FTP. 
I am writing this project to find areas I can improve.

###### Goals
- Test and learn how network communications and file management systems work
- Research and implement desirable codebase as I progress
- Make my raspberry pi valuable 

## TODO
- Terminal IO.
- Update AppSpace to return Error when passed path is not a directory or does not exist.

## Progress Log
#### 10/14
Experimenting with error handling methods, I am not sure which one to stick to. On one hand there is pattern matching and on the other, there are methods (and_then(), or_else(), etc.). Both are good but I feel like pattern matching makes my codebase cluttered while the latter can be hard to read at times. IDK
I also should refrain from using 'unwrap()' if I want to build real-time systems. Just saying 🤷‍♂️
