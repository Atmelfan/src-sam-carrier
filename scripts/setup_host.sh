#!/usr/bin/env bash

if [ $# -lt 3 ]; then
    echo "Usage: $0 USER HOST"
fi

NVM_USER="$1"
NVM_HOST="$2"

echo "HOST: Setting up remote user"
ssh -tt "$NVM_USER@$NVM_HOST" "sudo -i" << "EOF"
#!/usr/bin/env bash
echo "Running on $HOSTNAME"

# Headless mode
echo "$HOSTNAME: Disabling graphics..."
systemctl stop gdm3
systemctl disable gdm3
systemctl set-default multi-user.target

# Create "sam" user
if id "$1" &>/dev/null; then
  echo "$HOSTNAME: User already exists, skipping..."
else
  echo "$HOSTNAME: Creating user..."
  adduser "$USER"
fi

EOF