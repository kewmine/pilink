#!/bin/bash
# standalone install script to bootstrap entire service
wd=`pwd`/pilink

function panic() {
  echo -e "\n//--------------------\nPANICKING: $1"
  exit 1
}

function success() {
  echo -e "[*] $1"
}

# check if all the necessary tools are installed
function check_tools(){
  required_tools=("git" "pnpm" "rustup" "cargo")

  for tool in ${required_tools[@]}
  do
    which $tool &> /dev/null || missing_tools+=("$tool")
  done

  if [ ${#missing_tools[@]} -ne 0 ]
  then
          echo -e "you must install the following before continuing:\n\t${missing_tools[@]}"
          exit 1
  fi
  success "checked all the necessary tools"
}

function clone_repo(){
  repo="https://github.com/kewmine/pilink"
  git clone $repo &> /dev/null && cd pilink|| panic "something went wrong trying to clone the repo"
  success "cloned the repo"
}

function setup_rustenv(){
  rustup default nightly &> /dev/null || panic "something went wrong trying to setup rust nightly"
  rustup update &> /dev/null || panic "something went wrong trying to update rust"
  success "rust nightly updated"
}

function build_frontend() {
  cd $wd/svelte_frontend &> /dev/null || panic "svelte_frontend directory not found"
  pnpm update &> /dev/null || panic "failed to update pnpm environment"
  pnpm build &> /dev/null || panic "failed to build frontend"
  success "built frontend"
}


function build_backend() {
  cd $wd/actix_backend &> /dev/null || panic "actix_backend directory not found"
  cargo build &> /dev/null || panic "failed to build backend"
  success "built backend"
}

function bind_front_and_back() {
  cd $wd
  ln -s "$(realpath ./svelte_frontend/build)" "$(realpath ./actix_backend/src/apps/link_shortener/webpages)" || panic "something went wrong while trying to bind frontend and backend"
  success "bound frontend and backend"
}

function launch_server() {
  success "launching actix server | logs at $wd/actix.log"
  cd ./actix_backend && cargo run &> $wd/actix_backend/actix.log
}


# start checking
check_tools
clone_repo
setup_rustenv
build_frontend
bind_front_and_back
launch_server
