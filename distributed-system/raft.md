# Raft Algorithm

Raft algorithm is an algorithm that used to achieve consensus in distributed system. Typically used to keep distributed system in logically right condition.

## Leader Election
Raft algorithm assume that the distributed system is in leader-follower mode.

For each node in system, we define them can be in following three state
* Follower
* Leader
* Candidate
