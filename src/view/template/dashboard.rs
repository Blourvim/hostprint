use crate::model::host::Host;
use handlebars::Handlebars;
use serde_json::json;

pub fn generate_dashboard(host: Host) -> String {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("dashboard", include_str!("index.hbs"))
        .unwrap();

    let os_name = host.os.as_ref().and_then(|o| o.name.clone()).unwrap_or_default();
    let os_version = host.os.as_ref().and_then(|o| o.version.clone()).unwrap_or_default();
    let kernel = host.os.as_ref().and_then(|o| o.kernel.clone()).unwrap_or_default();
    let cpu_model = host.hardware.as_ref().and_then(|h| h.cpu_model.clone()).unwrap_or_default();
    let memory = host.hardware.as_ref().and_then(|h| h.memory_total_kb).map(|m| format!("{} MB", m / 1024)).unwrap_or_default();
    let uptime = host.hardware.as_ref().and_then(|h| h.uptime_seconds).map(|u| format!("{}s", u)).unwrap_or_default();

    let system = vec![
        json!({"label": "OS", "value": format!("{} {}", os_name, os_version)}),
        json!({"label": "Kernel", "value": kernel}),
        json!({"label": "CPU", "value": cpu_model}),
        json!({"label": "Memory", "value": memory}),
        json!({"label": "Uptime", "value": uptime}),
    ];

    let ports: Vec<_> = host.sockets.as_ref().map(|sockets| {
        sockets.iter().map(|s| {
            json!({
                "service": format!("PID: {}", s.process.pid.unwrap_or(0)),
                "protocol": format!("{:?}", s.protocol),
                "port": s.port,
                "status": s.state
            })
        }).collect()
    }).unwrap_or_default();

    let users: Vec<_> = host.users.as_ref().map(|users| {
        users.iter().map(|u| {
            json!({
                "username": u.name,
                "uid": u.uid,
                "gid": u.gid,
                "groups": u.groups.as_ref().map(|g| g.iter().map(|grp| grp.name.clone().unwrap_or_default()).collect::<Vec<_>>().join(", ")).unwrap_or_default()
            })
        }).collect()
    }).unwrap_or_default();

    let logged_in: Vec<_> = host.sessions.as_ref().map(|sessions| {
        sessions.iter().map(|s| {
            json!({
                "user": s.username,
                "tty": s.tty,
                "login": s.login_at,
                "host": s.from
            })
        }).collect()
    }).unwrap_or_default();

    let interfaces: Vec<_> = host.hardware.as_ref().and_then(|h| h.network_interfaces.as_ref()).map(|interfaces| {
        interfaces.iter().map(|i| {
            json!({
                "iface": i.name,
                "ip": i.ipv4_addresses.as_ref().and_then(|ips| ips.first().cloned()).unwrap_or_default(),
                "mask": "N/A",
                "status": if i.is_up.unwrap_or(false) { "UP" } else { "DOWN" }
            })
        }).collect()
    }).unwrap_or_default();

    let data = json!({
        "menu": {
            "title": "HostPrint",
            "items": ["Dashboard", "Settings", "Help"]
        },
        "header": "System Dashboard",
        "system": system,
        "ports": ports,
        "users": users,
        "loggedIn": logged_in,
        "interfaces": interfaces,
        "generated": "Now"
    });

    handlebars.render("dashboard", &data).unwrap_or_else(|e| format!("Error rendering template: {}", e))
}
