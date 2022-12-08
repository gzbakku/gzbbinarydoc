use std::{collections::HashMap};
use std::collections::hash_map::Iter as HashMapIter;
use std::collections::hash_map::IterMut as HashMapIterMut;

#[derive(Clone,Debug)]
pub enum DocValue{
    Object(HashMap<String,DocValue>),Vec(Vec<DocValue>),Num(i64),String(String),Float(f64),Binary(Vec<u8>),Null,Bool(bool)
}

#[allow(non_snake_case)]
pub trait ToDocValue{
    fn toDocValue(&self)->DocValue;
}

impl ToDocValue for DocValue{
    fn toDocValue(&self)->DocValue {
        return self.clone()
    }
}

impl ToDocValue for bool{
    fn toDocValue(&self)->DocValue {
        DocValue::Bool(self.clone())
    }
}

impl ToDocValue for String{
    fn toDocValue(&self)->DocValue {
        DocValue::String(self.to_string())
    }
}

impl ToDocValue for &str{
    fn toDocValue(&self)->DocValue {
        DocValue::String(self.to_string())
    }
}

impl ToDocValue for i64{
    fn toDocValue(&self)->DocValue {
        DocValue::Num(self.clone())
    }
}

impl ToDocValue for f64{
    fn toDocValue(&self)->DocValue {
        DocValue::Float(self.clone())
    }
}

impl ToDocValue for Vec<DocValue>{
    fn toDocValue(&self)->DocValue {
        DocValue::Vec(self.clone())
    }
}

impl ToDocValue for Vec<u8>{
    fn toDocValue(&self)->DocValue {
        DocValue::Binary(self.clone())
    }
}

impl ToDocValue for (){
    fn toDocValue(&self)->DocValue {
        DocValue::Null
    }
}

impl ToDocValue for HashMap<String,DocValue>{
    fn toDocValue(&self)->DocValue {
        DocValue::Object(self.clone())
    }
}

impl DocValue{
    pub fn str(v:&str)->DocValue{
        return DocValue::String(v.to_string());
    }
    pub fn string(v:String)->DocValue{
        return DocValue::String(v);
    }
    pub fn num(i:i64)->DocValue{
        return DocValue::Num(i);
    }
    pub fn float(i:f64)->DocValue{
        return DocValue::Float(i);
    }
    pub fn object()->DocValue{
        return DocValue::Object(HashMap::new());
    }
    pub fn vec()->DocValue{
        return DocValue::Vec(vec![]);
    }
    pub fn binary()->DocValue{
        return DocValue::Binary(vec![]);
    }
    pub fn bool(v:bool)->DocValue{
        return DocValue::Bool(v);
    }
    //array
    pub fn push<T>(&mut self,v:T)
    where
        T:ToDocValue
    {
        match self{
            DocValue::Vec(p)=>{
                p.push(v.toDocValue());
            },
            _=>{}
        }
    }
    pub fn vec_remove(&mut self,index:usize){
        match self{
            DocValue::Vec(p)=>{
                p.remove(index);
            },
            _=>{}
        }
    }
    //object
    pub fn insert<T>(&mut self,k:&str,v:T) -> Option<DocValue>
    where
        T: ToDocValue
    
    {
        match self{
            DocValue::Object(d)=>{
                return d.insert(k.to_string(),v.toDocValue());
            },
            _=>{None}
        }
    }
    pub fn contains_key(&self,key:&str)->Option<bool>{
        match self{
            DocValue::Object(v)=>{
                return Some(v.contains_key(key));
            },
            _=>{
                return None;
            }
        }
    }
    pub fn object_remove(&mut self,key:&str)->Option<DocValue>{
        match self{
            DocValue::Object(v)=>{
                return v.remove(key);
            },
            _=>{
                return None;
            }
        }
    }
    pub fn object_iter(&mut self)->Option<HashMapIter<String,DocValue>>{
        match self{
            DocValue::Object(v)=>{
                return Some(v.iter());
            },
            _=>{
                return None;
            }
        }
    }
    pub fn object_iter_mut(&mut self)->Option<HashMapIterMut<String,DocValue>>{
        match self{
            DocValue::Object(v)=>{
                return Some(v.iter_mut());
            },
            _=>{
                return None;
            }
        }
    }
    pub fn update_path(&mut self,path:Vec<&str>,key:&str,value:DocValue)->Result<(),()>{
        match self{
            DocValue::Object(v)=>{return update_path(
                path,v,key,value
            );},
            _=>{return Err(());}
        }
    }
    //updates
    pub fn update_string(&mut self,v:String){
        match self{
            DocValue::String(d)=>{
                *d = v
            },
            _=>{}
        }
    }
    pub fn update_num(&mut self,v:i64){
        match self{
            DocValue::Num(d)=>{
                *d = v
            },
            _=>{}
        }
    }
    pub fn update_float(&mut self,v:f64){
        match self{
            DocValue::Float(d)=>{
                *d = v
            },
            _=>{}
        }
    }
    pub fn update_object(&mut self,k:&str,v:DocValue){
        match self{
            DocValue::Object(d)=>{
                d.insert(k.to_string(), v);
            },
            _=>{}
        }
    }
    pub fn update_vec(&mut self,v:Vec<DocValue>){
        match self{
            DocValue::Vec(d)=>{
                *d = v;
            },
            _=>{}
        }
    }
    pub fn update_binary(&mut self,v:Vec<u8>){
        match self{
            DocValue::Binary(d)=>{
                *d = v;
            },
            _=>{}
        }
    }
    pub fn update_bool(&mut self,v:bool){
        match self{
            DocValue::Bool(d)=>{
                *d = v;
            },
            _=>{}
        }
    }
    //extractors
    pub fn as_object(&self)->Option<HashMap<String,DocValue>>{
        match self{
            DocValue::Object(v)=>{
                return Some(v.clone());
            },
            _=>{
                return None;
            }
        }
    }
    pub fn as_vec(&self)->Option<Vec<DocValue>>{
        match self{
            DocValue::Vec(v)=>{
                return Some(v.clone());
            },
            _=>{
                return None;
            }
        }
    }
    pub fn as_binary(&self)->Option<Vec<u8>>{
        match self{
            DocValue::Binary(v)=>{
                return Some(v.clone());
            },
            _=>{
                return None;
            }
        }
    }
    pub fn as_string(&self)->Option<String>{
        match self{
            DocValue::String(v)=>{
                return Some(v.clone());
            },
            _=>{
                return None;
            }
        }
    }
    pub fn as_num(&self)->Option<i64>{
        match self{
            DocValue::Num(v)=>{
                return Some(v.clone());
            },
            _=>{
                return None;
            }
        }
    }
    pub fn as_float(&self)->Option<f64>{
        match self{
            DocValue::Float(v)=>{
                return Some(v.clone());
            },
            _=>{
                return None;
            }
        }
    }
    pub fn as_bool(&self)->Option<bool>{
        match self{
            DocValue::Bool(v)=>{
                return Some(v.clone());
            },
            _=>{
                return None;
            }
        }
    }
    //checkers
    pub fn self_is_object(&self)->bool{
        match self{
            DocValue::Object(_v)=>{return true;},
            _=>{return false;}
        }
    }
    pub fn self_is_vec(&self)->bool{
        match self{
            DocValue::Vec(_v)=>{return true;},
            _=>{return false;}
        }
    }
    pub fn self_is_binary(&self)->bool{
        match self{
            DocValue::Binary(_v)=>{return true;},
            _=>{return false;}
        }
    }
    pub fn self_is_string(&self)->bool{
        match self{
            DocValue::String(_v)=>{return true;},
            _=>{return false;}
        }
    }
    pub fn self_is_num(&self)->bool{
        match self{
            DocValue::Num(_v)=>{return true;},
            _=>{return false;}
        }
    }
    pub fn self_is_float(&self)->bool{
        match self{
            DocValue::Float(_v)=>{return true;},
            _=>{return false;}
        }
    }
    pub fn self_is_bool(&self)->bool{
        match self{
            DocValue::Bool(_v)=>{return true;},
            _=>{return false;}
        }
    }
    pub fn self_is_null(&self)->bool{
        match self{
            DocValue::Null=>{return true;},
            _=>{return false;}
        }
    }
    //objecy key checks
    pub fn key_is_object(&self,key:&str)->bool{
        match self{
            DocValue::Object(v)=>{
                match v.get(key){
                    Some(d)=>{d.self_is_object()},
                    None=>{false}
                }
            },
            _=>{false}
        }
    }
    pub fn key_is_vec(&self,key:&str)->bool{
        match self{
            DocValue::Object(v)=>{
                match v.get(key){
                    Some(d)=>{d.self_is_vec()},
                    None=>{false}
                }
            },
            _=>{false}
        }
    }
    pub fn key_is_binary(&self,key:&str)->bool{
        match self{
            DocValue::Object(v)=>{
                match v.get(key){
                    Some(d)=>{d.self_is_binary()},
                    None=>{false}
                }
            },
            _=>{false}
        }
    }
    pub fn key_is_string(&self,key:&str)->bool{
        match self{
            DocValue::Object(v)=>{
                match v.get(key){
                    Some(d)=>{d.self_is_string()},
                    None=>{false}
                }
            },
            _=>{false}
        }
    }
    pub fn key_is_num(&self,key:&str)->bool{
        match self{
            DocValue::Object(v)=>{
                match v.get(key){
                    Some(d)=>{d.self_is_num()},
                    None=>{false}
                }
            },
            _=>{false}
        }
    }
    pub fn key_is_float(&self,key:&str)->bool{
        match self{
            DocValue::Object(v)=>{
                match v.get(key){
                    Some(d)=>{d.self_is_float()},
                    None=>{false}
                }
            },
            _=>{false}
        }
    }
    pub fn key_is_bool(&self,key:&str)->bool{
        match self{
            DocValue::Object(v)=>{
                match v.get(key){
                    Some(d)=>{d.self_is_bool()},
                    None=>{false}
                }
            },
            _=>{false}
        }
    }
    pub fn key_is_null(&self,key:&str)->bool{
        match self{
            DocValue::Object(v)=>{
                match v.get(key){
                    Some(d)=>{d.self_is_null()},
                    None=>{false}
                }
            },
            _=>{false}
        }
    }
    //doc functions
    pub fn write(&self)->Vec<u8>{
        crate::writer::write(self)
    }
    pub fn read(data:&Vec<u8>)->Result<DocValue,()>{
        let mut reader = crate::reader::Reader::new(data);
        reader.build()
    }
}

fn update_path(
    path:Vec<&str>,
    pool:&mut HashMap<String,DocValue>,
    key:&str,
    value:DocValue
)->Result<(),()>{

    if path.len() == 0{
        return Err(());
    }

    // let mut base = HashMap::new();
    let mut hold:&mut HashMap<String,DocValue> = pool;
    for item in path{
        match pool.get_mut(item){
            Some(v)=>{
                match v{
                    DocValue::Object(s)=>{
                        hold = s;
                    },
                    _=>{
                        return Err(());
                    }
                }
            },
            None=>{
                return Err(());
            }
        }
    }

    hold.insert(key.to_string(), value);

    return Ok(());

}