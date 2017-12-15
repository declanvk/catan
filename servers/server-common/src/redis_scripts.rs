use glob;
use redis_async::client;
use tokio_core::reactor::Core;
use futures::{future, Future};

use std::sync::Arc;
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::fs::{self, File};
use std::io::Read;
use std::net::SocketAddr;

use error::{ServerError, ServerResult};

pub fn load_scripts<P: AsRef<Path>>(
    script_folder: &P,
    redis_addr: &SocketAddr,
) -> ServerResult<HashMap<String, String>> {
    let metadata = fs::metadata(&script_folder)?;
    if !metadata.file_type().is_dir() {
        return Err(ServerError::Custom(
            "Path passed for script folder was not a directory"
                .to_owned(),
        ));
    }

    info!("Loading redis lua scripts from {}", script_folder.as_ref().display());

    let scripts: Vec<PathBuf> = glob::glob(&format!("{}/**/*.lua", script_folder.as_ref().display()))?
        .map(|value| value.unwrap())
        .collect();

    info!("Loaded scripts with paths: {:?}", scripts);

    let mut core = Core::new().unwrap();

    let connection = client::paired_connect(redis_addr, &core.handle());

    let send_data = connection.and_then(|connection| {
        let connection = Arc::new(connection);
        let script_shas = scripts.iter().map(move |script_path| {
            let mut script_file = File::open(script_path.as_path()).unwrap();
            let mut script_contents = String::new();
            script_file.read_to_string(&mut script_contents).unwrap();

            connection.send::<String>(resp_array!["SCRIPT", "LOAD", script_contents])
        });

        future::join_all(script_shas)
    });

    let resolved_shas: Vec<String> = core.run(send_data)?;
    let mut script_lookup = HashMap::new();
    script_lookup.extend(
        scripts
            .clone()
            .into_iter()
            .map(|v| {
                v.as_path()
                    .file_stem()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap()
            })
            .zip(resolved_shas.into_iter()),
    );

    return Ok(script_lookup);
}
