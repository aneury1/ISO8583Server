use std::io::Read;

fn payload() -> [u8;318] {
    let bytes: [u8; 318]   =
    [
        0x01, 0x3c, 0x60, 0x00, 0x02, 0x00, 0x00, 0x02, 0x00, 0x32, 0x38, 0x07, 0x80, 0x20, 0xc1, 0x82,
        0x1c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x33, 0x02, 0x28, 0x15, 0x50, 0x37, 0x00,
        0x00, 0x02, 0x15, 0x50, 0x33, 0x02, 0x28, 0x00, 0x70, 0x00, 0x01, 0x00, 0x02, 0x00, 0x37, 0x54,
        0x00, 0x83, 0x15, 0x12, 0x92, 0x55, 0x23, 0xd2, 0x40, 0x72, 0x01, 0x00, 0x00, 0x05, 0x65, 0x11,
        0x00, 0x00, 0x4d, 0x30, 0x30, 0x30, 0x31, 0x30, 0x30, 0x31, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
        0x31, 0x36, 0x37, 0x33, 0x39, 0x31, 0x30, 0x30, 0x31, 0x00, 0x03, 0x30, 0x30, 0x31, 0x32, 0x31,
        0x34, 0x01, 0x79, 0x9f, 0x02, 0x06, 0x00, 0x00, 0x00, 0x00, 0x03, 0x33, 0x9f, 0x03, 0x06, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x9f, 0x1a, 0x02, 0x02, 0x14, 0x95, 0x05, 0x04, 0x00, 0x00, 0x00,
        0x01, 0x5f, 0x2a, 0x02, 0x02, 0x14, 0x9a, 0x03, 0x23, 0x02, 0x28, 0x9c, 0x01, 0x00, 0x9f, 0x37,
        0x04, 0x0b, 0x20, 0x0e, 0x7d, 0x82, 0x02, 0x19, 0x80, 0x9f, 0x36, 0x02, 0x16, 0x7f, 0x9f, 0x10,
        0x12, 0x01, 0x10, 0xa0, 0x00, 0x01, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xff, 0x9f, 0x27, 0x01, 0x80, 0x9f, 0x34, 0x03, 0x1f, 0x03, 0x02, 0x5f, 0x34, 0x01,
        0x01, 0x57, 0x13, 0x54, 0x00, 0x83, 0x15, 0x12, 0x92, 0x55, 0x23, 0xd2, 0x40, 0x72, 0x01, 0x00,
        0x00, 0x05, 0x65, 0x11, 0x00, 0x0f, 0x9f, 0x26, 0x08, 0xb8, 0x6a, 0x97, 0x5b, 0x99, 0x26, 0x2c,
        0x3b, 0x84, 0x07, 0xa0, 0x00, 0x00, 0x00, 0x04, 0x10, 0x10, 0x9f, 0x09, 0x02, 0x00, 0x02, 0x9f,
        0x33, 0x03, 0xe0, 0x08, 0xc8, 0x9f, 0x35, 0x01, 0x22, 0x9f, 0x6e, 0x07, 0x02, 0x14, 0x00, 0x00,
        0x30, 0x30, 0x00, 0x9f, 0x41, 0x04, 0x00, 0x00, 0x00, 0x01, 0x9f, 0x1e, 0x09, 0x32, 0x36, 0x32,
        0x38, 0x31, 0x34, 0x31, 0x32, 0x36, 0x00, 0x19, 0x56, 0x4e, 0x46, 0x2d, 0x35, 0x2e, 0x31, 0x2e,
        0x33, 0x2d, 0x32, 0x38, 0x46, 0x45, 0x42, 0x32, 0x30, 0x32, 0x33, 0x00, 0x11, 0x32, 0x36, 0x32,
        0x2d, 0x38, 0x31, 0x34, 0x2d, 0x31, 0x32, 0x36, 0x00, 0x04, 0x30, 0x30, 0x30, 0x31
    ];
    bytes.clone()
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn extract_length(payload: &[u8])->i64{
    let value: i32 = (((payload[0] as i32) << 8) | (payload[1] as i32) ) as i32;
  let str = format!("{}", value);
  println!("{str}");
  let number: i64 = str.parse::<i32>().unwrap_or(0) as i64;
    number
}

fn extract_tpdu(payload: &[u8]) -> Result<[u8;5], String>{
   let mut ret : &[u8] = &payload[2..7];   ///u8::from_le_bytes(<[u8;5]::try_from(&payload[2..7]).) // payload[2..7].copy_from_slice();
    let mut a : [u8;5] =[
        ret[0],
        ret[1],
        ret[2],
        ret[3],
        ret[4],
    ];
    Ok(a)
    // return Ok(<[u8; 5]>::try_from(ret.clone()).unwrap());
  //  return Err("-1".to_string())
}
fn extract_mti(payload: &[u8]) -> Result<String, String>{
      ///u8::from_le_bytes(<[u8;5]::try_from(&payload[2..7]).) // payload[2..7].copy_from_slice();
    let mut a :String =  match payload[7]{
        0x02 => "02".to_string(),
        0x04 => "04".to_string(),
        0x05 => "05".to_string(),
        0x08 => "08".to_string(),
        _ => "0000".to_string()
    }; // format!("{:#02x}{:#02x}",payload[7], payload[8]).to_string();

     a = a + match payload[8]{
         0x10 => "10",
         0x20 => "20",
         0x30 => "30",
         0x40 => "40",
         0x50 => "50",
         0x60 => "60",
         0x70 => "70",
         0x80 => "80",
         0x90 => "90",
         _=>"00"
     };

        /*[
        payload[8],
        payload[9],
    ];*/
    Ok(a)
    // return Ok(<[u8; 5]>::try_from(ret.clone()).unwrap());
    //  return Err("-1".to_string())
}
fn convert_bitmap_to_binary_numbers(bitmap: [u8; 8])-> String {
    let mut iter= 0;
    let mut str = String::from("0");
    while iter<8{
        let single_byte = bitmap[iter];
// assert_eq!("00000110", format!("{:0>8}", "110"));
//                                |||
//                                ||+-- width
//                                |+--- align
//                                +---- fill
        let s = format!("{:0>8b}", single_byte).to_string();
        println!("Byte converted {:#X} to {}", single_byte, s.to_string());
        // str += reverse(s.chars().as_str()).as_str();
        str +=  s.chars().as_str();
        iter+=1;
    }
    return str;
}

fn print_number_active_in_bitmap(st: &String){
    let mut iter =0;
    while iter < 64{
        let ch  = st.chars().nth(iter).unwrap();
        if ch=='1' {
            println!("field {}: is active", iter)
        }else{
            println!("field {}: is not active", iter)
        }
        iter += 1;
    }
}

fn main() {
  //let lbg: u8 = 96;
  //let bitmap: [u8; 8] = [0x32,  0x38,  0x07, 0x80, 0x20, 0xC1,0x82, 0x1C];
  //let parse_binary = convert_bitmap_to_binary_numbers(bitmap);
    //print!("{}",convert_to_string_bit(lbg));
 // println!("Convert {}", parse_binary);
  ////print_number_active_in_bitmap(&parse_binary.as_str().to_string());

    //  000100110000011100111000000000000100000100100000110100000100111000
    //  000011001000111000000001111000000000100000110000011000001000011100

}
/*
000 MsgType              : "0200"                     *
001 BitMap               : "32 38 07 80 20 C1 82 1C"  *
003 ProcessingCode       : "000000"
004 TxnAmount            : "000000000333"
007 TransmitDateTime     : "0228155037"
011 SystemTraceNo        : "000002"
012 TxnTime              : "155033"
013 TxnDate              : "0228"
022 POSEntryMode         : "070"
023 CardSequenceNo       : "001"
024 NII                  : "002"
025 POSConditionCode     : "00"
035 Track2               : "5400000012925523=00072010000056511000"
041 TerminalID           : "M0001001"
042 AcquirerID           : "000000167391001"
048 PrivateAddData       : "001"
049 TxnCurrency          : "214"
055 Field55              : "9F 02 06 00 00 00 00 03 33 9F 03 06 00 00 00 00 00 00 9F 1A 02 02 14 95 05 04 00 00 00 01 5F 2A 02 02 14 9A 03 23 02 28 9C 01 00 9F 37 04 0B 20 0E 7D 82 02 19 80 9F 36 02 16 7F 9F 10 12 01 10 A0 00 01 22 00 00 00 00 00 00 00 00 00 00 00 FF 9F 27 01 80 9F 34 03 1F 03 02 5F 34 01 01 57 13 54 00 83 15 12 92 55 23 D2 40 72 01 00 00 05 65 11 00 0F 9F 26 08 B8 6A 97 5B 99 26 2C 3B 84 07 A0 00 00 00 04 10 10 9F 09 02 00 02 9F 33 03 E0 08 C8 9F 35 01 22 9F 6E 07 02 14 00 00 30 30 00 9F 41 04 00 00 00 01 9F 1E 09 32 36 32 38 31 34 31 32 36"
060 Field60              : "APP-0.0.0-000000000"
061 Field61              : "000-000-000"
062 Field62              : "0001"

*/

#[test]
fn test_length_of_buffer_payload(){
    let  payload = payload();
    let length = extract_length(&payload);
    assert_eq!(length, 316);
}

#[test]
fn test_extract_valid_tpdu(){
    let payload = payload();
    let payload =  extract_tpdu(&payload).unwrap();
    assert_eq!(payload, [0x60, 0x00, 0x02, 0x00, 0x00])
}
#[test]
fn test_extract_valid_mti(){
    let payload = payload();
    let payload =  extract_mti(&payload).unwrap();
    assert_eq!(payload, "0200")
}
#[test]
fn test_simple_bitmap_convertion(){
    //let lbg: u8 = 96;
    let bitmap: [u8; 8] = [0x32,  0x38,  0x07, 0x80, 0x20, 0xC1,0x82, 0x1C];
    let parse_binary = convert_bitmap_to_binary_numbers(bitmap);

    let mut ch : char =  parse_binary.chars().nth(3).unwrap();
    assert_eq!(ch, '1');
    ch      =  parse_binary.chars().nth(4).unwrap();
    assert_eq!(ch, '1');
    ch      =  parse_binary.chars().nth(5).unwrap();
    assert_eq!(ch, '0');
    ch      =  parse_binary.chars().nth(7).unwrap();
    assert_eq!(ch, '1');
    ch      =  parse_binary.chars().nth(11).unwrap();
    assert_eq!(ch, '1');
    ch      =  parse_binary.chars().nth(12).unwrap();
    assert_eq!(ch, '1');
    ch      =  parse_binary.chars().nth(13).unwrap();
    assert_eq!(ch, '1');
    ch      =  parse_binary.chars().nth(22).unwrap();
    assert_eq!(ch, '1');
    ch      =  parse_binary.chars().nth(23).unwrap();
    assert_eq!(ch, '1');
    ch      =  parse_binary.chars().nth(24).unwrap();
    assert_eq!(ch, '1');
    ch      =  parse_binary.chars().nth(25).unwrap();
    assert_eq!(ch, '1');
    ch      =  parse_binary.chars().nth(26).unwrap();
    assert_eq!(ch, '0');
    ch      =  parse_binary.chars().nth(63).unwrap();
    assert_eq!(ch, '0');
    ch      =  parse_binary.chars().nth(62).unwrap();
    assert_eq!(ch, '1');
   // println!("Convert {}", parse_binary);
}



