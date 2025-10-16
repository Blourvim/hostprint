# hostprint
hostprint (alpha) is an SSH-based system probe for quickly mapping out information on a server.
Currently in very early development expect rough edges, missing features, and experimental outputs.

The goal: Make it simple to snapshot the state of a server ,packages, users, services, and configs, to provide context for AI assistants and generate documentation.

Feel free to open a issue with feature requests.

## Getting Started

## Install
```bash
  git clone https://github.com/blourvim/hostprint.git
  cd hostprint
  cargo build --release

```

# Example Usage
```bash
    ./hostprint --address 10.0.0.5 --port 22 --key ~/.ssh/id_rsa --username debian
```




## General Goals
Tell me what this machine is right now distro, architecture, virtualization/container state.

Tell me what’s running & reachable services, open ports, reverse proxies, VPNs.

Tell me what’s installed & how package managers, packages, custom-built binaries.

Tell me what’s wrong or suspicious insecure permissions, exposed keys, weird processes, honeypot signs.

Provide actionable context for support/AI prioritized summary, raw data, and diffable JSON for future comparisons.


## General TODO

- [ ] If I am given a server that has no documentation, I want to know the following:

---

### System Information
- [x] Which operating system does it use

- [x] Which packages are installed and their versions  
- [ ] Check if there are potentially unnecessary packages installed on the server (e.g., video player, Minecraft, or VS Code)  
- [ ] Flag any important software  
- [ ] Check vulnerable versions against a CVE list  

---

### Package Management
- [x] Figure out which package managers are installed  
- [x] For each package manager found, check installed binaries and their versions  

- [ ] Find out if there are binaries installed by other means (e.g., local builds or custom scripts)  

---

### Documentation
- [ ] Gather any README files that may be present in common directories  
- [ ] Record their locations and modification dates  

---

### Firewall & Security
- [x] Identify which firewall is active  
- [ ] List all firewall rules  

---

### System Architecture
- [ ] Determine the architecture of the server  
- [ ] Check if it is bare metal, virtual machine, or a honeypot  

---

### Running Services
- [x] Identify running services
- [ ] Flag important or sensitive services  
  - [ ] Docker  
  - [ ] Virsh  
  - [ ] Anything malicious or suspicious  

---

### Hardware Information
- [x] Collect hardware information, including:  
  - [x] Storage  
  - [x] RAM  
  - [x] GPU  
  - [x] CPU  
  - [x] Peripherals  
  - [x] Motherboard  
  - [x] Disk  

---

### Network Configuration
- [ ] Identify network configurations:  
  - [ ] Which ports are exposed  
  - [ ] Firewall rules  
  - [ ] Type of networking (bridged, NAT, etc.)  
  - [ ] Whether it is behind a reverse proxy or proxy  
  - [ ] Current network settings  
  - [ ] Installed VPNs  
    - [ ] WireGuard  

---

### Logs
- [ ] Review system and application logs (if not too large)  

---

### File System Audit
- [ ] Audit for bad files or files of interest  
  - [ ] World-writable or misconfigured permissions (e.g., 777)  
  - [ ] `password.txt` or similar sensitive files  
  - [ ] Private keys or credentials  
