
variable "TAG" {
  default = "1.0.1"
}

variable "DOCKER_ORGANIZATION" {
  default = "juztamau5"
}

target "state-server" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-state-server:${TAG}"]
}

target "dispatcher" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-dispatcher:${TAG}"]
}

target "indexer" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-indexer:${TAG}"]
}

target "inspect-server" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-inspect-server:${TAG}"]
}

target "advance-runner" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-advance-runner:${TAG}"]
}

target "graphql-server" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-graphql-server:${TAG}"]
}

target "host-runner" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-host-runner:${TAG}"]
}

target "hardhat" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-hardhat:${TAG}"]
}

target "arbitration" {
  tags = ["${DOCKER_ORGANIZATION}/rollups-arbitration:${TAG}"]
}
