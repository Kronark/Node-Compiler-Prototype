# Contribution

## Code

- Variables and function are named in *snake_case* (e.g. this_is_a_name)
- Variable names that are reserved by keywords and can't be circumvented elegantly are denoted with an additional "_" suffix
- Fields are grouped by purpose
- Imports are alphabetically ordered
- Types need to be explicitely defined (no `auto`, `any` or similar)
- All files need an empty line at the end

## Version Control

- Branches are named in *kebab-case* (e.g. this-is-a-branch-name)
- Branches may only apply to one of the three project areas: `backend`, `frontend` and `documentation`
- Branches needs to signal the area they apply to with an appropriate prefix:
    - `backend`, `be`, `b` or similar for *backend*
    - `frontend`, `fe`, `f` or similar for *frontend*
    - `documenation`, `docs`, `d` or similar for *documentation*
