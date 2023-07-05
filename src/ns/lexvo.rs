// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Lexvo.org Ontology` vocabulary
//!
//! This module requires `ns-lexvo` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Lexvo.org Ontology|
//! |**Prefix**|lexvo|
//! |**Namespace base IRI**|<http://lexvo.org/ontology#>|
//! |**Description**|Lexvo.org Ontology Version 0.24 (2019-01-16)|
//! |**Is defined by**|<http://rdf.greggkellogg.net/distiller?command=serialize&output_format=ntriples&url=http:%2F%2Flexvo.org%2Fontology&raw>|
//!

use crate::namespace;

namespace!(
    base: "http://lexvo.org/ontology#",

    terms: [
        /// `radical`: A CJK character radical.
        (CJKRadical, "CJKRadical"),
        /// `character`: An abstract character as defined by the Unicode Standard,   Version 5, in Chapter 3.4.
        (Character, "Character"),
        /// `geographic region`: Geographic regions on Earth or elsewhere.
        (GeographicRegion, "GeographicRegion"),
        /// `language`: Groups of human language variants that are or were spoken, written, or signed at some point in time.
        (Language, "Language"),
        /// `script`: A writing system considered abstractly, i.e. independent     of language-specific variations.
        (Script, "Script"),
        /// `term`: A term in a specific language.
        (Term, "Term"),
        /// `broader`: The property of having a broader, more generic concept.
        (broader, "broader"),
        /// `translation`: The property of a CJK character being conventionally     associated with a CJK character radical.
        (characterRadical, "characterRadical"),
        /// `contains character`: The property of a term containing a specific character.
        (containsCharacter, "containsCharacter"),
        /// `evokes`: The property of conceptually evoking some object.
        (evokes, "evokes"),
        /// `character component`: The property of a character being composed of another character.
        (hasCharacterComponent, "hasCharacterComponent"),
        /// `has member`: The property of having a member.
        (hasMember, "hasMember"),
        /// `in script`: The property of a document being written in a specific script,     or a language being written or having being written in a specific script by a considerable     group of language users.
        (inScript, "inScript"),
        /// `involved in`: The property of being involved in something, e.g. for semantic participants' involvement in a semantic frame.
        (involvedIn, "involvedIn"),
        /// `involves`: The property of involving some entity, e.g. for semantic participants involved in a semantic frame.
        (involves, "involves"),
        /// `is focus of`:
        (isFocusOf, "isFocusOf"),
        /// `ISO 15924 alphabetic code`:
        (iso15924Alphacode, "iso15924Alphacode"),
        /// `ISO 639-1 code`:
        (iso639P1Code, "iso639P1Code"),
        /// `ISO 639-2 Bibliographical code`:
        (iso639P2BCode, "iso639P2BCode"),
        /// `ISO 639-2 Terminological code`:
        (iso639P2TCode, "iso639P2TCode"),
        /// `ISO 639-3 code`:
        (iso639P3Code, "iso639P3Code"),
        /// `ISO 639-5 code`:
        (iso639P5Code, "iso639P5Code"),
        /// `has label`: the semiotic property of possessing a natural language label (as opposed to artificial identifiers), e.g. the city of Paris is lexicalized in Italian using the Italian term "Parigi".
        (label, "label"),
        /// `in language`: The property of a resource being mainly encoded in a specific language.
        (language, "language"),
        /// `lexical category`: A lexical category that a term can be used in.
        (lexicalCategory, "lexicalCategory"),
        /// `MARC 21 / USMARC code`:
        (marcCode, "marcCode"),
        /// `means`: the semiotic property of a natural language word meaning something, i.e. a     relationship between words and resources that they represent (or represented) to a considerable number of humans
        (means, "means"),
        /// `member of`: The property of being a member of a group.
        (memberOf, "memberOf"),
        /// `narrower`: The property of having a narrower, less generic concept.
        (narrower, "narrower"),
        /// `nearly same as`: The property of being at least nearly the same as something else. For instance,     the geographical area comprising the island of Malta is nearly the same as the island of Malta.
        (nearlySameAs, "nearlySameAs"),
        /// `non-radical strokes`: The property of a character conventionally being     written with a specific number strokes, excluding the strokes of its radical.
        (nonRadicalStrokes, "nonRadicalStrokes"),
        /// `represented by`:
        (representedBy, "representedBy"),
        /// `represents`: the semiotic property of semantically representing something, i.e. a     relationship between words and other signs and resources that they represent (or represented) to a considerable number of humans
        (represents, "represents"),
        /// `script for`: The property of a script being used for a particular language by    a considerable of language users (but not necessarily the majority of all language users).
        (scriptFor, "scriptFor"),
        /// `somewhat same as`: The property of being at least somewhat the same as something else. For instance,     the City of Los Angeles is somewhat the same as the Greater Los Angeles area.
        (somewhatSameAs, "somewhatSameAs"),
        /// `strength`: The strength of an rdf:Statement, given is a value in [0,1].
        (strength, "strength"),
        /// `same as`: The property of genuine identity in the Leibnizian sense.
        (strictlySameAs, "strictlySameAs"),
        /// `subClassOf`:
        (subClassOf, "subClassOf"),
        /// `translation`: translational equivalence between words and other signs     and resources that they represent to or represented to a considerable number of humans
        (translation, "translation"),
        /// `used in`: The property of a language or writing system     being used somewhat extensively in a particular geographical region      at some point in time.
        (usedIn, "usedIn"),
        /// `uses script`: The property of a language being written or having being written in a specific      script by a considerable group of language users.
        (usesScript, "usesScript"),
        /// `variant`: The property of being a variant of another resource.
        (variant, "variant")    ]
);
