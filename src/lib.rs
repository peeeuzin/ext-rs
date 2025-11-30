use std::{
    io::{BufReader, Read},
    ops::Shr,
    str::FromStr,
};

pub struct ExtReader<R: Read>(pub BufReader<R>);

pub trait Extractable {
    fn extract<T>(&mut self, value: &mut T) -> &mut Self
    where
        T: FromStr;
}

impl<R: Read> Extractable for ExtReader<R> {
    fn extract<T>(&mut self, value: &mut T) -> &mut Self
    where
        T: FromStr,
    {
        let delimiters = [b' ', b'\n', b'\t'];

        let mut buf = Vec::new();

        read_until(&mut self.0, &delimiters, &mut buf);

        *value = String::from_utf8(buf)
            .unwrap()
            .trim()
            .parse::<T>()
            .ok()
            .unwrap();

        self
    }
}

impl<R: Read, T: FromStr> Shr<&mut T> for ExtReader<R> {
    type Output = Self;

    fn shr(mut self, rhs: &mut T) -> Self::Output {
        self.extract(rhs);
        self
    }
}

fn read_until<T>(reader: &mut BufReader<T>, delimiters: &[u8], buffer: &mut Vec<u8>) -> usize
where
    T: Read,
{
    let mut total_read = 0;

    loop {
        let mut byte = [0u8; 1];
        let bytes_read = reader.read(&mut byte).unwrap();

        if bytes_read == 0 {
            break;
        }

        if delimiters.contains(&byte[0]) {
            break;
        }

        buffer.push(byte[0]);
        total_read += bytes_read;
    }

    total_read
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;

    #[test]
    fn test_extractable() {
        let data = b"42 3.1 hello\nworld";
        let cursor = std::io::Cursor::new(data);
        let mut reader = ExtReader(BufReader::new(cursor));

        let mut int_value: i32 = 0;
        let mut float_value: f64 = 0.0;
        let mut string_value = String::new();
        reader
            .extract(&mut int_value)
            .extract(&mut float_value)
            .extract(&mut string_value);

        assert_eq!(int_value, 42);
        assert_eq!(float_value, 3.1);
        assert_eq!(string_value, "hello");
    }

    #[test]
    fn test_shr_operator() {
        let data = b"100 2.5 test\ncase";
        let cursor = std::io::Cursor::new(data);
        let reader = ExtReader(BufReader::new(cursor));

        let mut int_value: i32 = 0;
        let mut float_value: f64 = 0.0;
        let mut string_value = String::new();
        reader >> &mut int_value >> &mut float_value >> &mut string_value;
        assert_eq!(int_value, 100);
        assert_eq!(float_value, 2.5);
        assert_eq!(string_value, "test");
    }
}
