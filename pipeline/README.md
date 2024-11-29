# Pipeline Pattern

A pattern where multiple threads are connected in a chain. Useful in certain cases:
* Data processing is divided into multiple stages.
* Conversion between different channel implementations.
* Conversion between unbounded and bounded channels (e.g. to prevent memory exhaustion, unprocessed data is dropped or processed in a different way).
