if [[ "$OSTYPE" == "linux-gnu" ]]; then
	set -e

	if [ -f /etc/redhat-release ]; then
		echo "Redhat Linux detected."
		echo "This OS is not supported with this script at present. Sorry."
		echo "Please refer to https://github.com/paritytech/substrate for setup information."
		exit 1;
	elif [ -f /etc/SuSE-release ]; then
		echo "Suse Linux detected."
		echo "This OS is not supported with this script at present. Sorry."
		echo "Please refer to https://github.com/paritytech/substrate for setup information."
		exit 1;
	elif [ -f /etc/arch-release ]; then
		echo "Arch Linux detected."
		echo "This OS is not supported with this script at present. Sorry."
		echo "Please refer to https://github.com/paritytech/substrate for setup information."
		exit 1;
	elif [ -f /etc/mandrake-release ]; then
		echo "Mandrake Linux detected."
		echo "This OS is not supported with this script at present. Sorry."
		echo "Please refer to https://github.com/paritytech/substrate for setup information."
		exit 1;
	elif [ -f /etc/debian_version ]; then
		echo "Ubuntu/Debian Linux detected."
		set -e

		BUILD_NUM=`curl -s https://storage.googleapis.com/wasm-llvm/builds/linux/lkgr.json | jq -r '.build'`
		if [ -z ${BUILD_NUM+x} ]; then
			echo "Could not fetch the latest build number.";
			exit 1;
		fi

		tmp=`mktemp -d`
		pushd $tmp > /dev/null
		echo "Downloading wasm-binaries.tbz2";
		curl -L -o wasm-binaries.tbz2 https://storage.googleapis.com/wasm-llvm/builds/linux/$BUILD_NUM/wasm-binaries.tbz2

		declare -a binaries=("wasm2wat" "wat2wasm") # Default binaries
		if [ "$#" -ne 0 ]; then
			echo "Installing selected binaries.";
			binaries=("$@");
		else
			echo "Installing default binaries.";
		fi
		
		for bin in "${binaries[@]}"
		do
			echo "Installing $bin into ~/.cargo/bin"
			tar -xvjf wasm-binaries.tbz2 wasm-install/bin/$bin > /dev/null
			cp -f wasm-install/bin/$bin ~/.cargo/bin/
		done
		popd > /dev/null
	fi
	else
		echo "Unknown Linux distribution."
		echo "This OS is not supported with this script at present. Sorry."
		echo "Please refer to https://github.com/paritytech/substrate for setup information."
		exit 1;
	fi

elif [[ "$OSTYPE" == "darwin"* ]]; then
	echo "Mac OS (Darwin) detected."
	echo "This OS is not supported with this script at present. Sorry."
	echo "Please refer to https://github.com/paritytech/substrate for setup information."
	exit 1;
elif [[ "$OSTYPE" == "freebsd"* ]]; then
	echo "FreeBSD detected."
	echo "This OS is not supported with this script at present. Sorry."
	echo "Please refer to https://github.com/paritytech/substrate for setup information."
	exit 1;
else
	echo "Unknown operating system."
	echo "This OS is not supported with this script at present. Sorry."
	echo "Please refer to https://github.com/paritytech/substrate for setup information."
	exit 1;
fi

echo ""
echo "Run source ~/.cargo/env now to update environment."
echo ""
