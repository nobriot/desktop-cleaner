# Define variables
SERVICE_NAME = desktop-cleaner
BINARY_NAME = desktop-cleaner
TARGET_DIR = target/release
MACOS_PLIST_FILE = com.example.desktop-cleaner.plist
LINUX_SERVICE_FILE = desktop-cleaner.service

# Default target
all: build

# Detect the operating system
UNAME_S := $(shell uname -s)
ifeq ($(OS),Windows_NT)
    UNAME_S := Windows
endif

# Build the binary
build:
	cargo build --release

# Create the plist file for macOS
$(MACOS_PLIST_FILE):
	@echo "<?xml version=\"1.0\" encoding=\"UTF-8\"?>" > $(MACOS_PLIST_FILE)
	@echo "<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">" >> $(MACOS_PLIST_FILE)
	@echo "<plist version=\"1.0\">" >> $(MACOS_PLIST_FILE)
	@echo "<dict>" >> $(MACOS_PLIST_FILE)
	@echo "    <key>Label</key>" >> $(MACOS_PLIST_FILE)
	@echo "    <string>com.example.desktop-cleaner</string>" >> $(MACOS_PLIST_FILE)
	@echo "    <key>ProgramArguments</key>" >> $(MACOS_PLIST_FILE)
	@echo "    <array>" >> $(MACOS_PLIST_FILE)
	@echo "        <string>/usr/local/bin/$(BINARY_NAME)</string>" >> $(MACOS_PLIST_FILE)
	@echo "    </array>" >> $(MACOS_PLIST_FILE)
	@echo "    <key>RunAtLoad</key>" >> $(MACOS_PLIST_FILE)
	@echo "    <true/>" >> $(MACOS_PLIST_FILE)
	@echo "    <key>KeepAlive</key>" >> $(MACOS_PLIST_FILE)
	@echo "    <true/>" >> $(MACOS_PLIST_FILE)
	@echo "    <key>StandardErrorPath</key>" >> $(MACOS_PLIST_FILE)
	@echo "    <string>/tmp/desktop-cleaner.err</string>" >> $(MACOS_PLIST_FILE)
	@echo "    <key>StandardOutPath</key>" >> $(MACOS_PLIST_FILE)
	@echo "    <string>/tmp/desktop-cleaner.out</string>" >> $(MACOS_PLIST_FILE)
	@echo "</dict>" >> $(MACOS_PLIST_FILE)
	@echo "</plist>" >> $(MACOS_PLIST_FILE)

# Create the service file for Linux
$(LINUX_SERVICE_FILE):
	@echo "[Unit]" > $(LINUX_SERVICE_FILE)
	@echo "Description=Desktop Cleaner Service" >> $(LINUX_SERVICE_FILE)
	@echo "" >> $(LINUX_SERVICE_FILE)
	@echo "[Service]" >> $(LINUX_SERVICE_FILE)
	@echo "ExecStart=/usr/local/bin/$(BINARY_NAME)" --dry-run --interval 15 >> $(LINUX_SERVICE_FILE)
	@echo "Restart=always" >> $(LINUX_SERVICE_FILE)
	@echo "User=$(USER)" >> $(LINUX_SERVICE_FILE)
	@echo "" >> $(LINUX_SERVICE_FILE)
	@echo "[Install]" >> $(LINUX_SERVICE_FILE)
	@echo "WantedBy=multi-user.target" >> $(LINUX_SERVICE_FILE)

# Create the service file for Windows (using PowerShell)
$(WINDOWS_SERVICE_NAME):
	@powershell -Command "New-Service -Name '$(WINDOWS_SERVICE_NAME)' -BinaryPathName '$(CURDIR)\\$(TARGET_DIR)\\$(BINARY_NAME).exe' -DisplayName 'Desktop Cleaner Service' -Description 'A service that moves files from the Desktop to the trash' -StartupType Automatic"

# Install the binary and service file
install: build
ifeq ($(UNAME_S),Darwin)
	sudo cp -f $(TARGET_DIR)/$(BINARY_NAME) /usr/local/bin/$(BINARY_NAME)
	make $(MACOS_PLIST_FILE)
	cp $(MACOS_PLIST_FILE) ~/Library/LaunchAgents/
	launchctl load ~/Library/LaunchAgents/$(MACOS_PLIST_FILE)
else ifeq ($(UNAME_S),Linux)
	sudo cp -f $(TARGET_DIR)/$(BINARY_NAME) /usr/local/bin/$(BINARY_NAME)
	make $(LINUX_SERVICE_FILE)
	sudo cp -f $(LINUX_SERVICE_FILE) /etc/systemd/system/
	sudo systemctl enable $(LINUX_SERVICE_FILE)
	sudo systemctl start $(LINUX_SERVICE_FILE)
else ifeq ($(UNAME_S),Windows)
	cp $(TARGET_DIR)/$(BINARY_NAME).exe /usr/local/bin/$(BINARY_NAME).exe
	make $(WINDOWS_SERVICE_NAME)
endif

# Uninstall the service
uninstall:
ifeq ($(UNAME_S),Darwin)
	launchctl unload ~/Library/LaunchAgents/$(MACOS_PLIST_FILE)
	sudo rm -f ~/Library/LaunchAgents/$(MACOS_PLIST_FILE)
	sudo rm -f /usr/local/bin/$(BINARY_NAME)
else ifeq ($(UNAME_S),Linux)
	sudo systemctl stop $(LINUX_SERVICE_FILE)
	sudo systemctl disable $(LINUX_SERVICE_FILE)
	sudo rm -f /etc/systemd/system/$(LINUX_SERVICE_FILE)
	sudo rm -f /usr/local/bin/$(BINARY_NAME)
else ifeq ($(UNAME_S),Windows)
	@powershell -Command "Remove-Service -Name '$(WINDOWS_SERVICE_NAME)'"
	sudo rm -f /usr/local/bin/$(BINARY_NAME).exe
endif

# Clean the project
clean:
	cargo clean
	rm -f $(MACOS_PLIST_FILE)
	rm -f $(LINUX_SERVICE_FILE)
	sudo rm -f /usr/local/bin/$(BINARY_NAME).exe
	sudo rm -f /usr/local/bin/$(BINARY_NAME)

.PHONY: all build install uninstall clean