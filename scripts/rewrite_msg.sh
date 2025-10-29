#!/bin/bash

# Read the commit message from stdin
MESSAGE=$(cat)

# Get the first line
FIRST_LINE=$(echo "$MESSAGE" | head -n1)

# Make the first line lowercase
FIRST_LINE=$(echo "$FIRST_LINE" | tr '[:upper:]' '[:lower:]')

# Truncate the first line to 60 characters
FIRST_LINE=${FIRST_LINE:0:60}

# If the first line doesn't start with a conventional type, prepend "chore: "
if [[ ! $FIRST_LINE =~ ^[a-z]+: ]]; then
    FIRST_LINE="chore: $FIRST_LINE"
fi

# Output the rewritten first line
echo "$FIRST_LINE"

# Output the rest of the message unchanged
echo "$MESSAGE" | tail -n +2
