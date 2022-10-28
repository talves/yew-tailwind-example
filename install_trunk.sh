#!/bin/bash

# Call the command for the package silently
trunk --help > /dev/null

# Get the exit code of the last command
command_exit_code="$(echo $?)"

# Run installation if exit code is not equal to 0
if [ "$command_exit_code" -ne "0" ]; then
    # Package does not exist: Do the package installation
    cargo install trunk
else
   echo "Skipping 'trunk' installation: Crate already exists"
fi;
