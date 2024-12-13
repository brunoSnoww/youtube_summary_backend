
.PHONY: all check-deps clone build model run clean

# Variables
REPO_URL=https://github.com/ggerganov/whisper.cpp.git
REPO_DIR=whisper.cpp
MODEL_SCRIPT=models/download-ggml-model.sh
MODEL=base.en

# Ensure all dependencies are installed
check-deps:
	@which sox > /dev/null || { echo "sox not found. Please install it using your package manager (e.g., apt, brew, etc.)."; exit 1; }
	@which ffmpeg > /dev/null || { echo "ffmpeg not found. Please install it using your package manager (e.g., apt, brew, etc.)."; exit 1; }
	@which git > /dev/null || { echo "git not found. Please install it using your package manager (e.g., apt, brew, etc.)."; exit 1; }
	@which cmake > /dev/null || { echo "cmake not found. Please install it using your package manager (e.g., apt, brew, etc.)."; exit 1; }

# Clone the repository if not already cloned
clone:
	@if [ ! -d "$(REPO_DIR)" ]; then \
		echo "Cloning whisper.cpp repository..."; \
		git clone $(REPO_URL); \
	else \
		echo "Repository already cloned."; \
	fi

# Build the project
build: clone
	@cd $(REPO_DIR) && cmake -B build && cmake --build build --config Release

# Download the model
model: clone
	@cd $(REPO_DIR) && sh ./$(MODEL_SCRIPT) $(MODEL)

# Run the transcription example
run: build model
	@cd $(REPO_DIR) && ./build/bin/main -f samples/jfk.wav

# Clean up build files
clean:
	@if [ -d "$(REPO_DIR)" ]; then \
		echo "Cleaning up build files..."; \
		cd $(REPO_DIR) && rm -rf build; \
	else \
		echo "Repository not found. Nothing to clean."; \
	fi

# Default target: ensure dependencies, clone, build, download model, and run
all: check-deps build model run

