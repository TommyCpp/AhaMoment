# A copy of toyDB

This is a idea copy of [toyDB](https://github.com/erikgrinaker/toydb). I will implement parts that I am interested in 
but copy paste the existing implementation on other parts to have a working applications.

Why not just fork it?

I want to implement it a simple DB, just for fun but don't have many time to design the architecture.

## Plan
### Build a hybrid KV storage with file backup
The current implementation is in memory only. We can adopt a `Storage` implementation 