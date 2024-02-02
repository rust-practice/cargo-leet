#!/bin/sh

# Expected to be run from the scripts folder

cp commit-msg ../.git/hooks/

if [ $? != 0 ]
then
    echo "[error] The copy seems to have failed. This script expects to be run from inside the scripts folder"
    exit 1
fi
