# git-bottle
The `Co-authored-by` [commit trailer](https://git-scm.com/docs/git-interpret-trailers) is the de-facto standard
to attribute a commit to multiple authors [^1] [^2] [^3] [^4], useful in particular when practicing
pair/mob/ensemble programming.

`git-bottle` is an interactive CLI tool to simplify the usage of `Co-authored-by` and other commit trailers.

## Usage
```
git bottle
```

<p align="center"><img src="/img/demo.gif?raw=true"/></p>

## Design principles
* zero-magic: it just runs `git commit -m <message>` where the message is built from your inputs
* zero-friction: you can start using it now, no configuration or changes required in your repository

## Installation

### From binaries

Oneliner for Linux:
```
curl -L https://github.com/simonecarriero/git-bottle/releases/download/0.1.0/git-bottle-x86_64-unknown-linux-gnu > git-bottle && \
    chmod +x git-bottle && \
    sudo mv git-bottle /usr/local/bin/git-bottle
```

Oneliner for macOS:
```
curl -L https://github.com/simonecarriero/git-bottle/releases/download/0.1.0/git-bottle-x86_64-apple-darwin > git-bottle && \
    chmod +x git-bottle && \
    sudo mv git-bottle /usr/local/bin/git-bottle
```

Change the last part (`sudo mv git-bottle /usr/local/bin/git-bottle`) if you prefer to use a different directory in
your `$PATH`.

### From cargo
```
cargo install --git https://github.com/simonecarriero/git-bottle
```

## Configuration
Configuration is not required. By default, `git-bottle` prompts for a message and a multi-selection of `Co-authored-by`
commit trailers, where the options are taken from the git log.

To customize the behavior, provide a `.git-bottle.yml` configuration file in your
repository or in any ancestor folder.

Take a look at the [schema of a .git-bottle.yml configuration file](docs/config/schema.md)
or at the following examples:
* [Example 1: multi-selection of `Co-authored-by` from the git log (default behavior)](docs/config/example_1.md)
* [Example 2: multi-selection of `Co-authored-by` from an explicit list of values](docs/config/example_2.md)
* [Example 3: selection of `Issue` from the last 10 commits in the git log](docs/config/example_3.md)
* [Example 4: Example 3 + Example 2](docs/config/example_4.md)

## References
[^1]: [git-core #451880 - Git should support multiple authors for a commit](https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=451880)
[^2]: [GitHub Blog - Commit together with co-authors](https://github.blog/2018-01-29-commit-together-with-co-authors/)
[^3]: [GitHub Docs - Creating a commit with multiple authors](https://docs.github.com/en/pull-requests/committing-changes-to-your-project/creating-and-editing-commits/creating-a-commit-with-multiple-authors)
[^4]: [GitLab Docs - Supported variables in commit templates](https://docs.gitlab.com/ee/user/project/merge_requests/commit_templates.html)
