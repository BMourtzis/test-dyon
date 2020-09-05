#[macro_use]
extern crate dyon;

use std::sync::Arc;
use dyon::{error, load, Module, Call, Runtime, Dfn, Type, Variable};
use dyon::embed::PushVariable;

fn main() {
    // let vec = vec![
    //     Obj::new(4.0, 5.0, 7.0),
    //     Obj::default()
    // ];

    let new_time = 4.0;

    let mut module = Module::new();

    error(load("src/scripts/component.dyon", &mut module));

    let ref module = Arc::new(module);

    let call = Call::new("update").arg(4.0);

    let mut rt = Runtime::new();

    let obj = Obj::new(4.0, 5.0, 6.0);
    rt.push(obj);

    error(call.run(&mut rt, module));

    // for obj in vec {
        
    // }

    dyon_fn!(fn get_new_time() -> f64 {
        return new_time;
    });
}

fn update_obj() {

}

fn test() {
    // error(run("src/scripts/test.dyon"));

    let mut module = Module::new();

    module.add(Arc::new("update_x".into()), update_x, Dfn::nl(vec![Type::F64], Type::Void));

    module.add(Arc::new("get_time".into()), get_time, Dfn::nl(vec![], Type::F64));

    error(load("src/scripts/test.dyon", &mut module));

    let ref module = Arc::new(module);

    let call = Call::new("test_time");
    error(call.run(&mut Runtime::new(), module));

    // let call = Call::new("f").arg(4.0);
    // match call.run_ret::<f64>(&mut Runtime::new(), module) {
    //     Ok(answer) => { println!("{}", answer); }
    //     Err(err) => { error(Err(err)); }
    // }
}

static TIME: f64 = 10.0;

dyon_fn!{fn update_x(i: f64) {
    println!("{}", i);
}}

dyon_fn!{fn get_time() -> f64 {
    return TIME;
}}

#[derive(Debug)]
struct Obj {
    x: f64,
    y: f64,
    z: f64
}

impl Obj {
    pub fn new (x: f64, y: f64, z: f64) -> Self {
        Obj {
            x,
            y,
            z
        }
    }

    pub fn default() -> Self {
        Obj {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    // dyon_fn!{fn update_x(&mut self, x: f64) {
    //     self.x = x;
    // }}

    // dyon_fn!{fn get_x(&self) -> f64 {
    //     return self.x;
    // }}
}

impl PushVariable for Obj {
    fn push_var(&self) -> Variable {
        return Variable::Vec4([self.x as f32, self.y as f32, self.z as f32, 0.0])
    }
}