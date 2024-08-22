#!/bin/bash

# Define the directories
directories=("modules/vf-graphql-holochain" "modules/graphql-client")

# Loop through each directory
for dir in "${directories[@]}"; do
  # Update the package version
  npm version patch --prefix "$dir"
  
  # Change into the directory
  cd "$dir" || exit
  
  # Publish the package
  npm publish --access=public
  
  # Go back to the root directory
  cd - || exit
done