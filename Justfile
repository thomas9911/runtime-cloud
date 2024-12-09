up:
  wash up
build:
  cd graphql-server && wash build
  cd action-runner && wash build
  cd graphql-server && wash app delete bb-runtime-cloud && wash app deploy local.wadm.yaml
