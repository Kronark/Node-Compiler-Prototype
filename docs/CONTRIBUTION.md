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

## Design Principles

At the core of decision making regarding the eligibility and design of features to be added lie two design principles (so far). The definitions of such are quoted directly from the *Tome of Kronark*, which will be made fully available in time. These principles exist to ensure this Node Compiler project is developed towards its original intentions for the benefit of all.

### Black Box Abolishment

> In pursuit of increasing optimisation potential and maximising maintainability, the causal chain behind any byte within an output sequence needs to be traceable and fully editable. A *black box* occurs when these properties are not provided and the resulting bytes can not be explicitely controlled. Traditional usecase specific compilers follow black box design principles by moving optimisations to the backend and consequently removing user access. The direct correlation between the frontend representation and the backend output is severed from a user's perspective. However, access to every abstraction level within a binary construct allows users to optimise their output beyond what compiler architects are capable of anticipating. Abolishing black box design thus promises increased optimisation potential while incidentally allowing user-defined compilation target specification without compiler architect intervention.

#### ***Isn't the format node a black box?***
Yes, and it's functionality is being redesigned as you read. Unfortunately, there is a necessity for such formatting capability in day-to-day use and no adequate solution available so far. The most promising approach that has been dreamt up so far is a meta- / modding- pipeline embedded into the software. Users might gain the ability to create separate node graphs for the express purpose of adding user defined functionality to the compiler backend. Such an intermediate step would not only eliminate remaining black box designs at the lowest levels, it would also make the system infinitely scalable as users would control formatting capabilities. However, such a system throws an interesting concundrum: Why not make *all* built-in nodes, or at the very least all data transforming nodes, modifieable through this system?

### Usecase Generality

> While interconnected node graphs are a common frontend representation in the field of computer programming, the node compiler presented hereafter will not cater to any specific deployment. Although it may appear trivial to provide embedded procedures otherwise impossible within the intended compilation design, the near endless extent of potential node compiler applications prohibits any usecase explicitness, regardless of popularity. All supplied functionality is therefore strictly general purpose and *never* usecase specific. Naturally, this principle does *not* extend to any user-defined node designs - solely to the nodes provided as built-in utilities.

#### ***What even are other usecases for this software?***
While programming will most likely be one of the most popular usecases, there are many other potential applications - potentially even infinitely so. One of these are ***markup languages***, which are practically guaranteed to find support within the node compiler's standard library, as web development has become increasingly prevalent for instance. Note that this usecase not only includes *HTML*, but also *XML* and *SVG*, the latter being of incredible importance for modern software. A more niche usage may be the generation of ***document files*** such as PDFs or similar, since they benefit from procedural generation in many scenarios. A notable example of document related compiler usage would be the generation of invoices and the inclusion of aforementioned compiler generated SVG-based vector graphics. Furthermore, ***JSON*** based files have gained widespread adoption, not only in programmatic data transfer scenarios, but also as a general purpose human readable formatting approach. A widely used yet niche example would be the formatting of *Minecraft Resource- and Datapacks*. 

#### ***Why not add support for each usecase via specialised built-in nodes?***
The purpose of this project is to create a program that provides a node-based syntax for the general purpose of placing arbitrarily defined byte sequences into files. It is designed to cover not only the usecases that can be thought of in the current, but also be capable of covering future usecases noone has dreamt of yet. By taking on the task of adding built-in nodes for specialised usecases, development of the node compiler itself will never end, because the specialised usecases are endless. If we as developers decide to support any specific usecase over others, remaining usecases will demand support inevitably. Such a scenario will lead to the neverending bloating of this piece of software, which is undesirable. A much more scalable approach is to find general purpose building blocks any usecase might benefit from, leaving any usecase specific functionality for the users to define as they are encountered.

#### ***How is usecase specificity defined?***
Defining the line between usecase generality and usecase specificity is a difficult task. A general rule of thumb is the following: If a feature finds usage for more purposes than it was designed for, it probably is sufficiently generic. Further: It must be impossible to create the desired behaviour using existing nodes to an acceptable standard, and it must not remove or impede other functionality. Furthermore, it has proven helpful to consider examples of either side's extremes. For instance, a node that takes in a sequence of bytes and splits it at a specified index into two sub-sequences is generally applicable. Another such example would be a node that reverses the order of bytes in an input sequence. Such nodes can be applied to practically any usecase the compiler could ever be utilised for and are therefore usecase generic. On the other side, imagine a node that takes in a sequence of bytes that are required to be in a specific format (e.g. an intermediate representation). This node then internally transforms that input according to known compiler optimisations and outputs an optimised sequence of the same formatting. Such a node not only violates the *black box abolishment* principle, it also is solely applicable to the programming usecase in a scenario where the predefined formatting is targetted. It is therefore usecase specific and the behaviour needs to be made achievable by the user through other means.
