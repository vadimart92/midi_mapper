pub mod main {
    use widestring::U16CString;

    type LPBYTE = *mut u8;
    type LPCWSTR = *const u16;
    type DWORD = u32;
    type PWORD = *mut u16;
    type MidiPortPtr = isize;

    #[link(name = "teVirtualMIDI64", kind = "static")]
    extern "C" {

        fn virtualMIDIGetVersion(
            major: PWORD,
            minor: PWORD,
            release: PWORD,
            build: PWORD,
        ) -> LPCWSTR;
        fn virtualMIDIGetDriverVersion(
            major: PWORD,
            minor: PWORD,
            release: PWORD,
            build: PWORD,
        ) -> LPCWSTR;
        fn virtualMIDICreatePortEx2(
            port_name: LPCWSTR,
            callback: i64,
            dw_callback_instance: i64,
            max_sysex_length: DWORD,
            flags: DWORD,
        ) -> MidiPortPtr;
        fn virtualMIDIClosePort(midi_port: MidiPortPtr);
        fn virtualMIDISendData(
            midi_port: MidiPortPtr,
            midi_data_bytes: LPBYTE,
            length: DWORD,
        ) -> bool;
    }

    pub fn get_client_version() -> String {
        let result = unsafe {
            let mut tmp: u16 = 0;
            let buffer = virtualMIDIGetVersion(&mut tmp, &mut tmp, &mut tmp, &mut tmp);
            let utf16string = U16CString::from_ptr_str(buffer);
            utf16string.to_string_lossy()
        };
        result
    }
    pub fn get_driver_version() -> String {
        let result = unsafe {
            let mut tmp: u16 = 0;
            let buffer = virtualMIDIGetDriverVersion(&mut tmp, &mut tmp, &mut tmp, &mut tmp);
            let utf16string = U16CString::from_ptr_str(buffer);
            utf16string.to_string_lossy()
        };
        result
    }
    pub fn open_port(name: &str) -> MidiDevice {
        let mut result = MidiDevice {
            name: name.to_string(),
            is_open: false,
            port_ptr: 0,
        };
        result.open();
        result
    }

    #[derive(Default)]
    pub struct MidiDevice {
        name: String,
        is_open: bool,
        port_ptr: isize,
    }

    impl MidiDevice {
        pub fn open(&mut self) {
            let name_str = U16CString::from_str(self.name.as_str()).unwrap();
            self.port_ptr = unsafe {
                let port =
                    virtualMIDICreatePortEx2(name_str.as_ptr(), 0, 0, 65535 as DWORD, 1 as DWORD);
                port
            };
            self.is_open = true;
        }
        pub fn close(&self) {
            unsafe {
                virtualMIDIClosePort(self.port_ptr);
            }
        }
        /** https://www.cs.cmu.edu/~music/cmsip/readings/MIDI%20tutorial%20for%20programmers.html */
        pub fn play_note(&self, a: u8, b: u8, c: u8) {
            let mut data: Vec<u8> = Vec::with_capacity(3);
            data.push(a);
            data.push(b);
            data.push(c);
            unsafe {
                virtualMIDISendData(
                    self.port_ptr.clone(),
                    data.as_mut_ptr() as LPBYTE,
                    3 as DWORD,
                );
            }
        }
    }
}

/*

Start device -> int
Stop device(number)
GetDevices
SendMidiCommand(dev, cmd)
BindController(dev, map)

*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
