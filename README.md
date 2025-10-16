# hostprint
As a system admin,


## General Goals
Tell me what this machine is right now distro, architecture, virtualization/container state.

Tell me what’s running & reachable services, open ports, reverse proxies, VPNs.

Tell me what’s installed & how package managers, packages, custom-built binaries.

Tell me what’s wrong or suspicious insecure permissions, exposed keys, weird processes, honeypot signs.

Provide actionable context for support/AI prioritized summary, raw data, and diffable JSON for future comparisons.


## General TODO:
[ ]if I am given a server, that I need to check out that has no documentation, I want to know the following:

[ ]Which operating system does it use,

[ ]Which packages are installed and their versions
[ ]	  is there video player or minecraft or vs code installed on the server
[ ]	  I want to flag any important software
[ ]		  vulnerable versions


[ ]I need to figure out if its got flatpaks, snaps, aur and or  common package managers installed
[ ]	Per each package manager found, check installed binaries and their versions

[ ]Find out if there are binaries installed by other means, such as a local build or simple scripts

[ ]I want to gather up any read me file that may be present in some common directories with their locations and date.

[ ] I want to know which firewall is active,
[ ]	  then list firewall rules. 

[ ]I want to know the architecture of the server
[ ]	 Is it bare metal, vm honey pot

[ ] I want to know which services are running,
[ ]	I want to flag important services
[ ]		Docker
[ ]		 virsh
[ ]		 firewall
[ ]		 anything malicious


[ ] I want to know hardware info,
[ ]	  storage
[ ]	  ram
[ ]	  gpu
[ ]	  cpu
[ ]	  any peripherals
[ ]	  motheboard
[ ]	  disk

[ ]I want to know the network configurations
[ ]	Which ports are exposed
[ ]	 what are the firewall rules
[ ]	 what type of networking is there
[ ]	 is it behind a reverse proxy, or proxy
[ ]	 what are current settings
[ ]	 any vpns that are installed
[ ]		 wireguard
		 
[ ]I want to know the logs if they are not too big

[ ]I want to audit for any bad files or files of interest
[ ]	bad permissions eg 777
[ ]	 password.txt
[ ]	 private keys
