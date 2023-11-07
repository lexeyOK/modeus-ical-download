# Modeus calendar telegram-bot

This bot has list of names to whom it sends .isc files on specific datetime.

flow chart for this bot:
```mermaid
flowchart TD
    Start["/start"] --> deside{Is in user list?}
    deside -->|No|fail[send: 
    You're not in users list]
    deside -->|Yes|help[send:
    This bot will send you .ics file each Sunday at 12:00am]
    help --> loop[send ical fliles each sunday]
    subgraph loop:
        loop -->download[download files for each user in list]
        download --> send[send ical files to each users]
    end
```

To configure it set env vars `USERNAME` and `PASSWORD` to appropriate values.
Set `users.csv` file with telegram usernames.