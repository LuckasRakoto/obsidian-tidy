# Advantages
- Opacity: hide details
- Shared resources
- Openness: Increased flexibility
- etc.
# Disadvanages
- Complexity 
- Security has to be applied to all services
# Client-Server
Server: provides the functionality (service) and a client
Client: Makes the requests and the server responds

The server can serve multiple clients
The communication occurs over a network
Client subtypes:
- Thin-client: Graphical interface, no logic (e.g. browsers)
- Thick-client: Graphical interface and part of the logic (e.g. most web apps)

# Multitier
Layer architecture applied to distributed systems
## Advantages
- Modules are reusable
- Scalable, flexible, maintainable
## Disadvantages
- Complexity
- Performance
- Cost

# Primary - secondary
Two level architecture but all roles are different
*Analytical tasks are replicated*
**Primary**: responsible for dividing the task and distributing the parts to secondaries
**Secondary**: Execute the task on parts of the data.
-> In the end, primary assembles the result and saves them.
E.g. MapReduce, Distributed databases
## Advantages
- Parallelization increases efficiency
- Primary increases the reliability of the system
- Result is guaranteed
## Disadvantages
- Heavily dependent on the primary
- Secondaries are isolated, risk of redundancy
- Needs lost of computation capacity
- Asynchronous replication fails at times
# P2P
Each nodes act like both the client and the server, direct communication between nodes without the need for a 3rd party (central coordinator)
## Advantages
- Robust against failures of participants
- Scalable
- Cost effective
## Disadvantages
- Security
- Integrity - hard to track changes across all peers
# Broker
Similar to [[6- Blackboard | Blackboard]]
![[Pasted image 20250908104138.png]]
## Advantages
- Easy to add, modify or remove servers
- server and client decoupled
- distribution is opaque to the developper
## Disadventages
- Increase complexity
- Error handling overhead
- Bottleneck
## Broker vs blackboard

| Broker                                                   | Blackboard                                          |
| -------------------------------------------------------- | --------------------------------------------------- |
| - Distributed accross multiple nodes, more scalable      | - Centralized, more secure                          |
| - Less secure                                            | - shared knowledge base                             |
| - Facilitates decoupled communication between components | - ideal for complex problems in data-driven systems |
# Service-oriented (SOA)
Everything is a service, they're autonomous and atomic softwares
## Advantages
## Disadvantages
# Event-driven architecture
**Producers**: generate stream of events
**Consumers**: listen to the events
## Advantages
- Highly scalable and distributed
- Responsiveness
- Modules are not responsible for delivering the messages
## Disadvantages
- No guarantee
- Coordinating messages accross services can be hard
- Processing the events in order