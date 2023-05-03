# Shared Register

yk


#### Notes

- Decided not to use Mutex anywhere because we want read ops that are concurrent with a write operation to be able to read that write.
- Need to use Arc on the AtomicPtr and the AtomicRegister because various operations can have their own threads in the server and need to be able to share the register. 