import asyncio
import os
from dotenv import load_dotenv

from agents import Agent, Runner, set_default_openai_api
from agents.mcp import MCPServerStdio


def get_api_key():
    secret_file = os.getenv("OPENAI_API_KEY_FILE")
    if secret_file and os.path.exists(secret_file):
        with open(secret_file, "r") as f:
            return f.read().strip()
    return os.getenv("OPENAI_API_KEY")

api_key = get_api_key()
if not api_key:
    raise RuntimeError("OPENAI_API_KEY not set and no secret file found")

os.environ["OPENAI_API_KEY"] = api_key
set_default_openai_api(api_key)

async def main() -> None:
    async with MCPServerStdio(
        name="Codex CLI",
        params={
            "command": "codex",
            "args": ["mcp-server"],
        },
        client_session_timeout_seconds=360000,
    ) as codex_mcp_server:
        print("Codex MCP server started.")

        developer_agent = Agent(
            name="Software Developer",
            instructions=(
                "You are an expert in building software"
                "Always call codex with \"approval-policy\": \"never\" and \"sandbox\": \"workspace-write\""
            ),
            mcp_servers=[codex_mcp_server],
            #model="gpt-5",
        )


        result = await Runner.run(developer_agent, "Just say 'Hello, World! Nothing more!")
        print(result.final_output)


if __name__ == "__main__":
    #asyncio.run(main())
