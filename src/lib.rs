#![no_std]
#![deny(clippy::all)]
#![deny(missing_docs)]
//! Implements the DIV203
use embedded_hal::{delay::DelayNs, spi::SpiDevice};

/// Driver for a DIP203
#[derive(Debug)]
pub struct DIP203<SPI, Delay> {
    spi: SPI,
    delay: Delay,
}

impl<SPI, Delay> DIP203<SPI, Delay>
where
    SPI: SpiDevice,
    Delay: DelayNs,
{
    /// Initializes the lcd
    pub fn new(spi: SPI, delay: Delay) -> Result<Self, ErrorDIP203<SPI::Error>> {
        let mut lcd = Self { spi, delay };

        lcd.init()?;
        Ok(lcd)
    }

    /// Executes the correct init sequence
    fn init(&mut self) -> Result<(), ErrorDIP203<SPI::Error>> {
        self.write_command(0x34)?; // Function set (RE=1)
        self.delay.delay_us(60);

        self.write_command(0x0B)?; // Extended Function set (4-line display, inverting cursor)
        self.delay.delay_us(60);

        self.write_command(0x30)?; // Function set (RE=0)
        self.delay.delay_us(60);

        self.write_command(0x01)?; // Clear Display
        self.delay.delay_ms(2);

        self.write_command(0x0C)?; // Display On, Cursor Off, Blink Off
        self.delay.delay_us(60);

        Ok(())
    }

    /// Trys to print something to the screen
    pub fn print(
        &mut self,
        line: Line,
        position: Position,
        message: &str,
    ) -> Result<(), ErrorDIP203<SPI::Error>> {
        const CHAR_LIMIT: usize = 20;
        if message.is_empty()
            || message.len() > CHAR_LIMIT.saturating_sub(usize::from(u8::from(position.clone())))
        {
            return Err(ErrorDIP203::StringMisformed);
        }

        // 0x80 is the DDRAM base address, combined with the line offset
        self.write_command(0x80 | (u8::from(line) + u8::from(position)))?;
        self.delay.delay_us(60);

        for b in message.bytes() {
            self.write_data(b)?;
            self.delay.delay_us(60);
        }

        Ok(())
    }
    /// Clears the screen
    pub fn clear(&mut self) {
        let _ = self.write_command(0x01);
        self.delay.delay_ms(2);
    }

    fn write_command(&mut self, byte: u8) -> Result<(), SPI::Error> {
        let data = &[0x1F, (byte & 0x0F), (byte >> 4 & 0x0F)];
        self.spi.write(data)?;
        Ok(())
    }
    fn write_data(&mut self, byte: u8) -> Result<(), SPI::Error> {
        let data = &[(0x1F | 0x40), (byte & 0x0F), (byte >> 4 & 0x0F)];
        self.spi.write(data)?;
        Ok(())
    }
}

/// Which Row to use
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Line {
    /// First Row
    First = 0x00,
    /// Second Row
    Second = 0x20,
    /// Third Row
    Third = 0x40,
    /// Four Row
    Four = 0x60,
}

impl From<Line> for u8 {
    fn from(value: Line) -> Self {
        match value {
            Line::First => 0x00,
            Line::Second => 0x20,
            Line::Third => 0x40,
            Line::Four => 0x60,
        }
    }
}
/// The position in the row
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Position {
    /// The 1st position
    First,
    /// The 2nd position
    Second,
    /// The 3rd position
    Third,
    /// The 4th position
    Fourth,
    /// The 5th position
    Fifth,
    /// The 6th position
    Sixth,
    /// The 7th position
    Seventh,
    /// The 8th position
    Eighth,
    /// The 9th position
    Ninth,
    /// The 10th position
    Tenth,
    /// The 11th position
    Eleventh,
    /// The 12th position
    Twelfth,
    /// The 13th position
    Thirteenth,
    /// The 14th position
    Fourteenth,
    /// The 15th position
    Fifteenth,
    /// The 16th position
    Sixteenth,
    /// The 17th position
    Seventeenth,
    /// The 18th position
    Eighteenth,
    /// The 19th position
    Nineteenth,
}
impl From<Position> for u8 {
    fn from(value: Position) -> Self {
        match value {
            Position::First => 0,
            Position::Second => 1,
            Position::Third => 2,
            Position::Fourth => 3,
            Position::Fifth => 4,
            Position::Sixth => 5,
            Position::Seventh => 6,
            Position::Eighth => 7,
            Position::Ninth => 8,
            Position::Tenth => 9,
            Position::Eleventh => 10,
            Position::Twelfth => 11,
            Position::Thirteenth => 12,
            Position::Fourteenth => 13,
            Position::Fifteenth => 14,
            Position::Sixteenth => 15,
            Position::Seventeenth => 16,
            Position::Eighteenth => 17,
            Position::Nineteenth => 18,
        }
    }
}
/// Errors for this driver
#[derive(Debug)]
pub enum ErrorDIP203<E> {
    /// String for displaying is too long or to small
    StringMisformed,
    /// Spi Error
    Spi(E),
}
impl<E> From<E> for ErrorDIP203<E> {
    fn from(value: E) -> Self {
        ErrorDIP203::Spi(value)
    }
}
