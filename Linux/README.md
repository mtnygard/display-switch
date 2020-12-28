# Linux setup instructions

## Allow yourself to use DDC to determine how to address your monitor

Add yourself to group `i2c`

```
sudo usermod -aG i2c $USER
newgrp i2c
```

Now list devices:

```
ddcutil detect
```

If all has gone well you will see something like this:

```
Display 1
   I2C bus:             /dev/i2c-7
   EDID synopsis:
      Mfg id:           DEL
      Model:            DELL S3220DGF
      Serial number:    BKCN4W2
      Manufacture year: 2019
      EDID version:     1.4
   VCP version:         2.1
```

## Install required packages

### Debian and derivatives

```
sudo apt install libudev-dev
```
