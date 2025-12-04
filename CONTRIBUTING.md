# Contributing

## License

By submitting a contribution to this *Kronark Node Compiler Prototype* project, you agree that your contribution is licensed under the [Apache 2.0 License](./LICENSE). You confirm that you wrote your contribution or otherwise have the necessary legal rights to submit your contribution under this project's [Apache 2.0 License](./LICENSE).

## Developer Certificate of Origin (DCO)

Each commit has to be signed off by the contributor it originated from, to ensure all contributions are made with proper authorisation. This means, each commit message needs to include a `Signed-off-by: <NAME> <EMAIL>` suffix, which matches the *public* name and email of the contributor. This can be accomplished automatically by committing via `git commit -s -m 'COMMIT MESSAGE'`. It can be activated by default within VSCode by navigating to `preferences > settings` and activating `Git: Always Sign Off`. You can read the DCO [here](./legal/DCO.md) or [online](https://developercertificate.org/).

## Version Control

- Branches are named in *kebab-case* (e.g. this-is-a-branch-name)
- Branches may only apply to one of the three project areas: `backend`, `frontend` and `documentation`
- Branches needs to signal the area they apply to with an appropriate prefix:
    - `backend`, `be`, `b` or similar for *backend*
    - `frontend`, `fe`, `f` or similar for *frontend*
    - `documenation`, `docs`, `d` or similar for *documentation*
    - `general`, `gen`, `g` or similar for *general* (reserved for management)

## Documentation
British english for documentation, US english for source code. Decision made by popular demand of the core team.

## Code

- *Catch-all*: Naming generally adheres to the language specific standard, if not noted otherwise below
- Variable names that are reserved by keywords and can't be circumvented elegantly are denoted with an additional "_" suffix
- Fields are grouped by purpose
- Imports are alphabetically ordered
- All files need an empty line at the end

## Comments

Place comments above statements where it is not immediately obvious from that statement, what or why it is being done. This includes many of the **rust macros** used to quickly define traits of a given data structure. In general, they should describe *why* the statement exists, not *what* it is doing. Comments should utilise british spelling as they are considered part of the documentation.

## Testing

All relevant functionality should be thoroughly tested, both in the backend and the frontend.

## Tasks

Necessary implementation workload is defined in the markdown files in `docs/`. Different tasks have varying dependencies, which are linked in each respective task file. On task completion, the task file is rewritten into adequate of the produced code. The current full dependency tree of all tasks can be visualised as follows:

<p align="center">
    <img style="display:block;" src="./docs/assets/dependency_graph.drawio.svg">
</p>
