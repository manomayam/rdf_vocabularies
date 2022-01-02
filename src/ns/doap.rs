// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Description of a Project (DOAP) vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Description of a Project (DOAP) vocabulary|
//! |**Prefix**|doap|
//! |**Namespace base IRI**|[http://usefulinc.com/ns/doap#](http://usefulinc.com/ns/doap#)|
//! |**Description**|The Description of a Project (DOAP) vocabulary, described using W3C RDF Schema and the Web Ontology Language.|
//! |**Is defined by**|[http://usefulinc.com/ns/doap#](http://usefulinc.com/ns/doap#)|
//!

use crate::namespace;

namespace!(
    "http://usefulinc.com/ns/doap#",;
    /// `Úložiště GNU Arch`: Dépôt GNU Arch du code source.
    ArchRepository, "ArchRepository",
    /// `BitKeeper Repository`: Dépôt BitKeeper du code source.
    BKRepository, "BKRepository",
    /// `Bazaar Branch`: Código fonte da ramificação Bazaar.
    BazaarBranch, "BazaarBranch",
    /// `CVS Repository`: Dépôt CVS du code source.
    CVSRepository, "CVSRepository",
    /// `Dépôt darcs`: Repositorio darcs del código fuente.
    DarcsRepository, "DarcsRepository",
    /// `Ramificação Git`: Git source code branch.
    GitBranch, "GitBranch",
    /// `Repositório Git`: Git source code repository.
    GitRepository, "GitRepository",
    /// `Repositório Mercurial`: Mercurial source code repository.
    HgRepository, "HgRepository",
    /// `Projet`: A project.
    Project, "Project",
    /// `Dépôt`: Úložiště zdrojových kódů.
    Repository, "Repository",
    /// `Subversion Repository`: Subversion Quellcode-Versionierungssystem.
    SVNRepository, "SVNRepository",
    /// `Specification`: A especificação de aspetos, técnicas ou outros do sistema.
    Specification, "Specification",
    /// `Version`: Versionsinformation eines Projekt Releases.
    Version, "Version",
    /// `raíz anónima`: Repositorio para acceso anónimo.
    anon_root, "anon-root",
    /// `audience`: Description of target user base
    audience, "audience",
    /// `blog`: URI of a blog related to a project
    blog, "blog",
    /// `prohlížeč`: Interface web del repositorio.
    browse, "browse",
    /// `suivi des bugs`: Správa chyb projektu.
    bug_database, "bug-database",
    /// `categoria`: Una categoría de proyecto.
    category, "category",
    /// `créé`: Fecha en la que algo fue creado, en formato AAAA-MM-DD. e.g. 2004-04-05
    created, "created",
    /// `description`: Beschreibung eines Projekts als einfacher Text mit der Länge von 2 bis 4 Sätzen.
    description, "description",
    /// `Entwickler`: Programador de software para o projeto.
    developer, "developer",
    /// `developer forum`: A forum or community for developers of this project.
    developer_forum, "developer-forum",
    /// `documentador`: Collaborateur à la documentation du projet.
    documenter, "documenter",
    /// `Spiegel der Seite zum Herunterladen`: Mirror da página web para fazer download.
    download_mirror, "download-mirror",
    /// `Seite zum Herunterladen`: Web-Seite von der die Projekt-Software heruntergeladen werden kann.
    download_page, "download-page",
    /// `soubor revize`: URI of download associated with this release.
    file_release, "file-release",
    /// `colaborador`: Collaborateur au projet.
    helper, "helper",
    /// `Homepage`: O URL da página de um projeto,<br>		asociada com exactamente um projeto.
    homepage, "homepage",
    /// `Implements specification`: Uma especificação que um projeto implementa. Pode ser uma padrão, API ou um nível de conformidade definida legalmente.
    implements, "implements",
    /// `idioma`: Código de idioma ISO do projeto para o qual foi traduzido
    language, "language",
    /// `Lizenz`: L'URI d'une description RDF de la licence sous laquelle le programme est distribué.
    license, "license",
    /// `umístění úložiště`: Localização de um repositório.
    location, "location",
    /// `mailing list`: Página web de la lista de correo o dirección de correo.
    mailing_list, "mailing-list",
    /// `développeur principal`: Hauptentwickler eines Projektes, der Projektleiter
    maintainer, "maintainer",
    /// `modul`: Jméno modulu v CVS, BitKeeper nebo Arch úložišti.
    module, "module",
    /// `nombre`: Jméno něčeho.
    name, "name",
    /// `página web antigua`: L'URL d'une ancienne page web d'un<br>		projet, associée avec un unique projet.
    old_homepage, "old-homepage",
    /// `Betriebssystem`: Operační systém, na jehož použití je projekt limitován. Vynechejte tuto vlastnost, pokud je projekt nezávislý na operačním systému.
    os, "os",
    /// `plataforma`: Indicator of software platform (non-OS specific), e.g. Java, Firefox, ECMA CLR
    platform, "platform",
    /// `programovací jazyk`: Linguagem de programação que o projeto usa ou é para ser utilizada.
    programming_language, "programming-language",
    /// `release`: A publicação de um projeto.
    release, "release",
    /// `repositório`: Source code repository.
    repository, "repository",
    /// `repository of`: The project that uses a repository.
    repositoryOf, "repositoryOf",
    /// `versión`: Identificador do lançamento da revisão do software.
    revision, "revision",
    /// `screenshots`: Web page with screenshots of project.
    screenshots, "screenshots",
    /// `service endpoint`: The URI of a web service endpoint where software as a service may be accessed
    service_endpoint, "service-endpoint",
    /// `short description`: Texte descriptif concis (8 ou 9 mots) d'un projet.
    shortdesc, "shortdesc",
    /// `supporting forum`: A forum or community that supports this project.
    support_forum, "support-forum",
    /// `tester`: Tester nebo jiný spoluautor kontrolující kvalitu.
    tester, "tester",
    /// `traducteur`: Collaborateur à la traduction du projet.
    translator, "translator",
    /// `vendor`: Vendor organization: commercial, free or otherwise
    vendor, "vendor",
    /// `Wiki`: URL adresa wiki projektu pro společné diskuse.
    wiki, "wiki"
);
