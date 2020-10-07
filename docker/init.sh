#!/bin/sh

# Start syslog coupled with stdout of PID 1
syslogd -n -S -O- &

# Create a named pipe which the logger consumes and writes to syslog
PIPE=$(mktemp -u)
mkfifo $PIPE
logger <$PIPE &

# Exec the process supervisor with redirected stdout/err to named pipe
exec 1>$PIPE
exec 2>$PIPE
exec runsvdir /etc/sv