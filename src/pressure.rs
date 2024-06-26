use std::cmp::min;
use std::error::Error;

use log::{debug, error /* info, warn */};

use rppal::i2c::I2c;

// Pressure sensor I2C address
const ADDR_PRESSURE_SENSOR: u16 = 0x4D;

pub struct Pressure {
    i2c: rppal::i2c::I2c,
    baseline: i32,
}

impl Pressure {
    pub fn init() -> Result<Pressure, Box<dyn Error>> {
        debug!("I2C: Configuring bus ...");

        let maybe_i2c = I2c::new();

        let mut i2c = match maybe_i2c {
            Ok(i2c) => i2c,
            Err(e) => {
                error!("Failed to initialize I2C.  Check raspi-config.");
                return Err(Box::new(e));
            }
        };

        debug!(
            "I2C: Created on bus {} at {} Hz",
            i2c.bus(),
            i2c.clock_speed()?
        );

        // Set the I2C slave address to the device we're communicating with.
        i2c.set_slave_address(ADDR_PRESSURE_SENSOR)?;

        debug!("I2C: slave address set to {}", ADDR_PRESSURE_SENSOR);

        let baseline = Pressure::read_io(&mut i2c)?;

        let sensor = Pressure {
            i2c: i2c,
            baseline: baseline,
        };

        debug!("I2C: baseline set to {}", sensor.baseline);

        Ok(sensor)
    }

    pub fn read(&mut self) -> Result<i32, Box<dyn Error>> {
        let pressure = Pressure::read_io(&mut self.i2c)?;
        // Compress the the range returned by the sensor to 0-127 required
        // for MIDI.  TODO:  Make this configurable
        const PRESSURE_SCALING_FACTOR: i32 = 6;
        Ok(min((pressure - self.baseline) / PRESSURE_SCALING_FACTOR, 127))
    }

    fn read_io(i2c: &mut rppal::i2c::I2c) -> Result<i32, Box<dyn Error>> {
        let mut reg = [0u8; 2];
        let mut result;
        i2c.read(&mut reg)?;
        result = reg[0] as i32;
        result <<= 8;
        result |= reg[1] as i32;
        result = result - 2048;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    // Import names from outer (for mod tests) scope.
    use super::*;

    use std::thread;
    use std::time::Duration;

    #[test]
    fn init() {
        let mut _sensor = Pressure::init().expect("Failed to initialize pressure sensor");
    }

    #[test]
    fn read() -> Result<(), Box<dyn Error>> {
        let mut sensor = Pressure::init().expect("Failed to initialize pressure sensor");
        let _pressure = sensor.read()?;
        Ok(())
    }

    /* This test is ignored by default because it expects pressure readings to change over time.
    In order to do that, you might need to blow some air into the tube.

    Run as
    cargo test pressure_step -- --ignored --nocapture
    */
    #[test]
    #[ignore]
    fn pressure_step() -> Result<(), Box<dyn Error>> {
        println!("Blow and draw air from the mouthpiece...");
        let mut sensor = Pressure::init().expect("Failed to initialize pressure sensor");
        let mut pressure_positive_detected = false;
        let mut pressure_negative_detected = false;
        for _ in 0..100 {
            let pressure = sensor.read()?;

            const EXPECTED_VARIATION: i32 = 10;

            if pressure > EXPECTED_VARIATION {
                pressure_positive_detected = true;
                println!("+ pressure: {}", pressure);
            }

            if pressure < -EXPECTED_VARIATION {
                pressure_negative_detected = true;
                println!("- pressure: {}", pressure);
            }

            if pressure_negative_detected && pressure_positive_detected {
                break;
            }
            thread::sleep(Duration::from_millis(50))
        }
        assert!(pressure_positive_detected);
        assert!(pressure_negative_detected);

        Ok(())
    }

    /* Test the range of raw pressure readings coming from the sensor.

    Run as
    cargo test pressure_range -- --ignored --nocapture
    */
    #[test]
    #[ignore]
    fn read_io() -> Result<(), Box<dyn Error>> {
        println!("Blow and draw on the mouthpiece...");
        let mut sensor = Pressure::init().expect("Failed to initialize pressure sensor");
        let mut max_val: i32 = 0;
        let mut min_val: i32 = i32::MAX;
        let mut pressure_range_detected = false;
        // Test that we can cover at least 1/4 of the full 12-bit output range of the sensor
        const EXPECTED_PRESSURE_RANGE: i32 = 4096 / 4;
        for _ in 0..100 {
            thread::sleep(Duration::from_millis(50));
            let pressure = Pressure::read_io(&mut sensor.i2c)?;
            println!("pressure: {}, min: {}, max: {}", pressure, min_val, max_val);
            if pressure > max_val {
                max_val = pressure;
            }
            if pressure < min_val {
                min_val = pressure;
            }
            if max_val - min_val > EXPECTED_PRESSURE_RANGE {
                pressure_range_detected = true;
                break;
            }
        }
        assert!(pressure_range_detected);
        Ok(())
    }
}
