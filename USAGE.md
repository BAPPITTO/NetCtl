# NetCtl Usage Guide

Practical guide to using NetCtl for network management and QoS control.

## Initial Setup

### Option 1: Interactive TUI Setup Wizard (NEW - Recommended)

Run the interactive setup wizard on Linux:

```bash
cd /Users/jerichofoster/NetCtl/backend
sudo cargo run --bin netctl-tui
```

The TUI guides you through 8 configuration steps:

1. **Welcome** - Introduction and feature overview
2. **Interface Selection** - Choose network interface (eth0, wlan0, etc.)
3. **IP Configuration** - Set IP address, netmask, gateway
4. **DNS Configuration** - Configure primary and secondary DNS
5. **Dashboard Setup** - Configure hostname and admin credentials
6. **Security Review** - Review security settings
7. **Configuration Summary** - Verify all settings
8. **Installation Complete** - Next steps

**Navigation:**

- Tab/Right Arrow: Next screen
- Shift+Tab/Left Arrow: Previous screen
- Enter: Confirm
- Q/Esc: Back/Exit

### Option 2: Package Installation (NEW)

**On Ubuntu/Debian:**

```bash
sudo apt install ./netctl_1.0.0-1_amd64.deb
sudo netctl-tui  # Run TUI setup
sudo systemctl start netctl
```

**On CentOS/RHEL:**

```bash
sudo rpm -i netctl-1.0.0-1.el8.x86_64.rpm
sudo netctl-tui  # Run TUI setup
sudo systemctl start netctl
```

## Starting Up

### Quick Start

```bash
cd /Users/jerichofoster/NetCtl
bash start.sh
```

This starts both backend and frontend in development mode.

### Manual Start

**Terminal 1 - Backend:**

```bash
cd /Users/jerichofoster/NetCtl/backend
RUST_LOG=info cargo run
```

**Terminal 2 - Frontend:**

```bash
cd /Users/jerichofoster/NetCtl/frontend
npm run dev
```

Then open <http://localhost:5173> in your browser.

## Dashboard Features

### Overview Tab

Shows system statistics:

- Connected devices count
- Total aggregate rate limit
- Packets dropped
- Active VLANs

### Devices Tab

Displays all managed devices with:

- Device name and MAC address
- Assigned VLAN
- Current rate limit
- Status (ACTIVE or BLOCKED)
- Last seen timestamp

### VLANs Tab

Manage virtual networks:

- View all VLANs with subnet info
- Create new VLANs
- Delete VLANs
- See DHCP status per VLAN

### QoS Tab

Control bandwidth and blocking:

- Create rate limiting rules
- Block specific devices
- Remove rules
- View active rules

### LAN Dashboard Configuration Tab (NEW)

Configure web dashboard for LAN access:

- **Hostname:** Set FQDN (e.g., netctl.local)
- **Port:** Configure access port (default 443 for HTTPS)
- **HTTPS:** Enable/disable SSL with self-signed certificates
- **DNS Verification:** Test DNS resolution
- **Loop Detection:** Prevents configuration errors

**Configuration Steps:**

1. Set desired hostname
2. Optionally change port
3. Toggle HTTPS (recommended)
4. Click "Verify DNS Configuration"
5. If DNS resolves correctly, click "Save Configuration"

**DNS Loop Detection:**

- Prevents hostname from resolving back to dashboard IP
- Warns if misconfiguration detected
- Suggests /etc/hosts entry as fallback

## Common Workflows

### Workflow 1: Create Guest Network

1. **Open Dashboard** → navigate to VLANs tab
2. **Click "+ Create VLAN"**
3. **Fill form:**
   - VLAN ID: `20`
   - Name: `Guest WiFi`
   - Subnet: `192.168.20.0/24`
   - Gateway: `192.168.20.1`
   - DHCP: Enabled
4. **Click Create**
5. **Wait** for system to configure (2-5 seconds)
6. **Verify:** Check VLANs tab shows your new VLAN

Via API:

```bash
curl -X POST http://localhost:3001/api/vlan \
  -H "Content-Type: application/json" \
  -d '{
    "vlan_id": 20,
    "name": "Guest WiFi",
    "subnet": "192.168.20.0/24",
    "gateway": "192.168.20.1",
    "dhcp_enabled": true
  }'
```

### Workflow 2: Register and Rate-Limit a Device

1. **Click Devices tab**
2. **Device appears** in list as system detects it, or...
3. **Navigate to QoS tab**
4. **In form, enter:**
   - MAC: `aa:bb:cc:dd:ee:ff`
   - Rate Limit: `100` (Mbps)
5. **Click Set Rule**
6. **Verify:** Rule appears in Active Rules table
7. **Device:** Now limited to 100 Mbps

Via API:

```bash
# Set 100 Mbps limit
curl -X POST http://localhost:3001/api/qos/device/aa:bb:cc:dd:ee:ff \
  -H "Content-Type: application/json" \
  -d '{"mac": "aa:bb:cc:dd:ee:ff", "rate_mbps": 100}'

# Block device (rate = 0)
curl -X POST http://localhost:3001/api/qos/device/aa:bb:cc:dd:ee:ff \
  -H "Content-Type: application/json" \
  -d '{"mac": "aa:bb:cc:dd:ee:ff", "rate_mbps": 0}'
```

### Workflow 3: Monitor Real-Time Metrics

1. **Dashboard** auto-updates every 2 seconds
2. **Watch Overview tab** for live stat changes
3. **Or use API** for custom monitoring:

```bash
# Get one-time snapshot
curl http://localhost:3001/api/metrics/summary

# Stream real-time (Node.js example)
const EventSource = require('eventsource');
const es = new EventSource('http://localhost:3001/api/metrics/stream');
es.onmessage = (e) => {
  console.log(JSON.parse(e.data));
};
```

### Workflow 4: Isolate a Problematic Device

**Scenario:** One device is using too much bandwidth

1. **Identify device** via Devices tab or by MAC
2. **Apply strict rate limit:**
   - Enter MAC in QoS form
   - Set rate to `25` Mbps (or lower)
   - Click Set Rule
3. **If still problematic, block it:**
   - Change rate to `0`
   - Click Set Rule again
4. **Verify** device can't communicate anymore
5. **Remove rule** to restore access

### Workflow 5: Setup Multiple VLANs for Network Segmentation

**Scenario:** Separate corporate, guest, and IoT traffic

1. **Create VLAN 10:**
   - Name: "Corporate"
   - Subnet: 192.168.10.0/24
   - Gateway: 192.168.10.1

2. **Create VLAN 20:**
   - Name: "Guest"
   - Subnet: 192.168.20.0/24
   - Gateway: 192.168.20.1

3. **Create VLAN 30:**
   - Name: "IoT Devices"
   - Subnet: 192.168.30.0/24
   - Gateway: 192.168.30.1

4. **Assign devices** to appropriate VLANs (via Devices tab)

5. **Set rate limits per VLAN:**
   - Corporate: 1000 Mbps
   - Guest: 100 Mbps
   - IoT: 50 Mbps

## Advanced Features

### Priority-Based QoS

Set multiple rate limits and priority levels:

```bash
# High priority: 500 Mbps
curl -X POST http://localhost:3001/api/qos/device/aa:bb:cc:dd:ee:01 \
  -d '{"mac": "aa:bb:cc:dd:ee:01", "rate_mbps": 500}'

# Medium priority: 200 Mbps
curl -X POST http://localhost:3001/api/qos/device/aa:bb:cc:dd:ee:02 \
  -d '{"mac": "aa:bb:cc:dd:ee:02", "rate_mbps": 200}'

# Low priority: 50 Mbps
curl -X POST http://localhost:3001/api/qos/device/aa:bb:cc:dd:ee:03 \
  -d '{"mac": "aa:bb:cc:dd:ee:03", "rate_mbps": 50}'
```

### Bulk Configuration via Scripts

```bash
#!/bin/bash
# Create multiple VLANs

create_vlan() {
  curl -X POST http://localhost:3001/api/vlan \
    -H "Content-Type: application/json" \
    -d "{\"vlan_id\": $1, \"name\": \"$2\", \"subnet\": \"$3\", \"gateway\": \"$4\", \"dhcp_enabled\": true}"
}

create_vlan 10 "Corporate" "192.168.10.0/24" "192.168.10.1"
create_vlan 20 "Guest" "192.168.20.0/24" "192.168.20.1"
create_vlan 30 "IoT" "192.168.30.0/24" "192.168.30.1"
```

### Monitoring Integration

Use Prometheus/Grafana with metrics export:

```bash
# Export metrics in Prometheus format
curl http://localhost:3001/api/metrics/summary | jq '.data | to_entries[] | "\(.key)_total \(.value)"'
```

## Dashboard Controls Reference

|Control|Action|Effect|
|---------|--------|---------|
|**Overview** tab|Click|Shows system summary|
|**Create VLAN** button|Click|Opens VLAN creation form|
|**Delete** button|Click on VLAN|Removes VLAN and config|
|**+ Device**|Via API|Registers new device|
|**Set Rule** in QoS|Enter MAC + rate, click|Applies bandwidth limit|
|**Remove** button|Click in QoS table|Removes rate limit|
|**Auto-refresh**|Every 2 seconds|Metrics and state updated|

## Troubleshooting

### Dashboard Won't Load

```bash
# Check backend is running
curl http://localhost:3001/api/health

# Check frontend built
ls frontend/dist/index.html

# Verify proxy config
cat frontend/vite.config.ts | grep proxy
```

### QoS Rules Don't Apply

```bash
# Verify rules are stored
curl http://localhost:3001/api/qos/devices

# Check XDP is attached
cat /sys/kernel/debug/tracing/events/xdp/xdp_redirect/enable

# On Linux, verify kernel version
uname -r  # Should be 5.8+
```

### VLAN Creation Fails

```bash
# Check network interfaces
ip link show

# Verify VLAN creation permissions
id  # Should be root or have CAP_NET_ADMIN

# Try with sudo
sudo /Users/jerichofoster/NetCtl/backend/target/release/netctl-daemon
```

### Metrics Not Updating

```bash
# Verify SSE stream
curl -v http://localhost:3001/api/metrics/stream

# Check browser console for errors
# F12 → Console tab

# Restart backend
pkill netctl-daemon
```

## Performance Tips

1. **Run backend on dedicated machine** if possible (especially for kernel-level operations)

2. **Use rate limits conservatively:**
   - Start high (1000 Mbps)
   - Lower gradually to find sweet spot
   - Monitor metrics for dropped packets

3. **Monitor disk usage:**
   - SQLite database grows over time
   - Clear old audit logs periodically

4. **Network optimization:**
   - Minimize VLAN count (max ~100)
   - Group similar devices together
   - Use DHCP wisely (enables auto-config)

5. **Dashboard optimization:**
   - Close unused browser tabs
   - Reduce update frequency if needed (frontend code)
   - Use API directly for scripts instead of dashboard

## Security Best Practices

1. **Access Control:**
   - Run backend in isolated network segment
   - Use firewall to restrict API access
   - Enable authentication when available (v0.2.0)

2. **Configuration:**
   - Store database in secure location
   - Regular backups of `/tmp/netctl.db`
   - Monitor for unauthorized changes

3. **Monitoring:**
   - Watch metrics for anomalies
   - Set alerts for dropped packets
   - Regular log review

4. **Updates:**
   - Keep system software updated
   - Monitor NetCtl releases for security patches
   - Test updates in staging before production

## Getting Help

- **API Issues:** See [API Documentation](./API.md)
- **Build Problems:** See [Build Guide](./BUILD.md)
- **Logs:** Run with `RUST_LOG=debug` for verbose output
- **Dashboard Errors:** Check browser console (F12)
