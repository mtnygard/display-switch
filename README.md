# Turn a $30 USB switch into a full-featured KVM

This utility watches for USB device connect/disconnect events and switches monitor inputs via DDC/CI. This turns
a simple USB switch into a full-fledged KVM solution: press one button on your USB switch and all your monitors
connect to a different input.

It is supposed to be installed on all computers that could be connected to these monitors, since the app only switches
monitors "one way" and relies on itself running on the other computers to switch it "the other way" as needed.
 
## Platforms supported

The app is written twice: once for MacOS (in Swift) and once for Windows (in Rust). The idea is dead simple, but I
haven't found good cross-platform support for DDC, USB, device plug-n-play and power management in any one language
except C, and I didn't want to write this in C.

## Configuration

The configuration is pretty similar on both platforms:

On MacOS: the configuration file is expected in `~/Library/Preferences/display-switch.ini`
On Windows: the configuration file is expected in `/Users/USERNAME/AppData/Roaming/display-switch/display-switch.ini`

Configuration file settings:

```ini
  usb_device = "1050:0407"
  monitor_input = "Hdmi1"
  monitor_description = ""
```

`usb_device` is which USB device to watch (vendor id / device id in hex), and `monitor_input` is which monitor input
to switch to, when this device is connected. Supported values are `Hdmi1`, `Hdmi2`, `DisplayPort1`, `DisplayPort2`
If your monitor has an USB-C port, it's usually reported as `DisplayPort2`

If you have a laptop with a display that doesn't use I2C, you'll get errors from display_switch. Fill in a value for `monitor_description` with the name of the monitor to switch.

### Sample configuration

I have a Razer mouse and a Dell monitor. My desktop is only connected to the Dell monitor so its configuration is:

```ini
usb_device="1532:0029"
monitor_input="Hdmi1"
monitor_description=""
```

My laptop is plugged into HDMI2. The laptop's own display doesn't support I2C or DDC so the configuration I use on the laptop is this:

```ini
usb_device="1532:0029"
monitor_input="Hdmi2"
monitor_description="Dell S3220DGF"
```

## Logging

On MacOS: the log file is written to the system log.
On WindowS: the log file is written to `/Users/USERNAME/AppData/Local/display-switch/display-switch.log`

## Building from source

### Windows

[Install Rust](https://www.rust-lang.org/tools/install), then do `cargo build --release` from inside Windows directory.

### MacOS

[Install Xcode](https://developer.apple.com/xcode/), then do
`xcodebuild -scheme display_switch -configuration release -derivedDataPath .build build` from inside MacOS directory.

## Running on startup

### Windows

Copy `display_switch.exe` from `target\release` (where it was built in the previous step) to 
`C:\Users\Username\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup` (replace Username with your 
Windows user name).

### MacOS

```bash
  # Get your INI file in order! (see above)
  cp .build/Build/Products/Release/display_switch /usr/local/bin
  cp dev.haim.display-switch.daemon.plist ~/Library/LaunchAgents/
  launchctl load ~/Library/LaunchAgents/dev.haim.display-switch.daemon.plist
```
