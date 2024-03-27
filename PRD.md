# Product Requirements Document

This is documentation about the product, the behavior, the UX, and a description of how users will use this product.

## First iteration

1. CPM is a desktop app that runs on cross-platforms that show all of the code projects that users have on their disk.
2. When the user opens the app, the user will see a splash screen or loading indicator while the system will scan all the user's directories.
3. After the splash screen disappears and the scanning process is done, the user will see a list of projects they had with the Card Component.
4. The information provided by the Card component is:
   1. Icon Project: can be the favicon.ico (if web app) or just the logo of the programming language that is used in the project.
   2. Folder Name / Project Name
   3. Path folder
   4. Github Information (if any):
      1. Remote repository (if any)
      2. Pull and push information
   5. Size of the project directory
   6. When the project is initiated
   7. When the project is modified
   8. A dropdown button, with this set of actions:
      1. Delete Dependencies: the dependencies folder (ex: if the project is NodeJS, then it will delete the `node_modules` folder, for another programming language, soon)
         1. When clicked, will show AlertDialog with this information:
            1. Title: "Deleting the dependencies folder?"
            2. Description: "This will only delete the `node_modules` folder, not the project folder itself"
            3. Action Button: 1. No 2. Yes, delete it
      2. Delete Project: deleting the project, can't be undone.
         1. When clicked, will show AlertDialog with this information:
            1. Title: "Are you absolutely sure deleting the project?"
            2. Description:
               1. If there are unstaged changes or unpushed commit = "You still have [totalUnstagedChanges] and [totalUnpushedCommit] in this project"
               2. else, "This will delete the project itself and can't be undone"
            3. Action Button:
               1. No
               2. Yes, delete it

## 2nd iteration

Soon!
