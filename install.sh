#!/bin/bash


print_color() {
    local color="$1"
    local message="$2"
    case $color in
        "green")
            echo -e "\033[0;32m$message\033[0m"
            ;;
        "red")
            echo -e "\033[0;31m$message\033[0m"
            ;;
        "yellow")
            echo -e "\033[0;33m$message\033[0m"
            ;;
        *)
            echo "$message"
            ;;
    esac
}


check_command() {
    command -v "$1" >/dev/null 2>&1
}


check_program_installed() {
    if [ -f /usr/local/bin/calculator ]; then
        return 0
    else
        return 1
    fi
}


if check_program_installed; then
    print_color "yellow" "The program 'calculator' is already installed."
    read -p "Do you want to overwrite it? (y/n) " choice
    if [[ "$choice" != "y" ]]; then
        print_color "yellow" "Installation aborted."
        exit 0
    fi
fi


if check_command wget; then
    print_color "green" "wget is installed, starting download"
    wget -O /usr/local/bin/calculator https://github.com/Retro023/Calculator/raw/main/Dist/calculator3_0
    if [ $? -eq 0 ]; then
        print_color "green" "Download successful"
        
        
        chmod +x /usr/local/bin/calculator

        
        if [[ ":$PATH:" != *":/usr/local/bin:"* ]]; then
            echo 'export PATH=$PATH:/usr/local/bin' >> ~/.bashrc
            source ~/.bashrc
        fi

        print_color "green" "Installation complete. You can now run 'calculator'."
    else
        print_color "red" "Download failed"
        exit 1
    fi
else
    print_color "red" "wget is not installed. Please install it and run this script again."
    exit 1
fi
