# Shared Register

yk

### Design



#### Improvements

- Use TcpStream instead of Reqwest in the `write_with_quorum` function for consistencies sake
- Use serde instead of serializing and deserializing manually. I would've done this to begin with but the fact that I used reqwest was making a few nasty errors pop up, so I decided to just do it manually for the time being.
- There should be more Types, and there should be more pattern matching in several places to ensure that responses we have receieved were of the correct type and format. Currently, successes and errors are not encoded as types, but rather as strings, making it harder to use Rust's sexy pattern matching.

#### Questions/concerns
- [x] In the quorum protocol, if a specific number of nodes does not ACK the writing of the new state, then the message can be lost (breaks the termination property)
    - Answer: Assume that the network is reliable and there will always be a response from a reasonable amount of nodes.
- [x] Is it desirable for the quorum to be blocking? This means that incoming requests while a quorum is going on will experience delays, is this desirable?
    - If the messages are buffered (not lost) then this may not be a problem, this is kind of just normal network latency that is even accepted by RAFT
- [x] Did we handle case where the operation doesn’t terminate because the node it was sent to went down before it was able to start the quorum?
    - I think this is handled by the fact that the node will not be able to respond to the client and the client will timeout and retry the operation
- [x] What happens if a node goes down while a quorum is ongoing?
    - In this case, the other nodes will have updated their values, but the client will not recieve a response so it may try to resend the operation. This is fine because the operation is idempotent, meaning that it will not change the state of the register if it is applied multiple times.
- [x] What happens if many nodes start a quorum at once, is there a way this can lead to deadlock or split vote?
    - If multiple nodes start a quorum, then the there will still be an order of commits. Its possible that one commit very quickly overwrites another commit, but thats fine according to the requirements.
- [ ] Not sure how to use the Ordering stuff with AtomicPtr.

#### Planned tests
- [x] Do servers correctly handle messages to their HTTP API? 
- [x] Do servers correctly send messages to each other? i.e. Does the quorum protocol work?
- [ ] Can the register handle concurrent read and writes?
    - Not testable, prove this using a description.
- [ ] Are messages that are sent while a quorum is ongoing buffered? 
- [ ] Are the buffered messages sent after the quorum is done?


#### Notes

- Decided not to use Mutex anywhere because we want read ops that are concurrent with a write operation to be able to read that write.
- Need to use Arc on the AtomicPtr and the AtomicRegister because various operations can have their own threads in the server and need to be able to share the register. 