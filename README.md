# bmp5xx

True-to-spec async I2C driver for the BMP580/BMP581/BMP585 barometric pressure sensors.

- `no_std` compatible, works without an allocator
- Widely compatible, generic over [`embedded_hal_async`] I2C traits
- 100% documentation coverage
- Based on Bosch Sensortec BMP581 datasheet, written by a human
- Fully tested on real hardware: compatible with [Adafruit BMP580](https://www.adafruit.com/product/6411) and [Adafruit BMP581](https://www.adafruit.com/product/6407) development boards
- Working examples in the repository

### Features
- Normal, forced, and continuous mode measurements
- Configurable interrupts
- Out-of-range configuration
- Oversampling, IIR filtering

### Usage

Getting started is easy:
```rs
// initialize the sensor
let mut bmp5 = Bmp5xx::new(i2c, Delay, 0x47);
bmp5.init().await.unwrap();

// new pressure measurement
let pressure = bmp5.meas_pres().await.unwrap();
```

Advanced operations:
```rs
// change oversampling rate
bmp5.osr_temp(Oversampling::X8);
bmp5.osr_pres(Oversampling::X128);

// set up interrupts
bmp5.int(Interrupt::default().enable(true)).await.unwrap();

// start continuous measurement
bmp5.start_continuous(true).await.unwrap();
```

### Demo
If you don't have hardware to try it, check out the demo video: https://youtu.be/RD1FmZXssCE 

License: GPL-3.0-or-later
