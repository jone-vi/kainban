# ---------------------------------------------------------
# Codex MCP Agent Dockerfile
# ---------------------------------------------------------
FROM python:3.11-slim

# Install tools needed for node + npx + codex
RUN apt-get update && apt-get install -y \
    curl \
    bash \
    git \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Node 18+ (NodeSource)
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g npm

# Install Codex CLI globally
# (This ensures npx codex runs correctly)
RUN npm install -g -g @openai/codex

# Create workspace
WORKDIR /agent/codex

# Copy requirements + install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy the MCP server code
COPY codex_mcp.py .

# Default command: Run the MCP server over stdio
CMD ["python", "codex_mcp.py"]
