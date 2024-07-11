#!/bin/bash

# Function to print messages in color
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

# Function to check if a command exists
check_command() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check if the program is already installed
check_program_installed() {
    if [ -f /usr/local/bin/calculator ]; then
        return 0
    else
        return 1
    fi
}

# Check if the program already exists
if check_program_installed; then
    print_color "yellow" "The program 'calculator' is already installed."
    read -p "Do you want to overwrite it? (y/n) " choice
    if [[ "$choice" != "y" ]]; then
        print_color "yellow" "Installation aborted."
        exit 0
    fi
fi

# Check for wget
if check_command wget; then
    print_color "green" "wget is installed, starting download"
    wget -O /usr/local/bin/calculator https://github.com/Retro023/Calculator/raw/main/calculator
    if [ $? -eq 0 ]; then
        print_color "green" "Download successful"
        
        # Make the file executable
        chmod +x /usr/local/bin/calculator

        # Add to PATH if not already in PATH
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
