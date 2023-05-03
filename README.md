# Shared Register

yk

#### Notes

- Decided not to use Mutex anywhere because we want read ops that are concurrent with a write operation to be able to read that write.
- Need to use Arc on the AtomicPtr and the AtomicRegister because various operations can have their own threads in the server and need to be able to share the register. 

#### Questions/concerns
- [x] In the quorum protocol, if a specific number of nodes does not ACK the writing of the new state, then the message can be lost (breaks the termination property)
    - Answer: Assume that the network is reliable and there will always be a response from a reasonable amount of nodes.
- [x] Is it desirable for the quorum to be blocking? This means that incoming requests while a quorum is going on will experience delays, is this desirable?
    - If the messages are buffered (not lost) then this may not be a problem, this is kind of just normal network latency that is even accepted by RAFT
- [x] Did we handle case where the operation doesn’t terminate because the node it was sent to went down before it was able to start the quorum?
    - I think this is handled by the fact that the node will not be able to respond to the client and the client will timeout and retry the operation
- [x] What happens if a node goes down while a quorum is ongoing?
    - In this case, the other nodes will have updated their values, but the client will not recieve a response so it may try to resend the operation. This is fine because the operation is idempotent, meaning that it will not change the state of the register if it is applied multiple times.
- [ ] What happens if many nodes start a quorum at once, is there a way this can lead to deadlock or split vote?
    - If multiple nodes start a quorum, then the there will still be an order of commits. Its possible that one commit very quickly overwrites another commit, but thats fine according to the requirements.
- [ ] Not sure how to implement the Ordering in atomics

#### Tests
- [ ] Do servers correctly handle messages from clients?
- [ ] Do servers correctly send messages to each other? i.e. Does the quorum protocol work?
- [ ] Can the register handle concurrent read and writes?
- [ ] Are messages that are sent while a quorum is ongoing buffered? 
- [ ] Are the buffered messages sent after the quorum is done?