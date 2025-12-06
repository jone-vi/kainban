# Kanban + MCP Agent System

## Architecture
- **Rust Backend (Axum + SQLx)**
- **MySQL Database (separate container)**
- **Multiple MCP Agent Containers**
- **Per-Agent Workspace Volume**
- **DB-Backed Job Queue**
- **Optional Frontend (Kanban UI)**

---

## Backend (Rust)
- MCP client  
- Task delegation  
- Job scheduler  
- Agent registry  
- Heartbeat tracking  
- Load balancing (round-robin / least-busy)  
- Job states: `queued`, `assigned`, `running`, `success`, `failed`, `retrying`  
- API endpoints: tasks, jobs, agents  

---

## Database (MySQL)
- Tables: **agents**, **jobs**, **tasks**, **job_logs**  
- Persistent orchestration  
- Atomic job assignment  
- Retry counters  
- Audit logging  

---

## MCP Agents
- MCP server protocol  
- Tools: `fs`, `git`, `process`, `agent.runTask`  
- Capabilities-based execution  
- Per-agent isolated workspace volume  
- Local repo clone per agent  
- Git workflow: branch → modify → test → commit → push  
- Proved the backend with changes

---

## Workspace
- Separate volume per MCP agent  
- Repo cloned on agent startup  
- Safe isolation (no shared filesystem)  
- Remote sync via Git provider  
- No concurrency conflicts  

---

## Job Queue
- Backend creates jobs  
- Agents **poll** or backend **pushes**  
- Database-driven scheduling  
- Atomic state transitions  
- Logging + retry logic  

---

## System Flow
1. User delegates task  
2. Backend creates **job**  
3. Scheduler selects **available agent**  
4. Agent executes via MCP tools  
5. Agent commits/pushes changes  
6. Backend updates task state  

---

## Advantages
- Microservices architecture  
- Isolation per agent  
- Horizontal scalability  
- Extensibility (add new tools or agents)  
- Stability through DB orchestration  
- Secure sandboxing  
- Clear separation of concerns  
