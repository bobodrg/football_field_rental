#!/bin/bash
set -e

# -----------------------------------------------------
# WALLET-URI
# -----------------------------------------------------

ADMIN="../new_wallet.pem"
BOB="interactor/wallets/bob.pem"

# -----------------------------------------------------
# REȚEA
# -----------------------------------------------------
PROXY="https://devnet-gateway.multiversx.com"
CHAIN="D"

PROJECT="../output/football-field-rental.wasm"

ADDRESS_KEY="football-devnet-address"

# -----------------------------------------------------
# UTILS
# -----------------------------------------------------

save_address() {
  mxpy data store --key=${ADDRESS_KEY} --value=$1
}

getAddress() {
  mxpy data load --key=${ADDRESS_KEY}
}

get_wallet_address() {
  mxpy wallet convert --pem "$1" --out-format=address-bech32 | tail -1
}

log() {
  echo ""
  echo "===================================================="
  echo "$1"
  echo "===================================================="
}

# -----------------------------------------------------
# DEPLOY CONTRACT
# -----------------------------------------------------

deploy() {
  log "Deploying contract with ADMIN..."

  mxpy contract deploy \
    --bytecode=${PROJECT} \
    --pem=${ADMIN} \
    --chain=${CHAIN} \
    --gas-limit=80000000 \
    --proxy=${PROXY} \
    --arguments 1000000000000000000 \
    --send \
    --outfile="deploy.json"

  CONTRACT_ADDRESS=$(mxpy data parse --file="deploy.json" --expression="data['contractAddress']")

  save_address "${CONTRACT_ADDRESS}"

  echo "✅ Contract deployed at: $CONTRACT_ADDRESS"
}

# -----------------------------------------------------
# QUERY SLOT STATUS
# -----------------------------------------------------

getSlotStatus() {
  ADDRESS=$(getAddress)

  log "Query slot status"

  mxpy contract query ${ADDRESS} \
    --function getSlotStatus \
    --proxy ${PROXY}
}

# -----------------------------------------------------
# CREATE SLOT
# -----------------------------------------------------

createSlot() {
  ADDRESS=$(getAddress)

  log "Create football slot"

  mxpy contract call ${ADDRESS} \
    --pem=${ADMIN} \
    --gas-limit=60000000 \
    --value=1000000000000000000 \
    --function createFootballSlot \
    --arguments 1700 1900 \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN}
}

# -----------------------------------------------------
# PARTICIPATE – ADMIN
# -----------------------------------------------------

participateAdmin() {
  ADDRESS=$(getAddress)

  log "Admin participates"

  mxpy contract call ${ADDRESS} \
    --pem=${ADMIN} \
    --gas-limit=60000000 \
    --value=1000000000000000000 \
    --function participateToFootballSlot \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN}
}

# -----------------------------------------------------
# PARTICIPATE – BOB
# -----------------------------------------------------

participateBob() {
  ADDRESS=$(getAddress)

  log "Bob participates"

  mxpy contract call ${ADDRESS} \
    --pem=${BOB} \
    --gas-limit=60000000 \
    --value=1000000000000000000 \
    --function participateToFootballSlot \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN}
}

# -----------------------------------------------------
# SET MANAGER
# -----------------------------------------------------

setManager() {
  ADDRESS=$(getAddress)
  MANAGER=$1

  log "Set manager → $MANAGER"

  mxpy contract call ${ADDRESS} \
    --pem=${ADMIN} \
    --gas-limit=30000000 \
    --function setFootballFieldManager \
    --arguments "addr:${MANAGER}" \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN}
}

# -----------------------------------------------------
# CONFIRM SLOT
# -----------------------------------------------------

confirmSlot() {
  ADDRESS=$(getAddress)

  log "Confirm slot (BOB)"

  mxpy contract call ${ADDRESS} \
    --pem=${BOB} \
    --gas-limit=30000000 \
    --function confirmSlot \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN}
}

# -----------------------------------------------------
# PAY COURT
# -----------------------------------------------------

payCourt() {
  ADDRESS=$(getAddress)

  log "Bob pays court"

  mxpy contract call ${ADDRESS} \
    --pem=${BOB} \
    --gas-limit=60000000 \
    --function payCourt \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN}
}

# -----------------------------------------------------
# RUN ALL STEPS
# -----------------------------------------------------

run_all() {
  deploy

  MANAGER_BOB=$(get_wallet_address "${BOB}")
  setManager ${MANAGER_BOB}

  createSlot
  participateAdmin
  participateBob
  confirmSlot
  payCourt
  getSlotStatus
}

# -----------------------------------------------------
# COMMANDS
# -----------------------------------------------------

case "$1" in
  deploy) deploy ;;
  create) createSlot ;;
  joinA) participateAdmin ;;
  joinB) participateBob ;;
  confirm) confirmSlot ;;
  pay) payCourt ;;
  status) getSlotStatus ;;
  run_all) run_all ;;
  *) echo "Usage: ./run_tests.sh [deploy|create|joinA|joinB|confirm|pay|status|run_all]" ;;
esac
