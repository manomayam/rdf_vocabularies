// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Description of a Project (DOAP) vocabulary` vocabulary
//!
//! This module requires `ns-doap` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Description of a Project (DOAP) vocabulary|
//! |**Prefix**|doap|
//! |**Namespace base IRI**|<http://usefulinc.com/ns/doap#>|
//! |**Description**|The Description of a Project (DOAP) vocabulary, described using W3C RDF Schema and the Web Ontology Language.|
//! |**Is defined by**|<http://usefulinc.com/ns/doap#>|
//!

use crate::namespace;

namespace!(
    base: "http://usefulinc.com/ns/doap#",

    terms: [
        /// `GNU Arch repository`: GNU Arch source code repository.
        (ArchRepository, "ArchRepository"),
        /// `BitKeeper Repository`: BitKeeper source code repository.
        (BKRepository, "BKRepository"),
        /// `Bazaar Branch`: Bazaar source code branch.
        (BazaarBranch, "BazaarBranch"),
        /// `CVS Repository`: CVS source code repository.
        (CVSRepository, "CVSRepository"),
        /// `darcs Repository`: darcs source code repository.
        (DarcsRepository, "DarcsRepository"),
        /// `Git Branch`: Git source code branch.
        (GitBranch, "GitBranch"),
        /// `Git Repository`: Git source code repository.
        (GitRepository, "GitRepository"),
        /// `Mercurial Repository`: Mercurial source code repository.
        (HgRepository, "HgRepository"),
        /// `Project`: A project.
        (Project, "Project"),
        /// `Repository`: Source code repository.
        (Repository, "Repository"),
        /// `Subversion Repository`: Subversion source code repository.
        (SVNRepository, "SVNRepository"),
        /// `Specification`: A specification of a system's aspects, technical or otherwise.
        (Specification, "Specification"),
        /// `Version`: Version information of a project release.
        (Version, "Version"),
        /// `anonymous root`: Repository for anonymous access.
        (anon_root, "anon-root"),
        /// `audience`: Description of target user base
        (audience, "audience"),
        /// `blog`: URI of a blog related to a project
        (blog, "blog"),
        /// `browse`: Web browser interface to repository.
        (browse, "browse"),
        /// `bug database`: Bug tracker for a project.
        (bug_database, "bug-database"),
        /// `category`: A category of project.
        (category, "category"),
        /// `created`: Date when something was created, in YYYY-MM-DD form. e.g. 2004-04-05
        (created, "created"),
        /// `description`: Plain text description of a project, of 2-4 sentences in length.
        (description, "description"),
        /// `developer`: Developer of software for the project.
        (developer, "developer"),
        /// `developer forum`: A forum or community for developers of this project.
        (developer_forum, "developer-forum"),
        /// `documenter`: Contributor of documentation to the project.
        (documenter, "documenter"),
        /// `download mirror`: Mirror of software download web page.
        (download_mirror, "download-mirror"),
        /// `download page`: Web page from which the project software can be downloaded.
        (download_page, "download-page"),
        /// `file-release`: URI of download associated with this release.
        (file_release, "file-release"),
        /// `helper`: Project contributor.
        (helper, "helper"),
        /// `homepage`: URL of a project's homepage, 		associated with exactly one project.
        (homepage, "homepage"),
        /// `Implements specification`: A specification that a project implements. Could be a standard, API or legally defined level of conformance.
        (implements, "implements"),
        /// `language`: ISO language code a project has been translated into
        (language, "language"),
        /// `license`: The URI of an RDF description of the license the software is distributed under. E.g. a SPDX reference
        (license, "license"),
        /// `repository location`: Location of a repository.
        (location, "location"),
        /// `mailing list`: Mailing list home page or email address.
        (mailing_list, "mailing-list"),
        /// `maintainer`: Maintainer of a project, a project leader.
        (maintainer, "maintainer"),
        /// `module`: Module name of a Subversion, CVS, BitKeeper or Arch repository.
        (module, "module"),
        /// `name`: A name of something.
        (name, "name"),
        /// `old homepage`: URL of a project's past homepage, 		associated with exactly one project.
        (old_homepage, "old-homepage"),
        /// `operating system`: Operating system that a project is limited to.  Omit this property if the project is not OS-specific.
        (os, "os"),
        /// `platform`: Indicator of software platform (non-OS specific), e.g. Java, Firefox, ECMA CLR
        (platform, "platform"),
        /// `programming language`: Programming language a project is implemented in or intended for use with.
        (programming_language, "programming-language"),
        /// `release`: A project release.
        (release, "release"),
        /// `repository`: Source code repository.
        (repository, "repository"),
        /// `repository of`: The project that uses a repository.
        (repositoryOf, "repositoryOf"),
        /// `revision`: Revision identifier of a software release.
        (revision, "revision"),
        /// `screenshots`: Web page with screenshots of project.
        (screenshots, "screenshots"),
        /// `service endpoint`: The URI of a web service endpoint where software as a service may be accessed
        (service_endpoint, "service-endpoint"),
        /// `short description`: Short (8 or 9 words) plain text description of a project.
        (shortdesc, "shortdesc"),
        /// `supporting forum`: A forum or community that supports this project.
        (support_forum, "support-forum"),
        /// `tester`: A tester or other quality control contributor.
        (tester, "tester"),
        /// `translator`: Contributor of translations to the project.
        (translator, "translator"),
        /// `vendor`: Vendor organization: commercial, free or otherwise
        (vendor, "vendor"),
        /// `wiki`: URL of Wiki for collaborative discussion of project.
        (wiki, "wiki")    ]
);
