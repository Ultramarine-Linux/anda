

![Andaman Project](assets/anda-medium.png)

# Andaman

Andaman is a build system (and soon, a package manager) written in Rust.
It is inspired by Ultramarine Linux's `umpkg` project, and allows you to quickly build multiple types of projects into different artifacts.

> NOTE: Currently, Andaman only supports building RPM packages. Support for other artifact types will be added soon. Currently OCI/Docker is our secondary goal.

Andaman is planned to have the following features:

- Building artifact
- resolving and installing RPM packages
- Signing packages
- Build artifacts for:
    - Disk images and live media (powered by Lorax)
    - OSTree composes
    - RPM-OSTree composes
    - Docker images
    - Flatpak
- Generating whole repositories and composes from the above mentioned artifacts
- An extra user repository for packages that cannot be included in the main repositories (like the AUR)

It is planed to be the centerpiece of Ultramarine Linux (and tauOS) and its ecosystem, and a replacement for the existing [Koji](https://koji.build) system.

## Why?

Currently, Ultramarine Linux uses the [Koji](https://koji.build) system to build packages.

The build system itself is a bit of a mess, and requires a lot of manual setup to get a working system.
Koji contains a lot of legacy code and while flexible, it is very hard to use and maintain.
Fedora's packaging stack consists of many complicated services such as Bodhi and Pungi to add extra functionality to the system.
Which means that small communities like Ultramarine cannot use the same large stack of services.

The case is the same for Fyra Labs, who simply resorted to a series of very hacky solutions using Teleport to automatically build packages and push them to their
own repositories.
This is not very robust however, and scalability will become an issue when the number of repositories grows.

We want to create a stable, robust, and scalable build system that is easy to use and easy to maintain as an alternative to the Koji project.


## The architecture

Andaman is a build system written in Rust, and is powered by the following components:

- [Minio](https://min.io) (and S3 powered by the AWS Rust SDK) for storing artifacts
- [Rocket](https://rocket.rs) for the REST API server
- [Yew](https://yew.rs) for the web dashboard
- [RPM](https://rpm.org) the RPM package manager for building RPM packages
- [DNF](https://github.com/rpm-software-management/dnf) for resolving RPM packages and installing them (until we have a proper RPM frontend)
- [Kubernetes](https://kubernetes.io) for build orchestration
- [PostgreSQL](https://www.postgresql.org) for storing build metadata
- [SeaORM](https://www.sea-ql.org/SeaORM/) for database access

## Roadmap

* [x] Building RPM packages
* [x] Build artifact management
* [ ] Task scheduling for builds
* [ ] Full repository composition
* [ ] Build artifact signing
* [ ] OCI containers
* [ ] OSTree composes (Flatpak and RPM-OSTree)
* [ ] Building RPMs using an alternative package spec format
