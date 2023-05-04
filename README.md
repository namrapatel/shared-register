# Shared Register

yk.

jk I'll explain, the goal of the Shared Register implementation is to write a small distributed data structure that lives on `n` number of different nodes, has simple read and write functionality, but it should enable multiple readers and multiple writers at the same time, while maintaining a majority of nodes in sync on the state of the register. More specifically, it must adhere to the following properties:

1. Termination: If a correct (does not crash) process invokes an operation on the atomic register, then the operation eventually completes.
2. Validity: If a read is not concurrent with any write, the read operation returns the last value written. A read that is concurrent with a write may return either the last value written, or the value being concurrently written.
3. Ordering: If a read returns a value x1 and a subsequent (possibly concurrent) read returns x2, then the write of x2 does not precede the write of x1.
    - a. That is, reads impose a constraint on one another. Once any process reads a newly written value no other process may ever read an older value.
    - b. This imposes a total read order on the processes. It is effectively placing a globally enforced commit point on a concurrent write — this commit point is the first concurrent read.

Let's explain how each of these requirements is met by the design below.

### Solution design

The general design is a quorum-based protocol that ensures that a majority of nodes are in sync about the state of the Shared Register by not updating the state of the register until a majority of the "network" has agreed to update their state as well. Specifically, the protocol allows clients to call the `write_with_quorum` function, which will send a message to all nodes in the network containing the proposed new state, and wait for an "ACK" response from a majority of nodes—that states that they have updated their register with the value that was sent—before updating the state of the local node's register. This ensures that the register is always in a consistent state, and that the state of the register is always the most recent state that a majority of nodes have agreed upon.

#### Pseudocode

The following is Python-esqe pseudocode that describes the core of the Quorum protocol.

```python
def write_with_quorum(self, value):
    # Initializing variables
    ack_count = 0
    responses = {}
    quorum_size = 0

    # Calculate quorum size
    quorum_size = len(nodes) // 2 + 1

    # Send message to all nodes except local node
    for node in nodes:
        if node != local_node:
            send_message(node, proposed_state)

    # Start timer to wait for ACK responses for 5 seconds
    timer.start(5)

    # Wait for ACK responses from majority of nodes
    while timer.is_running():
        message = receive_message()
        if message.type == "ACK":
            responses[message.node] = message.state
            ack_count += 1
        if ack_count >= quorum_size:
            # Update state of local node's register with proposed new state
            local_register.update(proposed_state)    
            return "ACK"

    # Majority of nodes have not acknowledged the write within timeout period
    return "Error: write was not acknowledged by majority of nodes"

```

### Required improvements

- Write to `TcpStream` instead of making a HTTP call using `reqwest` in the `write_with_quorum` function for consistencys' sake.
- Use serde instead of serializing and deserializing manually. I would've done this to begin with but the fact that I used reqwest was making a few nasty errors pop up, so I decided to just do it manually for the time being.
- There should be more Types, and there should be more pattern matching in several places to ensure that responses we have receieved were of the correct type and format. Currently, successes and errors are not encoded as types, but rather as strings, making it harder to use Rust's sexy pattern matching.

### Personal notes
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