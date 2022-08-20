# Quadratic Voting

This is a simple library to implement a quadratic voting library. Quadratic voting weights the reserve token amount required to cast a vote for a given proposal on a quadratic scale. 

This library is modeled after the Substrate framework. Its purpose is to mimic the setup and implementations as they are setup in Substrate and experiment with Rust's intrinsic features such as traits and generics.

# Improvements To Ponder Upon

Permits proposals to be made again since the Proposal ID assigned is independent of the incoming proposal data used to initialize a proposal. 

Does not permit revotes by a user. Does not accomodate for split voting for example (Aye: 4, Nay: 5) by the same voter. 
