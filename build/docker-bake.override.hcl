# (c) Cartesi and individual authors (see AUTHORS)
# SPDX-License-Identifier: Apache-2.0 (see LICENSE)

variable "TAG" {
  default = "1.1.0"
}

variable "DOCKER_ORGANIZATION" {
  default = "juztamau5"
}

target "rollups-node" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-node:${TAG}"]
}

target "hardhat" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-hardhat:${TAG}"]
}

target "arbitration" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-arbitration:${TAG}"]
}
