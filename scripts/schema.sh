#!/bin/bash

# Define the main directory path
MAIN_DIR=$(dirname "$(realpath "$0")")/..

# Print out the main directory path for debugging
echo "Main directory: $MAIN_DIR"

CONTRACTS_DIR="$MAIN_DIR"
TS_DIR="$MAIN_DIR/ts"

# Print out the contracts directory path for debugging
echo "Contracts directory: $CONTRACTS_DIR"

# Print out the TypeScript directory path for debugging
echo "TypeScript directory: $TS_DIR"

# Generate the schema for the contract
cargo schema

# Start ts codegen script
cd $TS_DIR
yarn generate-ts

# Return to the main directory
cd $MAIN_DIR

# Clean up the schema file
rm -rv $MAIN_DIR/schema


