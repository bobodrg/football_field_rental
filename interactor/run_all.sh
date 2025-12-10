#!/bin/bash

set -e
rm -f state.toml

set -o pipefail

cd "$(dirname "$0")"   # intră în folderul interactor

echo "------------------------------------------------"
echo " STEP 1: Deploy smart contract"
echo "------------------------------------------------"
cargo run -- deploy
echo ""

echo "------------------------------------------------"
echo " STEP 2: Set court cost"
echo "------------------------------------------------"
cargo run -- setFootballCourtCost
echo ""

echo "------------------------------------------------"
echo " STEP 3: Create football slot"
echo "------------------------------------------------"
cargo run -- createFootballSlot
echo ""

echo "------------------------------------------------"
echo " STEP 4: Participate to slot"
echo "------------------------------------------------"
cargo run -- participateToFootballSlot
echo ""

echo "------------------------------------------------"
echo " STEP 5: Confirm slot"
echo "------------------------------------------------"
cargo run -- confirmSlot
echo ""

echo "------------------------------------------------"
echo " STEP 6: Pay court"
echo "------------------------------------------------"
cargo run -- payCourt
echo ""

echo "------------------------------------------------"
echo " STEP 7: Get slot status"
echo "------------------------------------------------"
cargo run -- getSlotStatus
echo ""

echo "================================================"
echo " ALL STEPS COMPLETED SUCCESSFULLY "
echo "================================================"
