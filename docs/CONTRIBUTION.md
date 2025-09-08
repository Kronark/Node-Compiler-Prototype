# Contribution

## Code

- Variables and function are named in *snake_case* (e.g. this_is_a_name)
- Variable names that are reserved by keywords and can't be circumvented elegantly are denoted with an additional "_" suffix
- Fields are grouped by purpose
- Imports are alphabetically ordered
- Types need to be explicitely defined (no `auto`, `any` or similar)
- All files need an empty line at the end

## Comments

Place comments above statements where it is not immediately obvious from that statement, what or why it is being done. This includes many of the **rust macros** used to quickly define traits of a given data structure. In general, they should describe *why* the statement exists, not *what* it is doing.

## Version Control

- Branches are named in *kebab-case* (e.g. this-is-a-branch-name)
- Branches may only apply to one of the three project areas: `backend`, `frontend` and `documentation`
- Branches needs to signal the area they apply to with an appropriate prefix:
    - `backend`, `be`, `b` or similar for *backend*
    - `frontend`, `fe`, `f` or similar for *frontend*
    - `documenation`, `docs`, `d` or similar for *documentation*

## Tasks

Necessary implementation workload is defined in the markdown files in `docs/`. Different tasks have varying dependencies, which are linked in each respective task file. On task completion, the task file is rewritten into adequate of the produced code. The current full dependency tree of all tasks can be visualised as follows:

![](../docs/assets/dependency_graph.jpg)
