#!/bin/bash
if [[ ! `/usr/sbin/pidof -s greenery-rust` ]]; then /root/greenery-rust >> /root/greenery_log.txt 2>&1 & fi
