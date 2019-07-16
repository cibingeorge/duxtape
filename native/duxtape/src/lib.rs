#[macro_use] extern crate rustler;
//#[macro_use] extern crate rustler_codegen;
//#[macro_use] extern crate lazy_static;

extern crate ducc;

use ducc::{Ducc};
use rustler::{Env, Term, NifResult, Encoder};
use rustler::resource::ResourceArc;
use std::sync::Mutex;

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

struct DuccResource {
    pub ducc:  Mutex<Ducc>
}

unsafe impl Send for DuccResource {}
unsafe impl Sync for DuccResource {}


rustler_export_nifs! {
    "Elixir.Duxtape.Native",
    [
    ("new_context", 0, new_context),
    ("compile", 2, compile)
    ],
    Some(on_load)
}

fn on_load(env: Env, _info: Term) -> bool {
    resource_struct_init!(DuccResource, env);
    true
}

fn new_context<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource = ResourceArc::new(DuccResource{
        ducc: Mutex::new(Ducc::new())
    });

    Ok(resource.encode(env))
}

fn compile<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<DuccResource> = args[0].decode()?;
    let source: String = args[1].decode()?;

    let ducc = resource.ducc.lock().unwrap();
    let result = ducc.compile(&source, None);
    match result {
        Ok(func) =>
            func.call();
            return Ok(atoms::ok().encode(env)),
        Err(error) =>
            return Ok(atoms::ok().encode(env))
    }

    // source: &str
    // let until_byte: u8 = args[1].decode()?;

    // let mut resource_struct = resource.stream.try_lock().unwrap();

    // let resource = ResourceArc::new(DuccResource{
    //     ducc: Arc::new(Mutex::new(Ducc::new()))
    // });


    Ok(atoms::ok().encode(env))
    // Ok(resource.encode(env))
}


    // let func: Value = ducc.compile("(function(x, y) { return x + y; })", None).unwrap()
    //     .call(()).unwrap();
