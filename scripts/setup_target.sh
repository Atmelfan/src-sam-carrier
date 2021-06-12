#!/usr/bin/env bash

NVM_USER=sam

# Run as root
echo "Running on $HOSTNAME"
if [[ $(id -u) -ne 0 ]] ; then
  echo "Please run as root"
  exit 1
fi

# Headless mode
echo "-- $HOSTNAME: Disabling graphics..."
systemctl stop gdm3
systemctl disable gdm3
systemctl set-default multi-user.target

# Create "sam" user
if id "$NVM_USER" &>/dev/null; then
  echo "-- $HOSTNAME: User already exists, skipping..."
else
  echo "-- $HOSTNAME: Creating user $NVM_USER..."
  adduser "$NVM_USER"
fi
