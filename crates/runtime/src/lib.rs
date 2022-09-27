use std::{
    path::Path,
    sync::{Arc, RwLock},
};

use wasi_common::pipe::{ReadPipe, WritePipe};
use wasi_faas_http_outbound::{self, HttpOutbound};

use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::{tokio::WasiCtxBuilder, WasiCtx};

struct Context {
    pub wasi: WasiCtx,
    pub http_outbound: HttpOutbound,
}

#[derive(Clone)]
pub struct Runtime {
    engine: Engine,
    linker: Arc<Linker<Context>>,
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
        let mut linker: Linker<Context> = Linker::new(&engine);

        wasmtime_wasi::tokio::add_to_linker(&mut linker, |cx| &mut cx.wasi).unwrap();
        wasi_faas_http_outbound::add_to_linker(&mut linker, |cx| &mut cx.http_outbound).unwrap();

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

    pub async fn exec(
        &self,
        args: Vec<String>,
        envs: Vec<(String, String)>,
        stdin: Vec<u8>,
    ) -> Vec<u8> {
        let stdin = ReadPipe::from(stdin);

        let stdout_buf: Vec<u8> = vec![];
        let stdout_mutex = Arc::new(RwLock::new(stdout_buf));
        let stdout = WritePipe::from_shared(stdout_mutex.clone());

        let wasi = WasiCtxBuilder::new()
            .args(&args)
            .unwrap()
            .envs(&envs)
            .unwrap()
            .stdin(Box::new(stdin))
            .stdout(Box::new(stdout))
            .inherit_stderr()
            .build();

        let context = Context {
            wasi,
            http_outbound: HttpOutbound::new(wasi_faas_http_outbound::AllowedHttpHosts::AllowAll),
        };

        let mut store = Store::new(&self.engine, context);
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
