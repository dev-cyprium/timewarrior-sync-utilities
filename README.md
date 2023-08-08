# Timew extension: sync time

This extension allows you to sync your timewarrior database with a remote server. It's a simple wrapper around the `timew` executable written in rust.

## Caveats

Full server implementation exists in this [repo](https://github.com/timewarrior-synchronize/timew-sync-server) however this is a more simplified version for home networks. One of the original use cases was to use it with an Asus FTP server on a home network inside the router.

This implementation takes a different approach, instead of `go` it's written in rust, and it's decentralized and doesn't require authorization. It's meant to be used in a home network where you trust all the devices.

## Planned features

- [x] Upload file to FTP server
- [x] Download file from FTP server
- [ ] Auto sync file with FTP server on specific commands
- [ ] Web interface to interact with data
- [ ] Different storage options (FTP, SFTP, Dropbox, Google Drive, ssh, etc)
