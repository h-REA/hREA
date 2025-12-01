# hREA Deployment Guide

## Overview

This guide covers various deployment scenarios for hREA, from development setups to production environments. hREA can be deployed as a standalone application, as part of a larger system, or in various network configurations.

## Deployment Architectures

### 1. Single Node Development

**Use Case**: Local development, testing, proof-of-concept
- **Components**: Single Holochain conductor with hREA DNA
- **Database**: Local Holochain storage
- **Network**: No network peers required
- **Management**: Simple start/stop commands

```
┌─────────────────────────────────┐
│     Development Machine        │
│  ┌─────────────────────────┐    │
│  │     Holochain           │    │
│  │     Conductor           │    │
│  │  ┌─────────────────┐    │    │
│  │  │   hREA DNA      │    │    │
│  │  └─────────────────┘    │    │
│  └─────────────────────────┘    │
│  ┌─────────────────────────┐    │
│  │   Web Interface         │    │
│  │  (Development UI)       │    │
│  └─────────────────────────┘    │
└─────────────────────────────────┘
```

### 2. Small Team/Small Business

**Use Case**: Small organization, team collaboration
- **Components**: Multiple conductors, one per team member
- **Network**: Peer-to-peer network via bootstrap server
- **Storage**: Distributed across team members
- **Redundancy**: High (each member stores data)

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Team Member   │    │   Team Member   │    │   Team Member   │
│       A         │    │       B         │    │       C         │
│  ┌───────────┐  │    │  ┌───────────┐  │    │  ┌───────────┐  │
│  │Conductor  │◄─┼────┼──►│Conductor  │◄─┼────┼──►│Conductor  │  │
│  │  + hREA   │  │    │  │  + hREA   │  │    │  │  + hREA   │  │
│  └───────────┘  │    │  └───────────┘  │    │  └───────────┘  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         ▲                                              ▲
         │                                              │
         └──────────────────────────────────────────────┘
                    Bootstrap Server (Optional)
```

### 3. Enterprise/Production

**Use Case**: Large organizations, production systems
- **Components**: Dedicated servers, load balancers, monitoring
- **Network**: Private bootstrap servers, firewalled peers
- **Storage**: Distributed with backup strategies
- **High Availability**: Multiple conductor instances

```
Internet
    │
    ▼
┌─────────────────────────────────────────────────────────┐
│                  Load Balancer                          │
└─────────────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────────────┐
│                Web Server (UI/API)                      │
└─────────────────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────────────────┐
│               Holochain Network                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │ Conductor 1 │  │ Conductor 2 │  │ Conductor N │    │
│  │ (Primary)   │  │ (Secondary) │  │ (Backup)    │    │
│  └─────────────┘  └─────────────┘  └─────────────┘    │
└─────────────────────────────────────────────────────────┘
```

## System Requirements

### Minimum Requirements

| Component | CPU | Memory | Storage | Network |
|-----------|-----|--------|---------|---------|
| Single Node | 2 cores | 4 GB | 20 GB | 10 Mbps |
| Small Team | 4 cores | 8 GB | 100 GB | 100 Mbps |
| Enterprise | 8 cores | 16 GB | 500 GB | 1 Gbps |

### Recommended Requirements

| Component | CPU | Memory | Storage | Network |
|-----------|-----|--------|---------|---------|
| Production | 16 cores | 32 GB | 1 TB SSD | 10 Gbps |

### Software Dependencies

- **Operating System**: Ubuntu 20.04+, CentOS 8+, RHEL 8+, or equivalent
- **Docker**: 20.10+ (for containerized deployment)
- **Node.js**: 18.x LTS (for UI components)
- **Rust**: 1.70+ (for native compilation)

## Installation Methods

### Method 1: Docker Deployment (Recommended)

#### Prerequisites

```bash
# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/download/v2.20.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

#### Quick Start

```bash
# Clone repository
git clone https://github.com/h-REA/hREA.git
cd hREA

# Build and start services
docker-compose up -d

# Verify deployment
docker-compose ps
```

#### Docker Compose Configuration

```yaml
# docker-compose.yml
version: '3.8'

services:
  holochain:
    build: .
    container_name: hrea-holochain
    ports:
      - "8888:8888"  # Admin interface
      - "4242:4242"  # App interface
    volumes:
      - holochain-data:/var/lib/holochain
      - ./config:/app/config
    environment:
      - RUST_LOG=info
      - HC_ADMIN_INTERFACE_PORT=8888
      - HC_APP_INTERFACE_PORT=4242
    restart: unless-stopped

  ui:
    build: ./modules/ui
    container_name: hrea-ui
    ports:
      - "3000:3000"
    environment:
      - REACT_APP_HOLOCHAIN_URL=http://localhost:4242
    depends_on:
      - holochain
    restart: unless-stopped

  nginx:
    image: nginx:alpine
    container_name: hrea-nginx
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
    depends_on:
      - holochain
      - ui
    restart: unless-stopped

volumes:
  holochain-data:
    driver: local
```

#### SSL/HTTPS Configuration

```nginx
# nginx.conf
events {
    worker_connections 1024;
}

http {
    upstream holochain {
        server holochain:4242;
    }

    upstream ui {
        server ui:3000;
    }

    server {
        listen 443 ssl http2;
        server_name your-domain.com;

        ssl_certificate /etc/nginx/ssl/cert.pem;
        ssl_certificate_key /etc/nginx/ssl/key.pem;

        # Proxy UI requests
        location / {
            proxy_pass http://ui;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        # Proxy GraphQL requests
        location /graphql {
            proxy_pass http://holochain;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }
}
```

### Method 2: Native Installation

#### Build from Source

```bash
# Clone repository
git clone https://github.com/h-REA/hREA.git
cd hREA

# Install dependencies
sudo apt update
sudo apt install -y build-essential pkg-config

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Holochain
cargo install holochain --version "0.2.*"

# Build project
cargo build --release

# Build UI
cd modules/ui
npm install
npm run build
cd ../..
```

#### System Service Configuration

```ini
# /etc/systemd/system/hrea.service
[Unit]
Description=hREA Holochain Service
After=network.target

[Service]
Type=simple
User=hrea
Group=hrea
WorkingDirectory=/opt/hrea
ExecStart=/opt/hrea/target/release/holochain --conductor-port 8888
Restart=always
RestartSec=5

# Environment
Environment=RUST_LOG=info
Environment=HC_ADMIN_INTERFACE_PORT=8888
Environment=HC_APP_INTERFACE_PORT=4242

[Install]
WantedBy=multi-user.target
```

```bash
# Create user
sudo useradd -r -s /bin/false hrea

# Create directories
sudo mkdir -p /opt/hrea /var/lib/holochain
sudo chown hrea:hrea /opt/hrea /var/lib/holochain

# Install service
sudo cp hrea.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable hrea
sudo systemctl start hrea
```

## Configuration

### Holochain Conductor Configuration

#### Basic Configuration

```yaml
# conductor-config.yaml
network:
  bootstrap_service: https://bootstrap.holo.host
  transport_pool: [
    {type: "quic"},
    {type: "mem"}
  ]

keystore:
  danger_test_keystore: true

admin_interfaces:
  - driver:
      type: web_socket
      port: 8888
    allowed_origins: "*"

app_interfaces:
  - driver:
      type: web_socket
      port: 4242
    allowed_origins: "*"
```

#### Production Configuration

```yaml
# production-conductor-config.yaml
network:
  bootstrap_service: "https://your-bootstrap-server.com"
  transport_pool: [
    {type: "quic", port: 10001},
    {type: "quic", port: 10002}
  ]
  network_type: "quic_bootstrap_service"

keystore:
  passphrase: "your-secure-passphrase"
  danger_test_keystore: false

admin_interfaces:
  - driver:
      type: web_socket
      port: 8888
    allowed_origins: ["https://your-domain.com"]

app_interfaces:
  - driver:
      type: web_socket
      port: 4242
    allowed_origins: ["https://your-domain.com"]

environment_path: "/var/lib/holochain"
```

### hREA Application Configuration

#### Environment Variables

```bash
# .env file
# Holochain Configuration
HC_ADMIN_INTERFACE_PORT=8888
HC_APP_INTERFACE_PORT=4242
HC_CONDUCTOR_CONFIG_PATH=/opt/hrea/conductor-config.yaml

# Database Configuration
DATABASE_PATH=/var/lib/holochain/database
BACKUP_PATH=/var/backups/holochain

# Logging Configuration
RUST_LOG=info
LOG_FILE=/var/log/hrea.log

# Security Configuration
ENABLE_TLS=true
CERT_PATH=/etc/ssl/certs/hrea.pem
KEY_PATH=/etc/ssl/private/hrea.key

# Performance Configuration
MAX_CONCURRENT_REQUESTS=1000
WEBSOCKET_TIMEOUT=30
```

## Deployment Scripts

### Automated Setup Script

```bash
#!/bin/bash
# deploy.sh - Automated hREA deployment script

set -e

# Configuration
HREA_VERSION="latest"
INSTALL_DIR="/opt/hrea"
SERVICE_USER="hrea"
CONFIG_DIR="/etc/hrea"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] WARNING:${NC} $1"
}

error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ERROR:${NC} $1"
    exit 1
}

# Check if running as root
if [[ $EUID -ne 0 ]]; then
   error "This script must be run as root"
fi

# Install dependencies
log "Installing dependencies..."
apt update && apt install -y curl git build-essential pkg-config

# Create service user
log "Creating service user..."
if ! id "$SERVICE_USER" &>/dev/null; then
    useradd -r -s /bin/false -m -d $INSTALL_DIR $SERVICE_USER
fi

# Download and install hREA
log "Installing hREA..."
cd /tmp
git clone https://github.com/h-REA/hREA.git
cd hREA
cargo build --release

# Install binaries
cp target/release/holochain /usr/local/bin/
cp -r . $INSTALL_DIR
chown -R $SERVICE_USER:$SERVICE_USER $INSTALL_DIR

# Create configuration directory
mkdir -p $CONFIG_DIR
cp conductor-config.yaml $CONFIG_DIR/
chown -R $SERVICE_USER:$SERVICE_USER $CONFIG_DIR

# Install systemd service
log "Installing systemd service..."
cp hrea.service /etc/systemd/system/
systemctl daemon-reload
systemctl enable hrea

# Create log directory
mkdir -p /var/log/hrea
chown $SERVICE_USER:$SERVICE_USER /var/log/hrea

log "Installation completed successfully!"
log "Start the service with: systemctl start hrea"
log "Check status with: systemctl status hrea"
```

### Health Check Script

```bash
#!/bin/bash
# health-check.sh - hREA health monitoring

ADMIN_PORT=8888
WEBHOOK_URL="https://your-monitoring-service.com/webhook"

check_service() {
    local service=$1
    local port=$2

    if nc -z localhost $port; then
        log "✓ $service is running on port $port"
        return 0
    else
        error "✗ $service is not responding on port $port"
        return 1
    fi
}

check_holochain_admin() {
    local response=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:$ADMIN_PORT/agents)

    if [[ $response == "200" ]]; then
        log "✓ Holochain admin interface responding"
        return 0
    else
        warn "Holochain admin interface returned HTTP $response"
        return 1
    fi
}

send_alert() {
    local message=$1
    curl -X POST -H 'Content-type: application/json' \
         --data "{\"text\":\"hREA Alert: $message\"}" \
         $WEBHOOK_URL
}

# Main health check
log "Starting hREA health check..."

# Check service ports
services_healthy=true

if ! check_service "Holochain Admin" $ADMIN_PORT; then
    services_healthy=false
fi

if ! check_holochain_admin; then
    services_healthy=false
fi

# Check disk space
disk_usage=$(df /var/lib/holochain | awk 'NR==2 {print $5}' | sed 's/%//')
if [[ $disk_usage -gt 90 ]]; then
    warn "Disk usage is at ${disk_usage}%"
    services_healthy=false
fi

# Check memory usage
memory_usage=$(free | awk 'NR==2{printf "%.0f", $3*100/$2}')
if [[ $memory_usage -gt 90 ]]; then
    warn "Memory usage is at ${memory_usage}%"
    services_healthy=false
fi

# Send alerts if needed
if [[ $services_healthy == false ]]; then
    send_alert "hREA health check failed on $(hostname)"
    exit 1
fi

log "All health checks passed"
exit 0
```

### Backup Script

```bash
#!/bin/bash
# backup.sh - hREA data backup script

BACKUP_DIR="/var/backups/holochain"
SOURCE_DIR="/var/lib/holochain"
RETENTION_DAYS=30
S3_BUCKET="s3://your-backup-bucket/hrea"

# Create backup directory
mkdir -p $BACKUP_DIR

# Create timestamp
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
BACKUP_FILE="hrea_backup_${TIMESTAMP}.tar.gz"

# Stop service
log "Stopping hREA service..."
systemctl stop hrea

# Create backup
log "Creating backup..."
tar -czf "${BACKUP_DIR}/${BACKUP_FILE}" -C $(dirname $SOURCE_DIR) $(basename $SOURCE_DIR)

# Restart service
log "Starting hREA service..."
systemctl start hrea

# Verify backup
if [[ -f "${BACKUP_DIR}/${BACKUP_FILE}" ]]; then
    log "Backup created successfully: ${BACKUP_FILE}"

    # Upload to S3 if configured
    if [[ -n $S3_BUCKET ]]; then
        log "Uploading backup to S3..."
        aws s3 cp "${BACKUP_DIR}/${BACKUP_FILE}" "$S3_BUCKET/"
    fi
else
    error "Backup creation failed"
fi

# Clean old backups
log "Cleaning old backups..."
find $BACKUP_DIR -name "hrea_backup_*.tar.gz" -mtime +$RETENTION_DAYS -delete

log "Backup completed"
```

## Monitoring and Maintenance

### System Monitoring

#### Prometheus Configuration

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'hrea-holochain'
    static_configs:
      - targets: ['localhost:9464']
    metrics_path: /metrics
    scrape_interval: 30s

  - job_name: 'hrea-ui'
    static_configs:
      - targets: ['localhost:3000']
    metrics_path: /metrics
    scrape_interval: 30s
```

#### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "hREA Monitoring",
    "panels": [
      {
        "title": "Holochain Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(holochain_requests_total[5m])",
            "legendFormat": "{{method}}"
          }
        ]
      },
      {
        "title": "Memory Usage",
        "type": "graph",
        "targets": [
          {
            "expr": "process_resident_memory_bytes / 1024 / 1024",
            "legendFormat": "Memory (MB)"
          }
        ]
      }
    ]
  }
}
```

### Log Management

#### Log Rotation

```bash
# /etc/logrotate.d/hrea
/var/log/hrea/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 644 hrea hrea
    postrotate
        systemctl reload hrea
    endscript
}
```

#### Centralized Logging

```yaml
# filebeat.yml
filebeat.inputs:
- type: log
  enabled: true
  paths:
    - /var/log/hrea/*.log
  fields:
    service: hrea
    environment: production

output.elasticsearch:
  hosts: ["elasticsearch:9200"]
  index: "hrea-%{+yyyy.MM.dd}"
```

## Security Configuration

### Network Security

#### Firewall Configuration

```bash
# UFW rules
sudo ufw allow 22/tcp      # SSH
sudo ufw allow 80/tcp      # HTTP
sudo ufw allow 443/tcp     # HTTPS
sudo ufw allow 8888/tcp    # Holochain admin (internal only)
sudo ufw allow 4242/tcp    # Holochain app (internal only)

# Restrict admin interface to internal network
sudo ufw allow from 10.0.0.0/8 to any port 8888
sudo ufw allow from 192.168.0.0/16 to any port 8888

sudo ufw enable
```

### SSL/TLS Configuration

#### Let's Encrypt Certificate

```bash
# Install certbot
sudo apt install certbot python3-certbot-nginx

# Generate certificate
sudo certbot --nginx -d your-domain.com

# Auto-renewal
sudo crontab -e
# Add: 0 12 * * * /usr/bin/certbot renew --quiet
```

## Troubleshooting

### Common Issues

#### Service Won't Start

```bash
# Check service status
systemctl status hrea

# Check logs
journalctl -u hrea -f

# Check configuration
holochain --help
```

#### Network Connectivity Issues

```bash
# Check port availability
netstat -tlnp | grep :4242
netstat -tlnp | grep :8888

# Test Holochain admin interface
curl -s http://localhost:8888/agents | jq .

# Test app interface
curl -s http://localhost:4242/health
```

#### Performance Issues

```bash
# Check system resources
top
htop
iostat -x 1

# Check Holochain metrics
curl -s http://localhost:9464/metrics
```

### Debug Commands

```bash
# Verbose logging
RUST_LOG=debug hrea

# Check conductor state
hc admin --interface-port 8888 conductor list-apps
hc admin --interface-port 8888 conductor list-dnas

# Check agent status
hc admin --interface-port 8888 agent list

# Dump conductor state
hc admin --interface-port 8888 conductor dump-state
```

## Disaster Recovery

### Backup and Restore

#### Data Recovery

```bash
# Stop service
systemctl stop hrea

# Restore from backup
cd /var/lib/
tar -xzf /var/backups/holochain/hrea_backup_YYYYMMDD_HHMMSS.tar.gz

# Fix permissions
chown -R hrea:hrea holochain

# Start service
systemctl start hrea
```

#### Configuration Recovery

```bash
# Restore conductor configuration
cp /etc/hrea/backup/conductor-config.yaml /etc/hrea/

# Reload service
systemctl reload hrea
```

### High Availability Setup

#### Multiple Conductors

```bash
# Start multiple conductor instances
holochain --conductor-port 8888 --config conductor1.yaml &
holochain --conductor-port 8889 --config conductor2.yaml &
holochain --conductor-port 8890 --config conductor3.yaml &
```

#### Load Balancer Configuration

```nginx
upstream holochain_backend {
    server 127.0.0.1:8888 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8889 weight=1 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8890 weight=1 max_fails=3 fail_timeout=30s;
}

server {
    location /holochain {
        proxy_pass http://holochain_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_connect_timeout 5s;
        proxy_read_timeout 30s;
    }
}
```

## Performance Tuning

### Holochain Optimization

#### Network Configuration

```yaml
# conductor-config.yaml
network:
  transport_pool: [
    {
      type: "quic",
      port: 10001,
      bind_to: "0.0.0.0"
    }
  ]
  bootstrap_service: "https://bootstrap.holo.host"

app_interface_connection_timeout: 30
keep_alive_timeout: 60
```

#### Resource Limits

```yaml
# systemd service limits
[Service]
LimitNOFILE=65536
LimitNPROC=4096
MemoryLimit=2G
CPUQuota=100%
```

### Database Optimization

#### SQLite Configuration

```sql
-- Performance optimizations
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA cache_size = -2000;  -- 2MB cache
PRAGMA temp_store = MEMORY;
PRAGMA mmap_size = 268435456;  -- 256MB memory map
```

## Scaling Considerations

### Horizontal Scaling

#### Multi-Region Deployment

```bash
# Region-specific bootstrap servers
export BOOTSTRAP_SERVERS="https://bootstrap-us.holo.host,https://bootstrap-eu.holo.host"

# Start conductor with region-specific configuration
holochain --config conductor-us.yaml --bootstrap-service $BOOTSTRAP_SERVERS
```

#### Load Distribution

```yaml
# docker-compose.yml (multi-node)
version: '3.8'

services:
  holochain-1:
    extends:
      file: docker-compose.yml
      service: holochain
    environment:
      - NODE_ID=node-1
      - BOOTSTRAP_PEERS=node-2:10001,node-3:10001

  holochain-2:
    extends:
      file: docker-compose.yml
      service: holochain
    environment:
      - NODE_ID=node-2
      - BOOTSTRAP_PEERS=node-1:10001,node-3:10001
```

### Vertical Scaling

#### Resource Allocation

```yaml
# Kubernetes deployment
apiVersion: apps/v1
kind: Deployment
metadata:
  name: hrea
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: hrea
        image: hrea:latest
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "4Gi"
            cpu: "2000m"
```

This deployment guide provides comprehensive instructions for deploying hREA in various environments. Choose the deployment method that best fits your requirements and scale.