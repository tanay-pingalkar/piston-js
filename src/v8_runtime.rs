use std::os::raw::c_void;

pub struct Runtime<'s, 'i> {
    pub context_scope: v8::ContextScope<'i, v8::HandleScope<'s>>,
    pub context: v8::Local<'s, v8::Context>,
}

impl<'s, 'i> Runtime<'s, 'i> {
    pub fn new<T>(
        isolate_scope: &'i mut v8::HandleScope<'s, ()>,
        external: *mut T,
        global_fn: fn(
            &mut v8::HandleScope<'s, ()>,
            &mut v8::Local<'s, v8::ObjectTemplate>,
            external: v8::Local<'s, v8::External>,
        ),
    ) -> Runtime<'s, 'i> {
        let mut global = v8::ObjectTemplate::new(isolate_scope);

        global.set(
            v8::String::new(isolate_scope, "Log").unwrap().into(),
            v8::FunctionTemplate::new(
                isolate_scope,
                |scope: &mut v8::HandleScope,
                 args: v8::FunctionCallbackArguments,
                 mut _retval: v8::ReturnValue| {
                    let message = args.get(0);

                    let message: serde_json::Value = serde_v8::from_v8(scope, message).unwrap();
                    let message = message.to_string();

                    println!("{}", message);
                },
            )
            .into(),
        );

        let external = external as *mut c_void;
        let external = v8::External::new(isolate_scope, external);

        global_fn(isolate_scope, &mut global, external);
        let context = v8::Context::new_from_template(isolate_scope, global);
        let context_scope = v8::ContextScope::new(isolate_scope, context);
        Runtime {
            context_scope,
            context,
        }
    }

    pub fn executes(&mut self, file: String) {
        let script = v8::String::new(&mut self.context_scope, &file).unwrap();
        let scope = &mut v8::HandleScope::new(&mut self.context_scope);
        let try_catch = &mut v8::TryCatch::new(scope);

        let script =
            v8::Script::compile(try_catch, script, None).expect("failed to compile script");

        if script.run(try_catch).is_none() {
            let exception = try_catch.exception().unwrap();
            let exception_string = exception
                .to_string(try_catch)
                .unwrap()
                .to_rust_string_lossy(try_catch);

            panic!("{}", exception_string);
        }
    }

    pub fn call_fn(
        &mut self,
        func: v8::Local<'s, v8::Function>,
        args: &[v8::Local<v8::Value>],
    ) -> v8::Local<'s, v8::Value> {
        let global = self.context.global(&mut self.context_scope).into();

        func.call(&mut self.context_scope, global, args).unwrap()
    }

    pub fn get_fn(&mut self, fn_name: &str) -> Result<v8::Local<'s, v8::Function>, &str> {
        let draw_str = v8::String::new(&mut self.context_scope, fn_name).unwrap();
        let function = self
            .context
            .global(&mut self.context_scope)
            .get(&mut self.context_scope, draw_str.into())
            .unwrap();

        if let Ok(s) = v8::Local::<v8::Function>::try_from(function) {
            Ok(s)
        } else {
            Err("function not found")
        }
    }

    pub fn get_external<T>(args: &v8::FunctionCallbackArguments) -> &'s mut T {
        let ext = args.data().unwrap();
        let ext = v8::Local::<v8::External>::try_from(ext).unwrap();
        let ext = ext.value() as *mut T;

        unsafe { &mut *ext }
    }
}
