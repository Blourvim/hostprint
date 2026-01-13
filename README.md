# hostprint: Agentless Linux Server Inspection over SSH

**hostprint** is a lightweight, agentless Linux system inspection tool that connects over SSH and snapshots a server’s state into structured JSON. It is designed for quickly understanding undocumented, inherited, or unfamiliar Linux servers without installing agents or modifying system state.

This project is particularly useful for infrastructure audits, incident response, server discovery, environment documentation, and as structured input for automation or LLM-based analysis.

---

## What hostprint Does

hostprint connects to a remote Linux host via SSH, executes a curated set of read-only system commands, parses their output, and produces a normalized JSON snapshot describing the machine’s current state.

The snapshot may include:

* Operating system, kernel, architecture, and environment details
* Hardware and virtualization signals
* Installed packages and software inventory
* Running services and processes
* Network configuration, listening ports, and interfaces
* Basic security-relevant signals and anomalies

The resulting JSON is intended to be:

* Human-readable
* Diffable over time
* Machine-consumable by other tools, pipelines, or LLMs

> **Status:** Alpha software. Interfaces, output schemas, and coverage may change without notice.

---

## Key Characteristics

* SSH-only access, no daemons or agents
* Does not make changes to the target system
* Avoids raw shell dumps
* Relies on standard utilities as much as possible

---

## Use Cases

hostprint is optimized for scenarios such as:

* Auditing unknown or inherited Linux servers
* Generating baseline system documentation
* Comparing server state across time or environments
* Feeding structured host data into analysis or AI systems

It is explicitly **not** designed to replace configuration management, monitoring, security scanning tools or for compliance assistance.

---

## Goals

Given a Linux server with little or no documentation, hostprint aims to answer:

* What operating system, kernel, and architecture is running?
* What software is installed, and via which package systems?
* What services are active and which ports are exposed?
* What configurations or signals appear unusual or risky?
* How can this state be captured, diffed, and analyzed later?

---

## Non-Goals

hostprint intentionally does **not** attempt to be:

* A vulnerability scanner or penetration testing tool
* A monitoring or observability agent
* A compliance or policy auditing solution
* A configuration management system
* A source of guaranteed completeness or correctness

It reports only what can be observed through standard user-accessible system tooling over SSH.

---

## Installation

```bash
git clone https://github.com/blourvim/hostprint.git
cd hostprint
cargo build --release
```

The resulting binary will be located at:

```bash
target/release/hostprint
```

---

## Usage

```bash
./hostprint \
  --address 10.0.0.5 \
  --port 22 \
  --username debian \
  --key ~/.ssh/id_rsa
```

The tool connects to the specified host, performs inspection, and emits structured JSON describing the system state.

---

## Output

hostprint produces normalized JSON and serves an html file suitable for:

* Manual inspection
* Version control and diffs
* Automated pipelines
* Programmatic consumption by other tools

Exact fields and schemas may evolve as the project matures.

---

## Project Status

hostprint is under active development and should be considered experimental. Feedback, issues, and contributions are welcome.

License: MIT
