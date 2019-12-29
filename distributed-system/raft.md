# Raft Algorithm

Raft algorithm is an algorithm that used to achieve consensus in distributed system. Typically used to keep distributed system in logically right condition.

## State
All node will have following state:
1. `currentTerm`
2. `votedFor`
3. `log[]`
4. `commitIndex`
5. `lastApplied`
6. `state`, which state is the node in

If the `state` is **leader**, then the node should also have following state:
1. `nextIndex[]`
2. `matchIndex[]`

### QA on State
1. Difference between `lastApplied` and `commitIndex`?
`lastApplied` is for stateful machine while `commitIndex` is for Raft algorithm, change of `lastApplied` will only be triggered by node itself. 

2. Difference between `matchIndex` and `nextIndex`
`nextIndex` is the best guess of the index of the entry to be sent. 
`matchIndex` means that if for follower `c`, `matchIndex[c]` is `v`, the leader can make sure that the log between \[1, `v`\] is the **same** for node `c` and leader.

## Events
Through out the Raft Algorithm, there will be five events possible that could change the state of nodes:
1. Election timeout
2. Heart beat timeout(Only for leader)
3. Request vote request recevied
4. Append entries received
5. Add logs request from client

## Common behavior
Those behavior shall be executed whenever an event triggered:
1. If `commitIndex` > `lastApplied`, increase `lastApplied` and apply log\[`lastApplied`\] to the state machine. **Repeat** until `commitIndex` == `lastApplied`
2. If RPC request contains `term` > `currentTerm`(Note that `currentTerm` is for current node), then set `currentTerm` to `term` and convert to follower, reset election timeout

## Event 1
> Election timeout

### For leader node
Not possible, leader's election will never timed out

### For candidate node
Start election like followers did.

### For follower node
Convert to candidate,
Start election:
1. change `status` to candidate
2. increment `currentTerm`
3. set `voteFor` to its index.
4. Reset election timeout
5. send `RequestVote` to every other nodes

`RequestVote` will contains:
1. `term`
2. `candidateId`
3. `lastLogIndex`, the index of last log(Note it is not last *commited* log)
4. `lastLogTerm`, the term of last log(Note it's not last *committed* log)

If recevied `AppendEnties` now, convert to follower, discard all the response.
If get more than half node to grant the vote, then conver to leader

## Event 2
> Heartbeat timeout

### For leader node
Broadcast `AppendEntry` request

`AppendEntry` request contains:
1. `term`
2. `leaderId`
3. `prevLogId`
4. `prevLogTerm`
5. `entries[]`, list of logs that client need to replica
6. `leaderCommit`, commit index of leader

`AppendEntry` response should contains:
1. `term`, the term of target
2. `sueecss`, true if the target contains log with id = `prevLogId` and term = `prevLogTerm`


## Event 3
> Request vote request recevied

### For follower node
1. If the node's `term` > request's `term`, return false.
2. If the node haven't vote for this term, grant vote, return true

### For leader and candidate node
Based on the term to decide.

## Event 4
> Append entries received

### For follower node
Decide what to return 
1. If the node's `term` > request's `term`, return false.
2. If the node does not contain such a log that id = `prevLogId` and term  = `prevLogTerm`

Append entry (must execute following even decided to return false):
1. check if there are existing log that with same index as any log in request. If does have, then **delete the existing log in node and any log following it**.
2. Append new log

Change commit index:
- if `leaderCommit` > `commitIndex` of node, then set `commitIndex` = `min(leaderCommit, index of last new entry)`

