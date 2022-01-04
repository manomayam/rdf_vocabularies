// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `The Bibliographic Ontology` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|The Bibliographic Ontology|
//! |**Prefix**|bibo|
//! |**Namespace base IRI**|[<http://purl.org/ontology/bibo/>](<http://purl.org/ontology/bibo/>)|
//! |**Description**|The Bibliographic Ontology describes bibliographic things on the semantic Web in RDF.  This ontology can be used as a citation ontology, as a document classification ontology, or simply as a way to describe any kind of document in RDF. It has been inspired by many existing document description metadata formats, and can be used as a common ground for converting other bibliographic data sources.|
//! |**Is defined by**|[<http://purl.org/ontology/bibo/>](<http://purl.org/ontology/bibo/>)|
//!

use crate::namespace;

namespace!(
    "<http://purl.org/ontology/bibo/>",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Academic Article`: A scholarly academic article, typically published in a journal.
    AcademicArticle, "AcademicArticle",
    /// `Article`: A written composition in prose, usually nonfiction, on a specific topic, forming an independent part of a book or other publication, as a newspaper or magazine.
    Article, "Article",
    /// `audio document`: An audio document; aka record.
    AudioDocument, "AudioDocument",
    /// `audio-visual document`: An audio-visual document; film, video, and so forth.
    AudioVisualDocument, "AudioVisualDocument",
    /// `Bill`: Draft legislation presented for discussion to a legal body.
    Bill, "Bill",
    /// `Book`: A written or printed work of fiction or nonfiction, usually on sheets of paper fastened or bound together within covers.
    Book, "Book",
    /// `Book Section`: A section of a book.
    BookSection, "BookSection",
    /// `Brief`: A written argument submitted to a court.
    Brief, "Brief",
    /// `Chapter`: A chapter of a book.
    Chapter, "Chapter",
    /// `Code`: A collection of statutes.
    Code, "Code",
    /// `Collected Document`: A document that simultaneously contains other documents.
    CollectedDocument, "CollectedDocument",
    /// `Collection`: A collection of Documents or Collections
    Collection, "Collection",
    /// `Conference`: A meeting for consultation or discussion.
    Conference, "Conference",
    /// `Court Reporter`: A collection of legal cases.
    CourtReporter, "CourtReporter",
    /// `Document`: A document (noun) is a bounded physical representation of body of information designed with the capacity (and usually intent) to communicate. A document may manifest symbolic, diagrammatic or sensory-representational information.
    Document, "Document",
    /// `document part`: a distinct part of a larger document or collected document.
    DocumentPart, "DocumentPart",
    /// `Document Status`: The status of the publication of a document.
    DocumentStatus, "DocumentStatus",
    /// `Edited Book`: An edited book.
    EditedBook, "EditedBook",
    /// `EMail`: A written communication addressed to a person or organization and transmitted electronically.
    Email, "Email",
    /// `Event`: 
    Event, "Event",
    /// `Excerpt`: A passage selected from a larger work.
    Excerpt, "Excerpt",
    /// `Film`: aka movie.
    Film, "Film",
    /// `Hearing`: An instance or a session in which testimony and arguments are presented, esp. before an official, as a judge in a lawsuit.
    Hearing, "Hearing",
    /// `Image`: A document that presents visual or diagrammatic information.
    Image, "Image",
    /// `Interview`: A formalized discussion between two or more people.
    Interview, "Interview",
    /// `Issue`: something that is printed or published and distributed, esp. a given number of a periodical
    Issue, "Issue",
    /// `Journal`: A periodical of scholarly journal Articles.
    Journal, "Journal",
    /// `Legal Case Document`: A document accompanying a legal case.
    LegalCaseDocument, "LegalCaseDocument",
    /// `Decision`: A document containing an authoritative determination (as a decree or judgment) made after consideration of facts or law.
    LegalDecision, "LegalDecision",
    /// `Legal Document`: A legal document; for example, a court decision, a brief, and so forth.
    LegalDocument, "LegalDocument",
    /// `Legislation`: A legal document proposing or enacting a law or a group of laws.
    Legislation, "Legislation",
    /// `Letter`: A written or printed communication addressed to a person or organization and usually transmitted by mail.
    Letter, "Letter",
    /// `Magazine`: A periodical of magazine Articles. A magazine is a publication that is issued periodically, usually bound in a paper cover, and typically contains essays, stories, poems, etc., by many writers, and often photographs and drawings, frequently specializing in a particular subject or area, as hobbies, news, or sports.
    Magazine, "Magazine",
    /// `Manual`: A small reference book, especially one giving instructions.
    Manual, "Manual",
    /// `Manuscript`: An unpublished Document, which may also be submitted to a publisher for publication.
    Manuscript, "Manuscript",
    /// `Map`: A graphical depiction of geographic features.
    Map, "Map",
    /// `Multivolume Book`: A loose, thematic, collection of Documents, often Books.
    MultiVolumeBook, "MultiVolumeBook",
    /// `Newspaper`: A periodical of documents, usually issued daily or weekly, containing current news, editorials, feature articles, and usually advertising.
    Newspaper, "Newspaper",
    /// `Note`: Notes or annotations about a resource.
    Note, "Note",
    /// `Patent`: A document describing the exclusive right granted by a government to an inventor to manufacture, use, or sell an invention for a certain number of years.
    Patent, "Patent",
    /// `Performance`: A public performance.
    Performance, "Performance",
    /// `Periodical`: A group of related documents issued at regular intervals.
    Periodical, "Periodical",
    /// `Personal Communication`: A communication between an agent and one or more specific recipients.
    PersonalCommunication, "PersonalCommunication",
    /// `Personal Communication Document`: A personal communication manifested in some document.
    PersonalCommunicationDocument, "PersonalCommunicationDocument",
    /// `Proceedings`: A compilation of documents published from an event, such as a conference.
    Proceedings, "Proceedings",
    /// `Quote`: An excerpted collection of words.
    Quote, "Quote",
    /// `Reference Source`: A document that presents authoritative reference information, such as a dictionary or encylopedia .
    ReferenceSource, "ReferenceSource",
    /// `Report`: A document describing an account or statement describing in detail an event, situation, or the like, usually as the result of observation, inquiry, etc..
    Report, "Report",
    /// `Series`: A loose, thematic, collection of Documents, often Books.
    Series, "Series",
    /// `Slide`: A slide in a slideshow
    Slide, "Slide",
    /// `Slideshow`: A presentation of a series of slides, usually presented in front of an audience with written text and images.
    Slideshow, "Slideshow",
    /// `Specification`: A document describing a specification.
    Specification, "Specification",
    /// `Standard`: A document describing a standard: a specification organized through a standards body.
    Standard, "Standard",
    /// `Statute`: A bill enacted into law.
    Statute, "Statute",
    /// `Thesis`: A document created to summarize research findings associated with the completion of an academic degree.
    Thesis, "Thesis",
    /// `Thesis degree`: The academic degree of a Thesis
    ThesisDegree, "ThesisDegree",
    /// `Webpage`: A web page is an online document available (at least initially) on the world wide web. A web page is written first and foremost to appear on the web, as distinct from other online resources such as books, manuscripts or audio documents which use the web primarily as a distribution mechanism alongside other more traditional methods such as print.
    Webpage, "Webpage",
    /// `Website`: A group of Webpages accessible on the Web.
    Website, "Website",
    /// `Workshop`: A seminar, discussion group, or the like, that emphasizes zxchange of ideas and the demonstration and application of techniques, skills, etc.
    Workshop, "Workshop",
    /// `abstract`: A summary of the resource.
    abstract_, "abstract",
    /// `affirmedBy`: A legal decision that affirms a ruling.
    affirmedBy, "affirmedBy",
    /// `annotates`: Critical or explanatory note for a Document.
    annotates, "annotates",
    /// `date argued`: The date on which a legal case is argued before a court. Date is of format xsd:date
    argued, "argued",
    /// `asin`: 
    asin, "asin",
    /// `list of authors`: An ordered list of authors. Normally, this list is seen as a priority list that order authors by importance.
    authorList, "authorList",
    /// `bdarcus`: 
    bdarcus, "bdarcus",
    /// `chapter`: An chapter number
    chapter, "chapter",
    /// `cited by`: Relates a document to another document that cites the first document.
    citedBy, "citedBy",
    /// `cites`: Relates a document to another document that is cited by the first document as reference, comment, review, quotation or for another purpose.
    cites, "cites",
    /// `coden`: 
    coden, "coden",
    /// `content`: This property is for a plain-text rendering of the content of a Document. While the plain-text content of an entire document could be described by this property.
    content, "content",
    /// `list of contributors`: An ordered list of contributors. Normally, this list is seen as a priority list that order contributors by importance.
    contributorList, "contributorList",
    /// `court`: A court associated with a legal document; for example, that which issues a decision.
    court, "court",
    /// `degree`: The thesis degree.
    degree, "degree",
    /// `M.A.`: masters degree in arts
    degrees/ma, "degrees/ma",
    /// `M.S.`: masters degree in science
    degrees/ms, "degrees/ms",
    /// `PhD degree`: PhD degree
    degrees/phd, "degrees/phd",
    /// `director`: A Film director.
    director, "director",
    /// `distributor`: Distributor of a document or a collection of documents.
    distributor, "distributor",
    /// `doi`: 
    doi, "doi",
    /// `eanucc13`: 
    eanucc13, "eanucc13",
    /// `edition`: The name defining a special edition of a document. Normally its a literal value composed of a version number and words.
    edition, "edition",
    /// `editor`: A person having managerial and sometimes policy-making responsibility for the editorial part of a publishing firm or of a newspaper, magazine, or other publication.
    editor, "editor",
    /// `list of editors`: An ordered list of editors. Normally, this list is seen as a priority list that order editors by importance.
    editorList, "editorList",
    /// `eissn`: 
    eissn, "eissn",
    /// `fgiasson`: 
    fgiasson, "fgiasson",
    /// `gtin14`: 
    gtin14, "gtin14",
    /// `handle`: 
    handle, "handle",
    /// `identifier`: 
    identifier, "identifier",
    /// `interviewee`: An agent that is interviewed by another agent.
    interviewee, "interviewee",
    /// `interviewer`: An agent that interview another agent.
    interviewer, "interviewer",
    /// `isbn`: 
    isbn, "isbn",
    /// `isbn10`: 
    isbn10, "isbn10",
    /// `isbn13`: 
    isbn13, "isbn13",
    /// `issn`: 
    issn, "issn",
    /// `issue`: An issue number
    issue, "issue",
    /// `issuer`: An entity responsible for issuing often informally published documents such as press releases, reports, etc.
    issuer, "issuer",
    /// `lccn`: 
    lccn, "lccn",
    /// `locator`: A description (often numeric) that locates an item within a containing document or collection.
    locator, "locator",
    /// `number of pages`: The number of pages contained in a document
    numPages, "numPages",
    /// `number of volumes`: The number of volumes contained in a collection of documents (usually a series, periodical, etc.).
    numVolumes, "numVolumes",
    /// `number`: A generic item or document number. Not to be confused with issue number.
    number, "number",
    /// `oclcnum`: 
    oclcnum, "oclcnum",
    /// `organizer`: The organizer of an event; includes conference organizers, but also government agencies or other bodies that are responsible for conducting hearings.
    organizer, "organizer",
    /// `owner`: Owner of a document or a collection of documents.
    owner, "owner",
    /// `page end`: Ending page number within a continuous page range.
    pageEnd, "pageEnd",
    /// `page start`: Starting page number within a continuous page range.
    pageStart, "pageStart",
    /// `pages`: A string of non-contiguous page spans that locate a Document within a Collection. Example: 23-25, 34, 54-56. For continuous page ranges, use the pageStart and pageEnd properties.
    pages, "pages",
    /// `performer`: 
    performer, "performer",
    /// `pmid`: 
    pmid, "pmid",
    /// `prefix name`: The prefix of a name
    prefixName, "prefixName",
    /// `presented at`: Relates a document to an event; for example, a paper to a conference.
    presentedAt, "presentedAt",
    /// `presents`: Relates an event to associated documents; for example, conference to a paper.
    presents, "presents",
    /// `producer`: Producer of a document or a collection of documents.
    producer, "producer",
    /// `recipient`: An agent that receives a communication document.
    recipient, "recipient",
    /// `reproducedIn`: The resource in which another resource is reproduced.
    reproducedIn, "reproducedIn",
    /// `reversedBy`: A legal decision that reverses a ruling.
    reversedBy, "reversedBy",
    /// `review of`: Relates a review document to a reviewed thing (resource, item, etc.).
    reviewOf, "reviewOf",
    /// `section`: A section number
    section, "section",
    /// `shortDescription`: 
    shortDescription, "shortDescription",
    /// `short title`: The abbreviation of a title.
    shortTitle, "shortTitle",
    /// `sici`: 
    sici, "sici",
    /// `status`: The publication status of (typically academic) content.
    status, "status",
    /// `accepted`: Accepted for publication after peer reviewing.
    status/accepted, "status/accepted",
    /// `draft`: Document drafted
    status/draft, "status/draft",
    /// `forthcoming`: Document to be published
    status/forthcoming, "status/forthcoming",
    /// `legal`: Legal document
    status/legal, "status/legal",
    /// `non peer reviewed`: A document that is not peer reviewed
    status/nonPeerReviewed, "status/nonPeerReviewed",
    /// `peer reviewed`: The process by which articles are chosen to be included in a refereed journal. An editorial board consisting of experts in the same field as the author review the article and decide if it is authoritative enough for publication.
    status/peerReviewed, "status/peerReviewed",
    /// `published`: Published document
    status/published, "status/published",
    /// `rejected`: Rejected for publication after peer reviewing.
    status/rejected, "status/rejected",
    /// `unpublished`: Unpublished document
    status/unpublished, "status/unpublished",
    /// `subsequentLegalDecision`: A legal decision on appeal that takes action on a case (affirming it, reversing it, etc.).
    subsequentLegalDecision, "subsequentLegalDecision",
    /// `suffix name`: The suffix of a name
    suffixName, "suffixName",
    /// `transcript of`: Relates a document to some transcribed original.
    transcriptOf, "transcriptOf",
    /// `translation of`: Relates a translated document to the original document.
    translationOf, "translationOf",
    /// `translator`: A person who translates written document from one language to another.
    translator, "translator",
    /// `upc`: 
    upc, "upc",
    /// `uri`: Universal Resource Identifier of a document
    uri, "uri",
    /// `volume`: A volume number
    volume, "volume"
);
