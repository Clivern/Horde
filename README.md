<p align="center">
    <img src="/static/logo.png" width="200" />
    <h3 align="center">Horde</h3>
    <p align="center">A Fast, Secure and Reliable Terraform Backend, Set up in Minutes.</p>
    <p align="center">
        <a href="https://github.com/Clivern/Horde/actions"><img src="https://github.com/Clivern/Horde/actions/workflows/build.yml/badge.svg"></a>
        <a href="https://github.com/Clivern/Horde/releases"><img src="https://img.shields.io/badge/Version-v0.1.0-green.svg"></a>
        <a href="https://github.com/Clivern/Horde/blob/main/LICENSE"><img src="https://img.shields.io/badge/LICENSE-MIT-green.svg"></a>
    </p>
</p>


### Usage

To install and run `Horde`, Please do the following:

```zsh
$ cargo install horde-rs
$ export ROCKET_CONFIG=/etc/rocket.toml
$ horde -s
```


### Docker

To run postgresql with docker

```zsh
$ docker run -itd \
  -e POSTGRES_USER=root \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=horde \
  -p 5432:5432 \
  --name horde \
  postgres:latest
```


### Versioning

For transparency into our release cycle and in striving to maintain backward compatibility, Horde is maintained under the [Semantic Versioning guidelines](https://semver.org/) and release process is predictable and business-friendly.

See the [Releases section of our GitHub project](https://github.com/clivern/horde/releases) for changelogs for each release version of Horde. It contains summaries of the most noteworthy changes made in each release.


### Bug tracker

If you have any suggestions, bug reports, or annoyances please report them to our issue tracker at https://github.com/clivern/horde/issues


### Security Issues

If you discover a security vulnerability within Horde, please send an email to [hello@clivern.com](mailto:hello@clivern.com)


### Contributing

We are an open source, community-driven project so please feel free to join us. see the [contributing guidelines](CONTRIBUTING.md) for more details.


### License

© 2022, clivern. Released under [MIT License](https://opensource.org/licenses/mit-license.php).

**Horde** is authored and maintained by [@Clivern](http://github.com/clivern).
