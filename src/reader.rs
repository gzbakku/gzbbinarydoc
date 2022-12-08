
use crate::DocValue;
use byteorder::{BigEndian, ReadBytesExt};
use std::{io::Cursor, collections::HashMap};

#[derive(Debug)]
pub struct Reader<'a>{
    data:&'a Vec<u8>,
}

#[derive(Debug)]
pub struct SubReader{
    start:usize,
    end:usize,
    cursor:usize,
}

impl SubReader{
    pub fn sub(&mut self,global:&mut Reader,size:usize)->Result<SubReader,()>{
        let start = self.start + self.cursor;
        let end = start + size - 1;
        // println!("sub : {:?} {:?} {:?} {:?}",start,end,self.cursor+size,size);
        if end >= global.data.len(){
            // println!("sub : {:?} {:?} {:?}",start,end,global.data.len());
            return Err(());
        }
        self.cursor += size;
        return Ok(SubReader{
            start:start,
            end:end,
            cursor:0
        });
    }
    pub fn read(&mut self,global:&mut Reader,size:usize)->Result<Vec<u8>,()>{
        let start = self.start + self.cursor;
        let end = start + size - 1;
        
        if end >= global.data.len(){
            // println!("read : {:?} {:?} {:?}",start,end,global.data.len());
            return Err(());
        }
        let mut build = vec![];
        for i in start..=end{
            build.push(global.data[i]);
        }
        // println!("read : {:?} {:?} {:?} {:?} {:?} {:?}",start,end,self.cursor+size,size,self,build);
        self.cursor += size;
        return Ok(build);
    }
    pub fn read_full(&mut self,global:&mut Reader)->Result<Vec<u8>,()>{
        let start = self.start;
        let end = self.end;
        if end >= global.data.len(){
            // println!("read_full : {:?} {:?} {:?}",start,end,global.data.len());
            return Err(());
        }
        let mut build = vec![];
        for i in start..=end{
            build.push(global.data[i]);
        }
        // println!("read_full : {:?} {:?} {:?} {:?}",start,end,self,build);
        self.cursor = end;
        return Ok(build);
    }
}

impl <'a>Reader<'a>{
    pub fn new(data:&Vec<u8>)->Reader{
        Reader {
            data:data
        }
    }
    pub fn build(&mut self)->Result<DocValue,()>{
        let mut sub = SubReader{
            start:0,
            end:self.data.len()-1,
            cursor:0
        };
        read_data_line(self,&mut sub)
    }
}

pub fn read_data_line(global:&mut Reader,reader:&mut SubReader)->Result<DocValue,()>{

    // println!("\n\nread_data_line\n\n");

    // println!("reader : {:?}",reader);

    let data_type = reader.read(global,1)?;
    let data_len_bytes = reader.read(global,8)?;
    let data_len = bytes_to_u64(data_len_bytes)?;
    let mut data = reader.sub(global,data_len as usize)?;

    // println!("{:?} {:?} {:?} {:?} {:?}",data_type,data_len,data,reader.cursor,global.data.len());

    process_data(data_type[0],global, &mut data)

}

pub fn process_data(data_type:u8,global:&mut Reader,reader:&mut SubReader)->Result<DocValue,()>{

    if data_type == 0{
        return process_object(global, reader);
    } else if data_type == 1{
        return process_vec(global, reader);
    } else if data_type == 2{
        return process_binary(global, reader);
    } else if data_type == 3{
        return process_string(global, reader);
    } else if data_type == 4{
        return process_num(global, reader);
    } else if data_type == 5{
        return process_float(global, reader);
    } else if data_type == 6{
        return process_bool(global, reader);
    } else if data_type == 7{
        return process_null(global, reader);
    } else {
        return Err(());
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

pub fn process_object(global:&mut Reader,reader:&mut SubReader)->Result<DocValue,()>{

    // println!("\n\nprocess_object\n\n");

    let mut map:HashMap<String, DocValue> = HashMap::new();

    loop{

        let key_len_bytes = reader.read(global,8)?;
        let key_len = bytes_to_u64(key_len_bytes.clone())?;
        let key_bytes = reader.read(global,key_len as usize)?;
        let key = bytes_to_string(key_bytes.clone())?;

        // println!("key : {:?}",key);

        let data_len_bytes = reader.read(global,8)?;
        let data_len = bytes_to_u64(data_len_bytes.clone())?;
        let mut data_reader = reader.sub(global,data_len as usize)?;
        let data = read_data_line(global,&mut data_reader)?;

        // println!("data : {:?}",data);

        map.insert(key,data);

        let continue_byte = reader.read(global,1)?;
        if continue_byte.len() != 1{return Err(());}
        if continue_byte[0] == 1{} else if continue_byte[0] == 0{break;} else {return Err(());}

    }

    return Ok(DocValue::Object(map));

}
pub fn process_vec(global:&mut Reader,reader:&mut SubReader)->Result<DocValue,()>{

    let mut build = vec![];

    loop{

        let len_rep_bytes = reader.read(global,8)?;
        let len_rep = bytes_to_u64(len_rep_bytes)?;
        let mut data_reader = reader.sub(global,len_rep as usize)?;
        let data = read_data_line(global,&mut data_reader)?;
        build.push(data);
        let continue_byte = reader.read(global,1)?;
        if continue_byte.len() != 1{return Err(());}
        if continue_byte[0] == 1{} else if continue_byte[0] == 0{break;} else {return Err(());}

    }

    return Ok(DocValue::Vec(build));

    // Err(())
}
pub fn process_binary(global:&mut Reader,reader:&mut SubReader)->Result<DocValue,()>{
    let as_bytes = reader.read_full(global)?;
    return Ok(DocValue::Binary(as_bytes));
}
pub fn process_string(global:&mut Reader,reader:&mut SubReader)->Result<DocValue,()>{
    let as_bytes = reader.read_full(global)?;
    let as_value = bytes_to_string(as_bytes)?; 
    return Ok(DocValue::string(as_value));
}
pub fn process_num(global:&mut Reader,reader:&mut SubReader)->Result<DocValue,()>{
    let as_bytes = reader.read_full(global)?;
    let as_value = bytes_to_i64(as_bytes)?; 
    return Ok(DocValue::Num(as_value));
}
pub fn process_float(global:&mut Reader,reader:&mut SubReader)->Result<DocValue,()>{
    let as_bytes = reader.read_full(global)?;
    let as_value = bytes_to_f64(as_bytes)?; 
    return Ok(DocValue::Float(as_value));
}
pub fn process_bool(global:&mut Reader,reader:&mut SubReader)->Result<DocValue,()>{
    let as_bytes = reader.read_full(global)?;
    if as_bytes.len() != 1{return Err(());}
    let hold:bool;
    if as_bytes[0] == 0{hold = false;} else if as_bytes[0] == 1{hold = true;} else {return Err(());}
    return Ok(DocValue::Bool(hold));
}
pub fn process_null(global:&mut Reader,reader:&mut SubReader)->Result<DocValue,()>{
    let as_bytes = reader.read_full(global)?;
    if as_bytes.len() != 1{return Err(());}
    if as_bytes[0] != 0{return Err(());}
    return Ok(DocValue::Null);
}

pub fn bytes_to_i64(v:Vec<u8>)->Result<i64,()>{
    let mut rdr = Cursor::new(v);
    match rdr.read_i64::<BigEndian>(){
        Ok(v)=>{Ok(v)},
        Err(_)=>{Err(())}
    }
}
fn bytes_to_f64(v:Vec<u8>)->Result<f64,()>{
    let mut rdr = Cursor::new(v);
    match rdr.read_f64::<BigEndian>(){
        Ok(v)=>{Ok(v)},
        Err(_)=>{Err(())}
    }
}
fn bytes_to_u64(v:Vec<u8>)->Result<u64,()>{
    let mut rdr = Cursor::new(v);
    match rdr.read_u64::<BigEndian>(){
        Ok(v)=>{Ok(v)},
        Err(_)=>{Err(())}
    }
}
fn bytes_to_string(v:Vec<u8>)->Result<String,()>{
    match String::from_utf8(v){
        Ok(v)=>{Ok(v)},
        Err(_)=>{Err(())}
    }
}