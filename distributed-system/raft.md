# Raft Algorithm

Raft algorithm is an algorithm that used to achieve consensus in distributed system. Typically used to keep distributed system in logically right condition.

## Leader Election
Raft algorithm assume that the distributed system is in leader-follower mode.

For each node in system, we define them can be in following three state
* Follower
* Leader
* Candidate

### From Follwer to Candidate
All of nodes will start as follower state. If a follower did not hear from leader for a while, they will become a candidate. The time for a node to wait before become a candidate is called **Election timeout**. 

### When become Candidate
When a follower become a candidate, it will starts a new **election term**(term++), and vote for itself, and then send vote request to other nodes.

if other node haven't voted in this term, they will vote for whoever request votes first and then reset its **Election timeout**

Whoever get majority vote will become Leader.
