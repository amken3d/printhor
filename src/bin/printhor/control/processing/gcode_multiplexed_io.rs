///! Input Multiplexer
///!
///! This feature is a bit inefficient and could be improved leveraging macros
///! because currently useless polls are performed in disabled channels
use crate::hwa;
use crate::control::{GCodeCmd, GCodeLineParserError};
use embassy_futures::select::Either3;
#[allow(unused)]
use futures_util::future;


// Utility to accept a common gcode stream from multiple sources
pub struct GCodeMultiplexedInputStream {
    #[cfg(feature = "with-serial-usb")]
    serial_usb_line_parser:
        crate::control::GCodeLineParser<hwa::device::USBSerialDeviceInputStream>,
    #[cfg(feature = "with-serial-port-1")]
    serial_port1_line_parser:
        crate::control::GCodeLineParser<hwa::device::UartPort1RxInputStream>,
    #[cfg(feature = "with-serial-port-2")]
    serial_port2_line_parser:
        crate::control::GCodeLineParser<hwa::device::UartPort2RxInputStream>,
}

impl GCodeMultiplexedInputStream {
    pub fn new(
        #[cfg(feature = "with-serial-usb")]
        serial_usb_rx_stream: hwa::device::USBSerialDeviceInputStream,
        #[cfg(feature = "with-serial-port-1")]
        serial_port1_rx_stream: hwa::device::UartPort1RxInputStream,
        #[cfg(feature = "with-serial-port-2")]
        serial_port2_rx_stream: hwa::device::UartPort2RxInputStream,
    ) -> Self {
        Self {
            #[cfg(feature = "with-serial-usb")]
            serial_usb_line_parser: crate::control::GCodeLineParser::new(serial_usb_rx_stream),
            #[cfg(feature = "with-serial-port-1")]
            serial_port1_line_parser: crate::control::GCodeLineParser::new(serial_port1_rx_stream),
            #[cfg(feature = "with-serial-port-2")]
            serial_port2_line_parser: crate::control::GCodeLineParser::new(serial_port2_rx_stream),
        }
    }

    pub async fn next_gcode(
        &mut self,
    ) -> (
        Result<GCodeCmd, GCodeLineParserError>,
        hwa::CommChannel,
    ) {
        cfg_if::cfg_if! {
            if #[cfg(feature="with-serial-usb")] {
                let f1 = self.serial_usb_line_parser.next_gcode();
            }
            else {
                let f1 = future::pending::<Result<Option<GCodeCmd>, GCodeLineParserError>>();
            }
        }
        cfg_if::cfg_if! {
            if #[cfg(feature="with-serial-port-1")] {
                let f2 = self.serial_port1_line_parser.next_gcode();
            }
            else {
                let f2 = future::pending::<Result<Option<GCodeCmd>, GCodeLineParserError>>();
            }
        }
        cfg_if::cfg_if! {
            if #[cfg(feature="with-serial-port-2")] {
                let f3 = self.serial_port2_line_parser.next_gcode();
            }
            else {
                let f3 = future::pending::<Result<Option<GCodeCmd>, GCodeLineParserError>>();
            }
        }

        match embassy_futures::select::select3(f1, f2, f3).await {
            Either3::First(_result) => {
                cfg_if::cfg_if! {
                    if #[cfg(feature="with-serial-usb")] {
                        (_result, hwa::CommChannel::SerialUsb)
                    }
                    else {
                        hwa::error!("Unexpectedly got nothing");
                        (Err(GCodeLineParserError::EOF), hwa::CommChannel::Internal)
                    }
                }
            }
            Either3::Second(_result) => {
                cfg_if::cfg_if! {
                    if #[cfg(feature="with-serial-port-1")] {
                        (_result, hwa::CommChannel::SerialPort1)
                    }
                    else {
                        hwa::error!("Unexpectedly got nothing");
                        (Err(GCodeLineParserError::EOF), hwa::CommChannel::Internal)
                    }
                }
            }
            Either3::Third(_result) => {
                cfg_if::cfg_if! {
                    if #[cfg(feature="with-serial-port-2")] {
                        (_result, hwa::CommChannel::SerialPort2)
                    }
                    else {
                        hwa::error!("Unexpectedly got nothing");
                        (Err(GCodeLineParserError::EOF), hwa::CommChannel::Internal)
                    }
                }
            }
        }
    }

    #[allow(unused)]
    pub fn get_state(&self, channel: hwa::CommChannel) -> async_gcode::AsyncParserState {

        match channel {
            #[cfg(feature="with-serial-usb")]
            hwa::CommChannel::SerialUsb => self.serial_usb_line_parser.get_state(),
            #[cfg(feature="with-serial-port-1")]
            hwa::CommChannel::SerialPort1 => self.serial_port1_line_parser.get_state(),
            #[cfg(feature="with-serial-port-2")]
            hwa::CommChannel::SerialPort2 => self.serial_port2_line_parser.get_state(),
            hwa::CommChannel::Internal => {
                async_gcode::AsyncParserState::ErrorRecovery
            }
        }
    }

    #[allow(unused)]
    pub fn get_line(&self, channel: hwa::CommChannel) -> u32 {

        match channel {
            #[cfg(feature="with-serial-usb")]
            hwa::CommChannel::SerialUsb => self.serial_usb_line_parser.get_line(),
            #[cfg(feature="with-serial-port-1")]
            hwa::CommChannel::SerialPort1 => self.serial_port1_line_parser.get_line(),
            #[cfg(feature="with-serial-port-2")]
            hwa::CommChannel::SerialPort2 => self.serial_port2_line_parser.get_line(),
            hwa:: CommChannel::Internal => {
                0
            }
        }
    }

    pub fn get_gcode_line(&self, channel: hwa::CommChannel) -> Option<u32> {

        match channel {
            #[cfg(feature="with-serial-usb")]
            hwa::CommChannel::SerialUsb => self.serial_usb_line_parser.gcode_line(),
            #[cfg(feature="with-serial-port-1")]
            hwa::CommChannel::SerialPort1 => self.serial_port1_line_parser.gcode_line(),
            #[cfg(feature="with-serial-port-2")]
            hwa::CommChannel::SerialPort2 => self.serial_port2_line_parser.gcode_line(),
            hwa::CommChannel::Internal => {
                None
            }
        }
    }

    pub fn reset(&mut self, comm_channel: hwa::CommChannel) {
        match comm_channel {
            #[cfg(feature="with-serial-usb")]
            hwa::CommChannel::SerialUsb => self.serial_usb_line_parser.reset(),
            #[cfg(feature="with-serial-port-1")]
            hwa::CommChannel::SerialPort1 => self.serial_port1_line_parser.reset(),
            #[cfg(feature="with-serial-port-2")]
            hwa::CommChannel::SerialPort2 => self.serial_port2_line_parser.reset(),
            hwa::CommChannel::Internal => {}
        }
    }
}
