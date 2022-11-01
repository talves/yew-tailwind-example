#!/bin/bash

TRUNK_BIN_VERSION=$(trunk --version)
echo $TRUNK_BIN_VERSION

if [ $TRUNK_VERSION ]; then
    if [ ! -f ~/.cargo/bin/trunk ]; then
        echo "trunk doesn't exist... installing $(echo $TRUNK_VERSION)"
        ## Should go to /opt/buildhome/.cargo/bin/trunk
        #  https://github.com/thedodd/trunk/releases/download/v0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz
        # curl -OL https://github.com/thedodd/trunk/releases/download/v$TRUNK_VERSION/trunk-x86_64-unknown-linux-gnu.tar.gz
        # tar xvzf trunk-x86_64-unknown-linux-gnu.tar.gz -C ~/.cargo/bin/
        # rm -rf trunk-x86_64-unknown-linux-gnu.tar.gz
        cargo install trunk --git https://github.com/thedodd/trunk --tag "v${TRUNK_VERSION}"
    else
        if [ "${TRUNK_BIN_VERSION}" = "trunk ${TRUNK_VERSION}" ]; then
            echo "$(trunk -V) already exists"
        else
            # Forces an install of the new version if it was different from installed version
            cargo install trunk --git https://github.com/thedodd/trunk --tag "v${TRUNK_VERSION}" --force
        fi
    fi
else
    # Call the command for the package silently
    ls -f ~/.cargo/bin/trunk > /dev/null

    # Get the exit code of the last command
    command_exit_code="$(echo $?)"

    # Run installation if exit code is not equal to 0
    if [ "$command_exit_code" -ne "0" ]; then
        # Package does not exist: Do the package installation
        cargo install trunk
    else
    echo "Skipping 'trunk' installation: Crate ($TRUNK_BIN_VERSION) already exists"
    fi;
fi