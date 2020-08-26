# greenery-rust

- Running on local:
  - Install rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - Add rust to env: `vim ~/.bashrc` add `export PATH="$HOME/.cargo/bin:$PATH"`
  - Run `cargo run`
    - You may need to run `sudo apt install gcc-multilib` and `sudo apt install libssl-dev`

- Running on server
  - SSH to the server `ssh -i <pem> ec2-user@<ip>`
  - Switch to root `sudo su`
  - install libssl `yum install openssl11` 
  - Install rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - Add rust to env: `vim ~/.bashrc` add `export PATH="$HOME/.cargo/bin:$PATH"`
    - `source ~/.bashrc`
  - Deploy artifact to ec2: `scp -i ~/Downloads/greenery-ec2.pem target/debug/greenery-rust ec2-user@35.163.148.219:/home/ec2-user/`
  - Move to root dir: `mv /home/ec2-usr/greenery-rust /root/
  - Deploy `runGreenery.sh` the same way if needed
  - Add command to crontab `crontab -e` if needed
    - Command: `*/3 * * * * bash /root/runGreenery.sh >> /root/cron_log.txt 2>&1`
    - Wait for the crontab to kick it off
  - If the app is already running
    - Kill it `ps -elf | grep greenery`, grab the pid, then `kill <pid>`
    - Wait for the crontab to kick it off
