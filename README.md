<p align="center">
    <img style="display:block;width:200px;height:200px;" src="./docs/assets/logo.svg">
</p>

# Node Compiler Prototype
A collaborative effort towards a fully featured prototype of the byte compiler design proposed by Kronark.

## Contribution Guidelines
We appreciate every contribution with a pull-request. See our full [contribution guidelines](./CONTRIBUTION.md).

## Project Documentation
This repository contains source code for a fully featured prototype of Kronark's *general purpose byte sequence constructor* design. Terminology utilised within this project and its resulting ecosystem is outlined in our [lexicon](./docs/LEXICON.md). This prototype is split into a [frontend](./docs/frontend/frontend.md) and a [backend](./docs/backend/backend.md) component, each of which act as a distinct piece of software. Either of the two programs can be executed on its own and will provide the limited feature-set it is designed for within the compiler. This decision was made for this prototype specifically to ensure a smooth transition to the self-hosted implementation of the node compiler following this project. The clear separation of frontend and backend allows for a gradual reimplementation instead of a full restart. Note that the self-hosting work will include the implementation of graphics libraries for the frontend, which is a significant workload compared to the more simple (feature- and dependency-wise) compiler backend.

When invoked together, the two programs communicate via IPC. Note that communication is only ever initiated by the frontend and *never* by the backend. Relevant information produced by the backend necessary for frontend visualisation is solely made available as a response to a frontend-initiated request (e.g. by triggering a compilation procedure).
