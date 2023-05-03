# Shared Register

yk

#### Notes

- Decided not to use Mutex anywhere because we want read ops that are concurrent with a write operation to be able to read that write.
- Need to use Arc on the AtomicPtr and the AtomicRegister because various operations can have their own threads in the server and need to be able to share the register. 

#### Questions/concerns
- [x] In the quorum protocol, if a specific number of nodes does not ACK the writing of the new state, then the message can be lost (breaks the termination property)
    - Answer: Assume that the network is reliable and there will always be a response from a reasonable amount of nodes.
- [ ] Is it desirable for the quorum to be blocking? This means that incoming requests while a quorum is going on will experience delays, is this desirable?
- [ ] What happens if many nodes start a quorum at once, is there a way this can lead to deadlock?
- [ ] What happens if a node goes down while a quorum is ongoing?
- [ ] Did we handle case where the operation doesn’t terminate because the node it was sent to went down before it was able to process the operation?
- [ ] Not sure how to implement the Ordering in atomics

#### Tests
- Are messages that are sent while a quorum is ongoing buffered?