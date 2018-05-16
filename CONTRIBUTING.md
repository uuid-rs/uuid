Contributing to Uuid
---
[Contributing to Uuid]: #contributing-to-uuid

Thank you for your interest in contributing to Uuid!

* [Feature Requests](#feature-requests)
* [Bug Reports](#bug-reports)
* [Pull Requests](#pull-requests)
* [Writing Documentation](#writing-documentation)
* [Issue Triage](#issue-triage)
* [Out-of-tree Contributions](#out-of-tree-contributions)
* [Helpful Links](#helpful-links)

For any questions, please make a post on [users.rust-lang.org][u-r-l-o], post
on [uuid-rs mailing list] or join our [gitter] channel.

*Reminder*: All contributors need to follow our [Code of Conduct].

[Code of Conduct]: CODE_OF_CONDUCT.md

# Feature Requests
[Feature Requests]: #feature-requests

`uuid` crate is still in flux. All features desired may not be present. As such
you are welcome to request for new features.

If you have the chance, please [search existing issues], as there is a chance
that someone has already requested your feature.

File your feature request with a descriptive title, as this helps others find
your request.

You can request your feature by following [this link][Feature Request Link] and
filling it in. The template used is shown below:

**Aside** We welcome pull requests for your own feature requests.

```markdown
**Is your feature request related to a problem? Please describe.**
A clear and concise description of what the problem is. Ex. I'm always frustrated when [...]

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Is it blocking?**
Is this issue blocking any of your work? If it is blocking any open source project, you can share the link of the issue

**Describe alternatives you've considered**
A clear and concise description of any alternative solutions or features you've considered.

**Additional context**
Add any other context or screenshots about the feature request here.

**Other**
Other information like relevant issues, external links, etc
```

[Feature Request Link]: https://github.com/uuid-rs/uuid/issues/new?template=Feature_request.md

# Bug Reports
[Bug Reports]: #bug-reports

While no one likes bugs, they are an unfortunate reality in software. Remember
we can't fix bugs we don't know about, so don't be shy about reporting.

If you have the chance, please [search existing issues], as there is a chance
that someone has already reported your error. This isn't strictly needed, as
sometimes you might not what exactly you are looking for.

File your issue with a descriptive title, as this helps others find your issue.

Reporting a bug is as easy as following [this link][Bug Report Link] and
filling it in. The template used is shown below:

```markdown
**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. ...
2. ...
3. ...

**Expected behavior**
A clear and concise description of what you expected to happen.

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Specifications (please complete the following information):**
- Target
- Version [e.g. 1.0]
- Features Enabled

**Additional context**
Add any other context about the problem here.

**Other**
Other information like relevant issues, external links, etc
```

Sometimes a backtrace may be needed. In that case, set `RUST_BACKTRACE`
environment variable to `1`. For example:

```bash
$ RUST_BACKTRACE=1 cargo build
```

**Aside** We welcome pull requests for your own bug reports.

[Bug Report Link]: https://github.com/uuid-rs/uuid/issues/new?template=Bug_report.md

# Pull Requests
[Pull Requests]: #pull-requests

Pull requests are the primary mechanism we use to change Uuid. GitHub itself
has some [great documentation] on using the Pull Request feature. We use the
"fork and pull" model described [here][fnp], where contributors push changes to
their personal fork and create pull requests to bring those changes into the
source repository.

Unless the changes are fairly minor (like documentation changes or tiny
patches), we require pull requests to relevant issues.

Please make pull requests against:
* `master` when making non-breaking changes 
* `breaking` if your changes alter the public API in a breaking manner

*Note* Our minimum rust version supported currently is `1.18.0`. Make sure you
don't use any Rust features introduced after this version. Our CI does test has
test builds for enforcing this restriction.

If the pull request is in work in progress stage, prepend`[WIP] ` in your PR
title. `WIP bot` will make sure that the PR doesn't accident get merged.

When you feel that the PR is ready, please ping one of the [maintainers] so 
they can review your changes.

[great documentation]: https://help.github.com/articles/about-pull-requests/
[fnp]: https://help.github.com/articles/about-collaborative-development-models/

# Writing Documentation
[Writing Documentation]: #writing-documentation

Documentation is an important part of Uuid. Lackluster or incorrect
documentation can cause headaches for the users of Uuid. Therefore,
documentation improvements are always welcome.

We follow the documentation style guidelines as given by [RFC 1574].

[RFC 1574]: https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md#appendix-a-full-conventions-text

# Issue Triage
[Issue Triage]: #issue-triage

Sometimes, an issue might stay open even after the relevant bug has been fixed.
Other times, the bug report may become invalid. Or we may just forget about the
bug.

You can help to go through old bug reports and check if they are still valid.
You can follow [this link][lrus] to look for issues like this.

[lrus]: https://github.com/uuid-rs/uuid/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-asc

# Out-of-tree Contributions
[Out-of-tree Contributions]: #out-of-tree-contributions

You can contribute to Uuid in other ways:

* Answer questions on [users.rust-lang.org][u-r-l-o], [uuid-rs mailing list] and/or
[gitter] channel.
* Find the [crates depending on `uuid`][dependent] and sending PRs to them,
helping them keep their version of `uuid` up-to-date.

[dependent]: https://crates.io/crates/uuid/reverse_dependencies

# Helpful Links
[Helpful Links]: #helpful-links

For people new to Uuid, and just starting to contribute, or even for more
seasoned developers, some useful places to look for information are:

* The Wikipedia entry on [Universally Unique Identifier][wiki-uuid].
* [RFC 4122] which gives the specification of Uuids.

[wiki-uuid]: https://en.wikipedia.org/wiki/Universally_unique_identifier
[RFC 4122]: https://www.ietf.org/rfc/rfc4122.txt

[u-r-l-o]: https://users.rust-lang.org
[uuid-rs mailing list]: https://uuid-rs.groups.io
[gitter]: https://gitter.im/uuid-rs/Lobby
[search existing issues]: https://github.com/uuid-rs/uuid/search?q=&type=Issues&utf8=%E2%9C%93
