
use crate::{DocValue};
use byteorder::{BigEndian, WriteBytesExt};

pub fn write(doc:&DocValue)->Vec<u8>{
    process_value(doc)
}

fn process_value(value:&DocValue)->Vec<u8>{
    if value.self_is_object(){
        return process_object(value);
    } else if value.self_is_vec(){
        return process_vec(value);
    } else if value.self_is_binary(){
        return process_binary(value);
    } else if value.self_is_num(){
        return process_num(value);
    } else if value.self_is_float(){
        return process_float(value);
    } else if value.self_is_string(){
        return process_string(value);
    } else if value.self_is_bool(){
        return process_bool(value);
    } else {
        return process_null(value);
    }
}

/*

data_len_rep - u64 num representing length of data in bytes represented in u64 bytes big endien

continue_byte - if 1 next is key if 0 this is last key

data_type - 1 byte each number is unique data type
    0 - object
    1 - vec
    2 - binary
    3 - string
    4 - num
    5 - float
    6 - bool
    7 - null

data_line - data_type data_len_rep data(nested as per data parse)

data_parse 
    object - **repeating pattern** data_len_rep key data_len_rep data continue_byte(if 1 next is key if 0 this is last key)
    vec - **repeating pattern** data_len_rep data continue_byte
    binary - data(vec<u8>)
    string - data(utf8 string as bytes)
    num - data(i64 num as bytes big endien)
    float - data(f64 num as bytes big endien)
    bool - data(0 for false 1 for true) - 1 byte
    null - data(0 as byte)  - 1 byte
*/

pub fn process_object(object:&DocValue)->Vec<u8>{
    let mut build = vec![];
    let map = object.as_object().unwrap();
    let map_len = map.len();
    let mut index = 0;
    for (key,value) in map.iter(){
        let continue_byte:u8;
        if index == map_len-1{continue_byte = 0;} else {continue_byte = 1;}
        let mut key_data_len = data_len_rep(key.len() as u64);
        let mut processed_data = process_value(value);
        let mut processed_data_len = data_len_rep(processed_data.len() as u64);
        build.append(&mut key_data_len);
        build.append(&mut key.as_bytes().to_vec());
        build.append(&mut processed_data_len);
        build.append(&mut processed_data);
        build.push(continue_byte);
        index += 1;
    }
    data_line(0, build)
}
pub fn process_vec(object:&DocValue)->Vec<u8>{
    let mut build = vec![];
    let mut index = 0;
    let pool = object.as_vec().unwrap();
    let pool_len = pool.len();
    for item in pool{
        let continue_byte:u8;
        if index == pool_len-1{continue_byte = 0;} else {continue_byte = 1;}
        let mut processed_value = process_value(&item);
        build.append(&mut data_len_rep(processed_value.len() as u64));
        build.append(&mut processed_value);
        build.push(continue_byte);
        index += 1;
    }
    data_line(1, build)
}
pub fn process_binary(object:&DocValue)->Vec<u8>{
    data_line(2, object.as_binary().unwrap())
}
pub fn process_string(object:&DocValue)->Vec<u8>{
    data_line(3, object.as_string().unwrap().as_bytes().to_vec())
}
pub fn process_num(object:&DocValue)->Vec<u8>{
    data_line(4, num_to_data(object.as_num().unwrap()))
}
pub fn process_float(object:&DocValue)->Vec<u8>{
    data_line(5, float_to_data(object.as_float().unwrap()))
}
pub fn process_bool(object:&DocValue)->Vec<u8>{
    let data:Vec<u8>;
    if object.as_bool().unwrap() {data = vec![1];} else {data = vec![0];};
    data_line(6, data)
}
pub fn process_null(object:&DocValue)->Vec<u8>{data_line(7, vec![0])}

//data builders
fn data_line(data_type:u8,mut data:Vec<u8>)->Vec<u8>{
    let mut data_len = data_len_rep(data.len() as u64);
    let mut build = vec![data_type];
    build.append(&mut data_len);
    build.append(&mut data);
    build
}
pub fn data_len_rep(v:u64)->Vec<u8>{
    let mut wtr = vec![];
    wtr.write_u64::<BigEndian>(v).unwrap();
    wtr
}
fn num_to_data(v:i64)->Vec<u8>{
    let mut wtr = vec![];
    wtr.write_i64::<BigEndian>(v).unwrap();
    wtr
}
fn float_to_data(v:f64)->Vec<u8>{
    let mut wtr = vec![];
    wtr.write_f64::<BigEndian>(v).unwrap();
    wtr
}