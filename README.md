I'll make things simple.
The server listens and reads requests. The requests can either be Get, List, or Put. I will not implement fancy features such as integers, strings, or arrays in byte format for the program to send over the network.  
## Progress Log
#### 10/14
Experimenting with error handling methods, I am not sure which one to stick to. On one hand there is pattern matching and methods (and_then(), or_else(), etc.) on the other. I also should also refrain from using 'unwrap()' if I want to build real-time systems. Just saying 🤷‍♂️
