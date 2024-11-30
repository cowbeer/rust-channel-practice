# Fan-in / Fan-out Pattern

A pattern where producers are sending data to a single channel (fan-in). After that, a single or multiple consumers collect 
data from the channel (fan-out).

Good for parallel processing and data aggregation for a single operation. The channel is used as a common result storage. 
Probably the only pattern where unbounded channels are always safe to use as the number of tasks is strictly defined.

A consumer can either wait for all producers to finish or start receiving data immediately. After the producers are finished, 
their threads are stopped.

