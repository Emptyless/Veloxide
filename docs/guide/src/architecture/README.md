# Domain Driven Design (DDD)

Domain-driven design (DDD) is an approach to software development that aims to align the software model with the business domain it serves. It involves several key concepts:

- Ubiquitous Language: DDD emphasizes the importance of establishing a common language between the development team and the business stakeholders. This language should be used consistently throughout the project to ensure that everyone is on the same page.

- Bounded Contexts: DDD recognizes that different parts of a software system may have different contexts and requirements. Bounded contexts define the boundaries of a specific part of the system and the language used within it.

- Entities and Value Objects: Entities are objects that have a unique identity and can change over time. Value objects, on the other hand, have no identity and are immutable. Both are important concepts in DDD for representing the business domain.

- Aggregates: Aggregates are a collection of related objects that are treated as a single unit. They are responsible for maintaining consistency and enforcing business rules within a bounded context.

- Domain Events: Domain events represent significant changes in the state of the domain model. They are used to communicate between different parts of the system and can trigger actions in other bounded contexts.

- Repositories: Repositories are used to store and retrieve domain objects from a data store. They abstract away the details of data access and provide a simple interface for the application to interact with.

DDD is a holistic approach to software design that prioritizes understanding the business domain and building a model that reflects it. By using a common language and focusing on the core concepts of the domain, developers can create more maintainable and scalable software systems.

## Domain Driven Design in Veloxide

The concepts of Aggregates, Entities, Value Objects, and Domain Events are all implemented in Veloxide using the [CQRS](https://docs.rs/cqrs-es/latest/cqrs_es/) crate. Further docs on the CQRS crate can be [found here](https://doc.rust-cqrs.org/intro.html).

Repositories are implemented using traits to provide a simple interface for the application to interact with, although this implementation is subject to change.

The concepts of Bounded Contexts and Ubiquitous Language are important concepts to apply when designing a domain model, however are not implemented in Veloxide as there isn't a real domain for the stack itself.

Layered Architecture is detailed in the design patterns section.
