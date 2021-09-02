# ImgSync

## What is this?

Auto image uploader wrote in Rust.

ImgSync uploads image files that are newly added in an observed directory to slack automatically.

## How to install

compiling this program.

```bash
git clone git@github.com:bean1310/ImgSync.git
cd ImgSync
make
```

install (for systemd system)

```bash
sudo make install-install
```

install (to .cargo dir)

```bash
make install
```

## Configuration

ImgSync configulation file is `/etc/img_sync`.

```txt
[basic]
dir=
[slack]
token=
channel_id=
```

### `[basic]`

- `dir`: An obserding directory. (e.g. /home/user/Pictures/ScreenShots/)

### `[slack]`

- token: Slack token
- channel_id: A slack channel id that post destination.

You can get Slack token from [Slack API](https://api.slack.com).
