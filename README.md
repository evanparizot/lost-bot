# shot-bot

1. Deploy by running `./deploy`
1. Service file is at `/lib/systemd/system/shot-bot.service`
1. Reload with `sudo systemctl daemon-reload`
1. Enable with `sudo systemctl enable shot-bot.service`
1. Start with `sudo systemctl start shot-bot.service`
1. Check status with `sudo systemctl status shot-bot.service`
1. Check logs with `journalctl --unit=shot-bot.service`
