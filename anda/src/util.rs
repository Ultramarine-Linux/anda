use crate::error::PackerError;
use anyhow::{anyhow, Result};
use log::{debug, info};
use std::collections::HashSet;
use tokio::fs::File;
use std::path::{Path, PathBuf};
use std::{fs, io};
use tokio_tar::{Archive,Builder as TarBuilder};
use walkdir::WalkDir;
use async_compression::tokio::bufread::GzipDecoder;
use tokio::io::{BufReader, AsyncReadExt};
use async_compression::tokio::bufread::GzipEncoder;
use async_compression::tokio::{
    bufread::ZstdDecoder,
    write::ZstdEncoder,
};
use tokio_stream::StreamExt;

pub struct ProjectPacker;

impl ProjectPacker {
    pub async fn pack(path: &PathBuf, output: Option<String>) -> Result<PathBuf, PackerError> {
        // get folder name of path
        // check if path is folder
        if !path.is_dir() {
            return Err(PackerError::Path(format!(
                "{} is not a folder",
                path.display()
            )));
        }

        let path = if path.file_name().is_none() {
            path.canonicalize().unwrap()
        } else {
            path.to_path_buf()
        };

        debug!("packing {}", path.display());

        let folder_name = path.clone();
        let folder_name = folder_name.file_name().unwrap().to_str().unwrap();

        let tarball_path = if let Some(output) = output {
            PathBuf::from(output)
        } else {
            let tarball_path = format!("/tmp/{}.andasrc.tar", folder_name);
            PathBuf::from(tarball_path)
        };

        let mut tarball = File::create(&tarball_path).await?;


        ///let enc = GzipEncoder::new(tarball);

        let mut tar = TarBuilder::new(tarball);

        // parse gitignore file
        let gitignore_path = path.join(".gitignore");
        let andaignore_path = path.join(".andaignore");

        let mut file_list = HashSet::new();

        if gitignore_path.exists() {
            let gitignore = gitignore::File::new(&gitignore_path).unwrap();

            let files = gitignore.included_files();

            for file in files.unwrap() {
                let file_path = file.strip_prefix(&path).unwrap();
                debug!("adding {}", file_path.display());
                if file_path.exists() {
                    file_list.insert(file_path.to_path_buf());
                }
            }
        }

        if andaignore_path.exists() {
            let andaignore = gitignore::File::new(&andaignore_path).unwrap();

            let files = andaignore.included_files();

            for file in files.unwrap() {
                let file_path = file.strip_prefix(&path).unwrap();
                if file_path.exists() {
                    file_list.insert(file_path.to_path_buf());
                }
            }
        }

        tar.follow_symlinks(true);
        // if gitignore and andaignore files don't exists, add all files in folder
        if !andaignore_path.exists() && !gitignore_path.exists() {
            WalkDir::new(&path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .for_each(|e| {
                    let file_path = e.path().strip_prefix(&path).unwrap();
                    file_list.insert(file_path.to_path_buf());
                });
        }
        let old_dir = std::env::current_dir().unwrap();

        std::env::set_current_dir(path).unwrap();

        //let mut tasks = Vec::new();

        //tar.append_dir_all(".", path)?;
        for file in file_list {
            debug!("adding {}", file.display());

            // set current directory to path


            // spawn a thread to add file to tarball
            tar.append_path(file.as_path()).await?;
        }

        debug!("Finishing pack");
        tar.finish().await.unwrap();
        std::env::set_current_dir(old_dir).unwrap();

        Ok(PathBuf::from(tarball_path))
    }

    pub async fn unpack_and_build(path: &PathBuf, workdir: Option<PathBuf>) -> Result<(), PackerError> {
        let mut tarball = File::open(path).await?;

        let mut buf = Vec::new();

        tarball.read_to_end(&mut buf).await?;

        //let tar = GzipDecoder::new(buf.as_slice());

        let mut tar = Archive::new(buf.as_slice());

        let workdir = if let Some(workdir) = workdir {
            workdir
        } else {
            let workdir = PathBuf::from("/tmp/anda");

            workdir
        };

        if workdir.exists() {
            debug!("removing {}", workdir.display());
            // check if it's the default temp dir
            if !workdir.to_str().unwrap().contains("/tmp/") {
                info!("workdir already exists, do you want to delete it? (y/N)");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if input.trim() == "y" {
                    std::fs::remove_dir_all(&workdir).unwrap();
                } else {
                    return Err(PackerError::Path(format!(
                        "workdir already exists, please delete it manually"
                    )));
                }
            }
        }




        tar.unpack(workdir.as_path()).await?;
        let old_pwd = std::env::current_dir().unwrap();


        std::env::set_current_dir(&workdir).unwrap();

        // print current dir
        println!("{}", std::env::current_dir().unwrap().display());
        // execute anda build internally
        //async_compression::tokio::bufread::
        crate::build::ProjectBuilder::new(workdir).build().await?;

        Ok(())
    }
}
