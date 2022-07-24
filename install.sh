#!/usr/bin/env bash

# shellcheck disable=SC2181

# Reset
Color_Off=''

# Regular Colors
Red=''
Green=''

# Bold
BGreen=''

Dim='' # White

if test -t 1; then
    # Reset
    Color_Off='\033[0m' # Text Reset

    # Regular Colors
    Red='\033[0;31m'   # Red
    Green='\033[0;32m' # Green

    Dim='\033[0;2m' # White

    # Bold
    BGreen='\033[1;32m' # Green
fi

case $(uname -sm) in
    "Darwin x86_64") target="x86_64-apple-darwin" ;;
    "Darwin arm64") target="aarch64-apple-darwin" ;;
    "Linux aarch64") target="aarch64-unknown-linux-gnu" ;;
    "Linux arm64") target="aarch64-unknown-linux-gnu" ;;
    "Linux x86_64") target="x86_64-unknown-linux-gnu" ;;
    *) target="x86_64-unknown-linux-gnu" ;;
esac

if [ "$target" = "x86_64-apple-darwin" ]; then
    # Is it rosetta?
    sysctl sysctl.proc_translated >/dev/null 2>&1
    if [ $? -eq 0 ]; then
        target="aarch64-apple-darwin"
        echo -e "$Dim Your shell is running in Rosetta 2. Downloading jimaku for $target instead. $Color_Off"
    fi
fi

github_repo="https://github.com/aeyoll/jimaku"

if [ $# -eq 0 ]; then
    jimaku_uri="$github_repo/releases/latest/download/jimaku-${target}.tar.gz"
else
    jimaku_uri="$github_repo/releases/download/${1}/jimaku-${target}.tar.gz"
fi

jimaku_install="${JIMAKU_INSTALL:-$HOME/.jimaku}"
bin_dir="$jimaku_install/bin"
exe="$bin_dir/jimaku"

if [ ! -d "$bin_dir" ]; then
    mkdir -p "$bin_dir"

    if (($?)); then
        echo -e "${Red}error${Color_Off}: Failed to create install directory $bin_dir" 1>&2
        exit 1
    fi
fi

curl --fail --location --progress-bar --output "$exe.tar.gz" "$jimaku_uri"

if (($?)); then
    echo -e "${Red}error${Color_Off}: Failed to download jimaku from $jimaku_uri" 1>&2
    exit 1
fi

tar xvzf "$exe.tar.gz" -C "$bin_dir"

if (($?)); then
    echo -e "${Red}error${Color_Off}: Failed to extract jimaku" 1>&2
    exit 1
fi

chmod +x "$exe"

if (($?)); then
    echo -e "${Red}error${Color_Off}: Failed to set permissions on jimaku executable." 1>&2
    exit 1
fi

rm "$exe.tar.gz"

echo -e "${Green}jimaku was installed successfully to ${BGreen}$exe$Color_Off"
