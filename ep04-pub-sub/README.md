# Pub-sub(Data hub) Pattern
A pattern where consumers are subscribed to a channel, usually via a “hub” object that manages subscriptions.
The hub is usually a passive instance that does not process data. When a producer call its methods, pub/sub logic is 
processed in the producer’s thread.

Pros:

* A clear and flexible project architecture.
* Subscriptions can be managed in a centralized way and may have additional logic, e.g. conditions.

Cons:

* Harder to implement.
* The hub can become a bottleneck if not implemented properly.
* Messages usually need to implement “Clone” trait.