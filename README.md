A util to generate PlantUML-compatible files from `*.gradle` files representing dependencies between a project's internal modules.

## How it works

The util looks for lines that look like: `implementation(project(":submodule"))` in all `*.gradle` files in a project and writes PlantUML-compatible spec to a file named `modules.uml` in the root of the project.

## How to use

```
$ gradle_modules_graph ~/my_gradle_project
$ plantuml modules.uml
```

Open generated `modules.png`.

PlantUML may need to be invoked with `PLANTUML_LIMIT_SIZE=8192` for a larger number of modules.
