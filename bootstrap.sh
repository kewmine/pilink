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

function setup_vps() {
  sudo bash -c "apt-get -y clean && dpkg --configure -a -y&& apt-get update -y && apt-get purge man-db -y && apt-get upgrade -y" &> /dev/null || panic "failed to update vps"
}

# check if all the necessary tools are installed
function check_tools(){

  # make an array of missing tools
  required_tools=("git" "pnpm" "rustup" "tool2" "tool3")
  for tool in ${required_tools[*]}; do
    which $tool &> /dev/null || missing_tools+=("$tool")
  done

  echo "${missing_tools[*]}"

  # install git if not found
  if [[ $(echo ${missing_tools[*]} | grep -Fw "git") ]]; then
      echo " - installing git"
      apt install -y git &> /dev/null || panic "error while trying to install git"
  fi

  # install rustup if not found
  if [[ $(echo ${missing_tools[*]} | grep -Fw "rustup") ]]; then
    echo " - installing rustup"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/install_rustup.sh
    sh /tmp/install_rustup.sh &> /dev/null || panic "error while trying to install rustup"
  fi

  # install pnpm if not found
  if [[ $(echo ${missing_tools[*]} | grep -Fw "pnpm") ]]; then
    echo " - installing pnpm"
    curl -fsSL https://get.pnpm.io/install.sh | sh - &> /dev/null || panic "error while trying to instlal pnpm"
  fi

  # update env
  source $HOME/.bashrc

  success "all the necessary tools are available"
}

function clone_repo(){
  repo="https://github.com/kewmine/pilink"
  git clone $repo &> /dev/null && cd pilink|| panic "error while trying to clone the repo"
  success "cloned the repo"
}

function setup_rustenv(){
  rustup default nightly &> /dev/null || panic "error while trying to setup rust nightly"
  rustup update &> /dev/null || panic "error while trying to update rust"
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
  ln -s "$(realpath ./svelte_frontend/build)" "$(realpath ./actix_backend/src/apps/link_shortener/webpages)" || panic "error while trying to bind frontend and backend"
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
