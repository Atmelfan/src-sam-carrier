#!/usr/bin/env bash
echo "Running on $HOSTNAME"
sudo su -

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
