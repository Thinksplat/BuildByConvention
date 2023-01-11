# BuildByConvention

A tool to setup a build environment by convention.  Very opinioniated

# Concept

Creating CMakeFiles for a project is a tedious task.  This tool aims to automate the process by convention.  It will create a CMakeLists.txt build
environment using the names of directories and the content within to determine the build targets.

The expectation is that libraries will be built, applications will be built, prototype applications will be built, and tests will be used.

# Opinionations

- The overall directory structure will be defined here as a starting point.
- Google test will be pulled in by default.
- External libraries will be fetched from external repositories.
  - Google test

# Directory Convention

- Directories beginning with `lib` will be built as libraries.
- Directories beginning with `app` will be built as applications.
- Directories beginning with `test` will be built as tests.
  - See note

Underscores can be used or not.  Camel case is likely used, but not required.

- A directory libhelper is built as a library named helper.
- A directory libHelper is built as a library named Helper.
- A directory lib_helper is built as a library named helper.
- A directory lib_Helper is built as a library named Helper.
  
## Note on tests

- It is expected that the test directory is named after the library it is testing.  For example, if
you have a library directory named libHelper, then the test directory should be named testHelper.
- The test directory will be automatically linked against the library it is testing.

## Expectations on tests

- It is expected that for each library, there will be a corresponding test directory.
- The software will complain, and possibly fail if this correspondance does not exist.
- The test should only depend on the single library it is testing.
- The test should not depend on any other libraries.
  - This can be overridden by using the .autobuild.yml file.
  - The software may complain if linking against other libraries.

# Special files

Within each lib, app, or test directory, there is a special file named .autobuild.yml that can be used to control the build process.

## .autobuild.yml files
- `include` - A list of directories to include in the build.  This is useful for including external libraries.
- `lib` - A list of project libraries to link against.
  - If you link against an internal project library, an include 
- `extlib` - A list of external libraries to link against.  