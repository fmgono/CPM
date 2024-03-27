![Prismify render](https://github.com/fmgono/CPM/assets/31404706/e084874c-37e9-40fd-979f-0d1f94d4b05c)

# What is CPM

CPM stands for "Code Project Manager". It is a simple, cross-platform desktop app that shows all the projects the dev has on their machine without the need to browse each directory by themselves, tells how much storage space is used for each project, and can also delete the dependencies to free space (by removing the node_modules folder for example).

## Features

These are the features that we want to build:

1. List all the code projects on our disk without the need to browse each directory by ourselves.
2. Ability to easily remove the projects or the dependencies for old projects to free some space.
3. Ability to overview the project pieces of information such as:
   1. The size of the project
   2. Creation date
   3. Last Modified date
   4. Git information (if any):
      1. Unstaged files
      2. Staged files
      3. Unpushed commit

## Techstacks

1. Frontend
   1. SvelteKit for frameworks
   2. TailwindCSS for styling
   3. Shadcn-ui (Svelte version)
   4.
2. Backend
   1. Tauri (Rust) for handling the backend (file system)

## Architecture

CPM is a cross-platform application built using the Tauri framework. Tauri is based on Microsoft Edge-similar webview and Rust to work. Read about Tauri [here](https://tauri.app/)

CPM relies on Rust API for file operations and Svelte, Typescript, and TailwindCSS for the webview. Rust codes are under the src-tauri directory whereas the webview code is under the src directory.

## How to Install?

CPM is currently under heavy development. You can give your suggestions and feedback on our [Discussions](https://github.com/fmgono/CPM/discussions/) page.

## Bug Reporting

If you find any bugs, please report them by submitting an issue on our [issue page](https://github.com/fmgono/CPM/issues) with a detailed explanation. Giving some screenshots would also be very helpful.

## How to Contribute

If you feel comfortable writing code using the tech stack we mentioned earlier, we highly encourage you to contribute and read our [Contributing Guide](CONTRIBUTING.md)
