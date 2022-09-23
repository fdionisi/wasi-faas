use std::{
    path::Path,
    sync::{Arc, RwLock},
};

use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::{tokio::WasiCtxBuilder, WasiCtx};

#[derive(Clone)]
pub struct Runtime {
    engine: Engine,
    linker: Arc<Linker<WasiCtx>>,
    modules: Vec<Module>,
}

impl Runtime {
    pub fn new() -> Self {
        let mut config = Config::new();
        // We need this engine's `Store`s to be async, and consume fuel, so
        // that they can co-operatively yield during execution.
        config.async_support(true);
        // config.consume_fuel(true);

        let engine = Engine::new(&config).unwrap();

        // A `Linker` is shared in the environment amongst all stores, and this
        // linker is used to instantiate the `module` above. This example only
        // adds WASI functions to the linker, notably the async versions built
        // on tokio.
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::tokio::add_to_linker(&mut linker, |cx| cx).unwrap();

        Runtime {
            engine,
            linker: Arc::new(linker),
            modules: vec![],
        }
    }

    pub fn add_module<P>(&mut self, module: P)
    where
        P: AsRef<Path>,
    {
        self.modules
            .push(Module::from_file(&self.engine, module).unwrap());
    }

    pub async fn exec(&self, req: Vec<u8>) -> Vec<u8> {
        let stdin = ReadPipe::from(req);

        let stdout_buf: Vec<u8> = vec![];
        let stdout_mutex = Arc::new(RwLock::new(stdout_buf));
        let stdout = WritePipe::from_shared(stdout_mutex.clone());

        let wasi = WasiCtxBuilder::new()
            .stdin(Box::new(stdin))
            .stdout(Box::new(stdout))
            .build();

        let mut store = Store::new(&self.engine, wasi);
        for module in self.modules.iter() {
            let instance = self
                .linker
                .instantiate_async(&mut store, &module)
                .await
                .unwrap();
            instance
                .get_typed_func::<(), (), _>(&mut store, "_start")
                .unwrap()
                .call_async(&mut store, ())
                .await
                .unwrap();
        }

        let mut buffer: Vec<u8> = Vec::new();
        stdout_mutex.read().unwrap().iter().for_each(|i| {
            buffer.push(*i);
        });

        buffer
    }
}
