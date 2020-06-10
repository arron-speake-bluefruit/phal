# Odroid XU4 GPIO

The Odroid shifter shield labels the pins with the gpio chip label and
the line number. The table below relates the chip labels to the device
paths.

| Chip label | Device path     |
|------------|-----------------|
| gpz        | /dev/gpiochip35 |
| gph0       | /dev/gpiochip34 |
| gpb4       | /dev/gpiochip33 |
| gpb3       | /dev/gpiochip32 |
| gpb2       | /dev/gpiochip31 |
| gpb1       | /dev/gpiochip30 |
| gpb0       | /dev/gpiochip29 |
| gpa2       | /dev/gpiochip28 |
| gpa1       | /dev/gpiochip27 |
| gpa0       | /dev/gpiochip26 |
| gpj4       | /dev/gpiochip25 |
| gpg2       | /dev/gpiochip24 |
| gpg1       | /dev/gpiochip23 |
| gpg0       | /dev/gpiochip22 |
| gpf1       | /dev/gpiochip21 |
| gpf0       | /dev/gpiochip20 |
| gpe1       | /dev/gpiochip19 |
| gpe0       | /dev/gpiochip18 |
| gpy6       | /dev/gpiochip17 |
| gpy5       | /dev/gpiochip16 |
| gpy4       | /dev/gpiochip15 |
| gpy3       | /dev/gpiochip14 |
| gpy2       | /dev/gpiochip13 |
| gpy1       | /dev/gpiochip12 |
| gpy0       | /dev/gpiochip11 |
| gpd1       | /dev/gpiochip10 |
| gpc4       | /dev/gpiochip9  |
| gpc3       | /dev/gpiochip8  |
| gpc2       | /dev/gpiochip7  |
| gpc1       | /dev/gpiochip6  |
| gpc0       | /dev/gpiochip5  |
| gpx3       | /dev/gpiochip4  |
| gpx2       | /dev/gpiochip3  |
| gpx1       | /dev/gpiochip2  |
| gpx0       | /dev/gpiochip1  |
| gpy7       | /dev/gpiochip0  |

So to use the pin labeled "GPX2.0" on the shifter shield as
a push-pull output named "foo", in your config you'd put:

```js
"foo": {
    "type": "output-pin",
    "chip": "/dev/gpiochip3",
    "line": 0,
    "pin-type": "push-pull"
}
```
