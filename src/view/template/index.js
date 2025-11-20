console.log("hello world")
const data = {
  "title": "Linux Server Dashboard",
  "header": "Linux Server Dashboard",
  "menu": { "title": "Dashboard Menu", "items": ["Firewalls", "Access Control", "Hardware", "Packages", "Servers"] },
  "system": [{ "label": "OS", "value": "Ubuntu 24.04 LTS" }, { "label": "Kernel", "value": "6.5.0" }, { "label": "Machine", "value": "x86_64" }, { "label": "Processor", "value": "x86_64" }, { "label": "Version", "value": "#42-Ubuntu SMP" }, { "label": "Node", "value": "server-01" }],
  "users": [{ "username": "root", "uid": 0, "gid": 0, "groups": "root" }, { "username": "ubuntu", "uid": 1000, "gid": 1000, "groups": "ubuntu, sudo" }],
  "loggedIn": [{ "user": "ubuntu", "tty": "pts/0", "login": "08:32", "host": "192.168.1.10" }, { "user": "root", "tty": "tty1", "login": "08:00", "host": "localhost" }],
  "ports": [{ "protocol": "TCP", "port": 22, "service": "sshd", "status": "LISTEN" }, { "protocol": "TCP6", "port": 80, "service": "nginx", "status": "LISTEN" }],
  "interfaces": [{ "iface": "lo", "ip": "127.0.0.1", "mask": "255.0.0.0", "status": "UP" }, { "iface": "eth0", "ip": "192.168.1.50", "mask": "255.255.255.0", "status": "UP" }],
  "generated": "2025-11-07"
}


  (function() {
    try {
      const dataEl = document.getElementById('dashboard-data');
      const data = JSON.parse(dataEl.textContent.trim());
      const tplEl = document.getElementById('dashboard-template');
      const template = Handlebars.compile(tplEl.innerHTML);
      document.getElementById('app').innerHTML = template(data);
      if (data.title) document.title = data.title;

      const copyBtn = document.getElementById('copy-dashboard');
      const feedback = document.getElementById('copy-feedback');
      copyBtn.addEventListener('click', () => {
        feedback.style.opacity = '1';
        setTimeout(() => { feedback.style.opacity = '0'; }, 1500);
      });
    } catch (err) {
      const app = document.getElementById('app') || document.body;
      app.innerHTML = '<pre style="color: #900; padding: 16px;">Error rendering dashboard:\n' + String(err) + '</pre>';
      console.error(err);
    }
  })();
