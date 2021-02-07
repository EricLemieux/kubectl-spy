#!/usr/bin/env bash
# End to end test the application

set -euo pipefail

APPLICATION=./target/release/kubectl-spy

function assert_fails() {
  if bash -c "$1"; then
    exit 1
  else
    exit 0
  fi
}

$APPLICATION my-secret-name
$APPLICATION my-secret-name another-secret
$APPLICATION database-secret -n database
$APPLICATION database-secret --namespace database
$APPLICATION empty-secret
assert_fails "$APPLICATION secret-that-does-not-exist"
