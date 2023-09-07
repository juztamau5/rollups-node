# (c) Cartesi and individual authors (see AUTHORS)
# SPDX-License-Identifier: Apache-2.0 (see LICENSE)

target "docker-metadata-action" {}
target "docker-platforms" {}

group "default" {
  targets = [
    "arbitration",
    "hardhat",
    "rollups-node",
  ]
}

target "rollups-node" {
  inherits   = ["docker-metadata-action", "docker-platforms"]
  dockerfile = "./build/Dockerfile"
  target     = "rollups-node"
  context    = ".."
}

target "hardhat" {
  inherits = ["docker-metadata-action", "docker-platforms"]
  target   = "hardhat"
  context    = "../rollups-contracts/onchain"
}

target "arbitration" {
  inherits = ["docker-metadata-action", "docker-platforms"]
  target   = "arbitration"
  context    = "../rollups-contracts/onchain"
}
