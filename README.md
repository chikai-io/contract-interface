# Near Contract Interface

## Goal

Exploration and research intending to enable usage of generic types and traits in contract usage/composition on the [NEAR](https://near.org/) ecossystem. 

In general, a contract has _methods_ that are _exposed_ accordingly to a trait (or a combination thereof), and is implemented for a _state_ structure, which can change in reaction to those methods calls.

Being able to compose the state (and to re-implement the traits in those compositions) could facilitate contract code sharing and the customization of such contracts, all of which could help with contract standards implementations.

With this goal in mind, the current approach of this project is to try having additional traits and generic structures to indicate data/workflow intent, even if macros are still being used to do the traits implementations.

## Function Exposure

One major item of consideration is the function exposure.  
Given the nature of the type information after the binary compilation, it's required that all generic types must be known at compile-time. That is, all generics must be specialized _at that point_ (when the contract is effectively exposing C-like functions to the runtime).  
This means that the trait/methods _definitions_ must _be able to_ be completely separated from the actual _exposure_ of those methods. A prime example would be a library that only defines traits and structures, where the users of such library would make their compositions/customizations and then make the actual exposure of their selected methods.  
In other words, function exposure will potentially need to be hand-rolled, instead of being generated at the trait definition - and this is worth taking a note.

### Builder-Pattern

A little tangent to the approach of increasing the usage of generics is the move towards a builder-pattern interface of contracts call, which have an option to be completely generic over all contracts - this, and the option of having the builder-pattern built from macros has been explored as well (note: macros changes which are currently not implemented!).

### Named Arguments

One major feature for composability that currently exists in `near_sdk` is the usage of `serde_json` for arguments/result parsing. Depending on how serde attributes are set, the methods can, for example, still receive and ignore extra arguments. So this increases the chances of extensions for messaging, although, conservely, unintended behaviour could be triggered if specific extra arguments are passed/received.

## Defined Items

This uses a distintic naming to differentiate contract method _callers_ from contract method _exposers._ Or _clients_ vs _servers._ Currently, structures that clients use are generally called `CallOut`, and those that servers use are called `CallIn`, but this could be improved.

Still on this subject of clients vs servers, it's worth clarifying how is the dataflow for each one's perspective.  
The client doesn't directly communicate with the server contract, as they are different programs, instead it only communicates with the runtime. The only needed information are the server contract address/`AccountId`, it's _exposed_ methods' names and their arguments' values. Any state, and any trait definitions are unnecessary to the client, again, since it can't have direct contact with values related to that. (note: near-sdk's current macro behaviour, for the client, is to _erase_ the trait definition since it's not required).  
The server also doesn't need to directly receive communications from the clients. They receive calls from the runtime, which simply invoke it's exposed functions. All required information about it's current state, and also about the client's call such as the argument values are given by the runtime (from the environment), and the server's response also goes back to the environment, not to the client. With this separations of concerns, the runtime is free to define it's exposed functions as it sees fit. The server is free to use as much generics and traits as it wants, it only needs to specialize all generics when effectively defining the exposed functions.

