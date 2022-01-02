// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Core organization ontology` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Core organization ontology|
//! |**Prefix**|org|
//! |**Namespace base IRI**|[http://www.w3.org/ns/org#](http://www.w3.org/ns/org#)|
//! |**Description**|Vocabulary for describing organizational structures, specializable to a broad variety of types of organization.|
//! |**Is defined by**|[http://www.w3.org/ns/org#](http://www.w3.org/ns/org#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/org#",;
    /// `Évènement`: Evento que da como resultado un cambio sustancial en la organización, por ejemplo, una fusión o una reestructuración total. Está pensado para situaciones en las que la organización resultante es lo suficientemente distinta de las organizaciones originales, tiene una identidad distinta y una URI también distinta. Se deberían definir subtipos de eventos mediante vocabularios específicos (Extension vocabularies) para referirse a categorías de eventos específicos. El momento o periodo en el que el evento ocurre se debería expresar mediante las propiedades `prov:startAtTime` y `prov:endedAtTime`, y una descripción del mismo se debería incluir mediante el uso de la propiedad `dct:description`.
    ChangeEvent, "ChangeEvent",
    /// `Organisation Formelle`: An Organization which is recognized in the world at large, in particular in legal jurisdictions, with associated rights and responsibilities. Examples include a Corporation, Charity, Government or Church. Note that this is a super class of `gr:BusinessEntity` and it is recommended to use the GoodRelations vocabulary to denote Business classifications such as DUNS or NAICS.
    FormalOrganization, "FormalOrganization",
    /// `director ejecutivo`: Un rôle correspondant à la propriété `org:headOf`
    Head, "Head",
    /// `Engagement`: Indica la natura della relazione di appartenenza di un Agent in un'organizzazione. Rappresenta una relazione n-aria tra un'Agent, un Organization e un Role. È possibile indicare direttamente la membership, indipendentemente dallo specifico Role, attraverso l'uso della proprietà `org:memberOf`
    Membership, "Membership",
    /// `Organization`: コミュニティー、その他の社会、商業、政治的な構造に共に編入された人々の集合を表わします。グループには、そこに属する人々を超えた、存在に対するある共通の目的や理由があり、エージェント（代理）を務めることができます。組織は、多くの場合、階層構造に分割できます。
    Organization, "Organization",
    /// `Collaborazione`: プロジェクトなどの2つ以上の組織間のコラボレーション。それは、アイデンティティを有し、その特定のメンバーとは無関係に目的を定めているという点で、組織としての基準を満たしますが、正式に認識された法的実体でも、あるより大きな組織内のサブユニットでもありません。一般的には、その内部の組織よりも存続期間が短いかもしれませんが、必ずしもそうとは限りません。
    OrganizationalCollaboration, "OrganizationalCollaboration",
    /// `unidad organizativa`: Organización que forma parte de una organización formal más amplia, como el servicio de informática o centro de cálculo de una universidad, y que sólo tiene reconocimiento pleno en el contexto de dicha organización formal, pero que no es una entidad legal propiamente dicha. Estas unidades pueden ser amplias y complejas, e incluir a otras unidades o incluso a otras organizaciones formales. Denominaciones alternativas: departamento.
    OrganizationalUnit, "OrganizationalUnit",
    /// `Impiego`: Un Poste représente une position au sein d'une Organisation qui existe indépendamment de la personne ou des personnes qui le remplissent. Les postes peuvent être utilisés pour représenter des situations où une personne est membre d'une Organisation d'office (par exemple, le Secrétaire d'Etat pour l'Ecosse fait partie du Cabinet du Royaume-Uni du fait d'être Secrétaire d'Etat pour l'Ecosse, non pas comme une personne physique). Un poste après peut être occupé par plusieurs personnes et peut donc être considéré comme une Organisation à part entière.
    Post, "Post",
    /// `Ruolo`: 人またはその他のエージェントが組織で担うことができる役割を表わします。この種のインスタンスは、抽象的な役割を記述します。特定の組織でその役割を担っている人の特定のインスタンスを示すためには、org:Membershipのインスタンスを使用します。
    Role, "Role",
    /// `Site`: An office or other premise at which the organization is located. Many organizations are spread across multiple sites and many sites will host multiple locations. In most cases a Site will be a physical location. However, we don't exclude the possibility of non-physical sites such as a virtual office with an associated post box and phone reception service. Extensions may provide subclasses to denote particular types of site.
    Site, "Site",
    /// `basata a`: Indicates the site at which a person is based. We do not restrict the possibility that a person is based at multiple sites.
    basedAt, "basedAt",
    /// `es modificado por`: Indica un evento che ha contribuito al cambiamento di questa organizzazione. A seconda dell'evento, l'organizzazione potrebbe essere esistente dopo l'evento o aver cessato la propria esistenza. È l'inverso di `org:originalOrganization`.
    changedBy, "changedBy",
    /// `classification`: Ordenación jerárquica que se hace de una organización dentro de un esquema de clasificación. Es posible que algunos vocabularios especifiquen esta propiedad de forma que el rango se corresponda con un `skos:ConceptScheme` específico. La conveniencia de incluir esta propiedad se está debatiendo y puede que se revise o elimine (en muchos casos las organizaciones se clasifican mejor si se define una jerarquía de subclases en un vocabulario aparte)
    classification, "classification",
    /// `ha membro`: 対象組織のメンバーであるエージェント（人または他の組織）を示します。org:memberOfの逆。さらに明確な説明については、そのプロパティーを参照してください。
    hasMember, "hasMember",
    /// `tiene membresía`: Indique pour cet Agent un engagement dans une Organisation. Inverse de `org:member`.
    hasMembership, "hasMembership",
    /// `possède un poste`: Indicate un Poste qui existe dans l'Organisation.
    hasPost, "hasPost",
    /// `sede principale`: Oficina principal de la organización, la opción por defecto para ponerse en contacto con una organización, aunque no corresponde necesariamente con las oficinas centrales de la organización.
    hasPrimarySite, "hasPrimarySite",
    /// `tiene sede registrada en`: Indicates the legally registered site for the organization, in many legal jurisdictions there is a requirement that FormalOrganizations such as Companies or Charities have such a primary designed site. 
    hasRegisteredSite, "hasRegisteredSite",
    /// `has site`: Lugar en donde la organización tiene algún tipo de presencia, incluso si es de forma indirecta (por ejemplo, una oficina virtual o servicio profesional que hagan la función de dirección registrada de la compañía). Es la relación inversa de `org:siteOf`.
    hasSite, "hasSite",
    /// `tiene suborganización`: Represents hierarchical containment of Organizations or Organizational Units; indicates an organization which is a sub-part or child of this organization.  Inverse of `org:subOrganizationOf`.
    hasSubOrganization, "hasSubOrganization",
    /// `possède une Unité`: 例えば、より大きな組織内の部局など、この組織の一部である単位を示します。
    hasUnit, "hasUnit",
    /// `head of`: Indique qu'une personne est le directeur ou le responsable formel d'une Organisation. Ceci indique souvent qu'il est au sommet de du graphe acyclique des `org:reportsTo`, même si une organisation peut avoir plus d'un responsable.
    headOf, "headOf",
    /// `ricoperto da`: Indica un Agent che ricopre un Post.
    heldBy, "heldBy",
    /// `holds`: Indicates a Post held by some Agent.
    holds, "holds",
    /// `identificatore`: Gives an identifier, such as a company registration number, that can be used to used to uniquely identify the organization. Many different national and international identier schemes are available. The org ontology is neutral to which schemes are used. The particular identifier scheme should be indicated by the datatype of the identifier value.  Using datatypes to distinguish the notation scheme used is consistent with recommended best practice for `skos:notation` of which this property is a specialization.
    identifier, "identifier",
    /// `linked to`: Relación arbitraria entre dos organizaciones. Las especializaciones de esta relación se pueden utilizar para denotar relaciones de financiación o suministro, entre otras.
    linkedTo, "linkedTo",
    /// `está ubicada en`: Indica la descrizione del luogo presso cui è possibile reperire una persona dell'organizzazione.
    location, "location",
    /// `member`: Indicates the Person (or other Agent including Organization) involved in the Membership relationship. Inverse of `org:hasMembership`
    member, "member",
    /// `durée d'engagement`: Optional property to indicate the interval for which the membership is/was valid.
    memberDuring, "memberDuring",
    /// `member of`: Indica che una persona è membro di una Organization senza una precisa indicazione sulla natura di questa appartenenza e sul suo ruolo. Si noti che la scelta del nome di questa proprietà non intende limitarla alla sola  rappresentazione formalmente di un'appartenenza. La proprietà può coprire anche altri coinvolgimenti nell'organizzazione. Questa proprietà può essere specializzata per indicare ruoli all'interno organizzazione o relazioni di diverse tipologie. Ha `org:hasmember` come proprietà inversa opzionale.
    memberOf, "memberOf",
    /// `organization`: Organización a la que pertenece el agente en calidad de miembro.
    organization, "organization",
    /// `organisation originelle`: Una o más organizaciones que existían antes de que sucediera el cambio en la organización. Dependiendo del tipo de cambio, dichas organizaciones pueden haber dejado de existir o no. Es la relación inversa de `org:changedBy`.
    originalOrganization, "originalOrganization",
    /// `impiego in`: Indica l'Organization in cui il Post è presente.
    postIn, "postIn",
    /// `purpose`: Indicates the purpose of this Organization. There can be many purposes at different levels of abstraction but the nature of an organization is to have a reason for existence and this property is a means to document that reason. An Organization may have multiple purposes. It is recommended that the purpose be denoted by a controlled term or code list, ideally a `skos:Concept`. However, the range is left open to allow for other types of descriptive schemes. It is expected that specializations or application profiles of this vocabulary will constrain the range of the purpose. Alternative names: _remit_ _responsibility_ (esp. if applied to OrganizationalUnits such as Government Departments).
    purpose, "purpose",
    /// `rémuneration`: Indica il salario o altra forma di remunerazione associata al ruolo. In genere, questo si denota usando uno schema di rappresentazione esistente come il `gr:PriceSpecification` ma il codominio è lasciato libero di essere specializzato a seconda delle applicazioni.
    remuneration, "remuneration",
    /// `reports to`: 組織図で描かれるかもしれないような上下関係を示します。エージェント間またはエージェントが就くことができるポスト間の上下関係を直接的に示すために使用できます。
    reportsTo, "reportsTo",
    /// `issue de`: この組織になった（導いた、作成された）きっかけとなった出来事を示します。
    resultedFrom, "resultedFrom",
    /// `a donné naissance à`: Indique une organisation qui a été créée ou a été modifiée à la suite d'un Évènement de changement. Inverse de `org:resultedFrom`.
    resultingOrganization, "resultingOrganization",
    /// `role`: Actividad que el agente desempeña en una relación de pertenencia a una organización.
    role, "role",
    /// `role (property)`: Questa è una meta-proprietà usata per annotare un'istanza di `org:Role` con una sotto-proprietà di `org:memberOf` e può essere usata per indicare direttamente il ruolo per facilitare un'interrogazione sui dati.
    roleProperty, "roleProperty",
    /// `indirizzo della sede`: Indica un indirizzo per la sede in una codifica appropriata. Il codominio è lasciato libero ma è consigliabile l'uso del vocabolario vCard (http://www.w3.org/TR/vcard-rdf/). L'indirizzo può includere email, numero di telefono e informazioni di geolocalizzazione e non è vincolato ad essere un indirizzo fisico.
    siteAddress, "siteAddress",
    /// `es sede de`: Indique une Organisation qui a une présence sur le site en question. Inverse de `org:hasSite`.
    siteOf, "siteOf",
    /// `sous-Organization de`: Represents hierarchical containment of Organizations or OrganizationalUnits; indicates an Organization which contains this Organization. Inverse of `org:hasSubOrganization`.
    subOrganizationOf, "subOrganizationOf",
    /// `es suborganización de manera transitiva de`: subOrganizationOfの推移閉包で、これを含むすべての組織の表現を与える。技術的に、これが推移閉包のスーパープロパティーであるため、追加の言明を含むことができますが、そのような使用法はお勧めできないことに注意してください。
    transitiveSubOrganizationOf, "transitiveSubOrganizationOf",
    /// `unité de`: Indicates an Organization of which this Unit is a part, e.g. a Department within a larger FormalOrganization. This is the inverse of `org:hasUnit`.
    unitOf, "unitOf"
);
