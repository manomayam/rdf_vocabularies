// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `The Geonames ontology` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|The Geonames ontology|
//! |**Prefix**|gn|
//! |**Namespace base IRI**|[http://www.geonames.org/ontology#](http://www.geonames.org/ontology#)|
//! |**Description**|The Geonames ontologies provides elements of description for geographical features, in particular those defined in the geonames.org data base|
//! |**Is defined by**|[http://www.geonames.org/ontology#](http://www.geonames.org/ontology#)|
//!

use crate::namespace;

namespace!(
    "http://www.geonames.org/ontology#",;
    /// ``: country, state, region ...
    A, "A",
    /// `första ordningens administrativ avdelning`: a primary administrative division of a country, such as a state in the United States
    A_ADM1, "A.ADM1",
    /// `historical first-order administrative division`: 
    A_ADM1H, "A.ADM1H",
    /// `второстепенна административна единица`: дальнейшее деление политико-административного деления первого порядка
    A_ADM2, "A.ADM2",
    /// `historical second-order administrative division`: 
    A_ADM2H, "A.ADM2H",
    /// `tredje ordningens administrativ avdelning`: a subdivision of a second-order administrative division
    A_ADM3, "A.ADM3",
    /// `historical third-order administrative division`: 
    A_ADM3H, "A.ADM3H",
    /// `fjärde ordningens administrativ avdelning`: a subdivision of a third-order administrative division
    A_ADM4, "A.ADM4",
    /// `historical fourth-order administrative division`: 
    A_ADM4H, "A.ADM4H",
    /// `fifth-order administrative division`: a subdivision of a fourth-order administrative division
    A_ADM5, "A.ADM5",
    /// `administrativ avdelning`: an administrative division of a country, undifferentiated as to administrative level
    A_ADMD, "A.ADMD",
    /// `historical administrative division`: 
    A_ADMH, "A.ADMH",
    /// `uthyrt område`: арендованный правительством Великобритании у властей Китая участок земли, часть Гонконга
    A_LTER, "A.LTER",
    /// `politisk enhet`: 
    A_PCL, "A.PCL",
    /// `beroende politisk enhet`: 
    A_PCLD, "A.PCLD",
    /// `fritt associerad stat`: 
    A_PCLF, "A.PCLF",
    /// `historical political entity`: 
    A_PCLH, "A.PCLH",
    /// `независимое политическое образование`: 
    A_PCLI, "A.PCLI",
    /// `част от независимо политическо образувание`: 
    A_PCLIX, "A.PCLIX",
    /// `наполовину зависимое политическое образование`: 
    A_PCLS, "A.PCLS",
    /// `historical capital of a political entity`: 
    A_PPCLH, "A.PPCLH",
    /// `historical populated place`: 
    A_PPLH, "A.PPLH",
    /// `енория`: an ecclesiastical district
    A_PRSH, "A.PRSH",
    /// `territory`: 
    A_TERR, "A.TERR",
    /// `зона`: 
    A_ZN, "A.ZN",
    /// `bufferzon`: область, официально признанная в качестве буфера между двумя нациями; военное присутствие минимально или вовсе отсутствует
    A_ZNB, "A.ZNB",
    /// `Class`: A class of features.
    Class, "Class",
    /// `Code`: A feature code.
    Code, "Code",
    /// `Feature`: A geographical feature
    Feature, "Feature",
    /// `Geonames Feature`: A feature described in geonames database, uniquely defined by its geonames identifier
    GeonamesFeature, "GeonamesFeature",
    /// ``: stream, lake, ...
    H, "H",
    /// `seaplane landing area`: место на воде для взлета и посадки гидросамолета
    H_AIRS, "H.AIRS",
    /// `anchorage`: an area where vessels may anchor
    H_ANCH, "H.ANCH",
    /// `залив`: a coastal indentation between two capes or headlands, larger than a cove but smaller than a gulf
    H_BAY, "H.BAY",
    /// `bukter`: coastal indentations between two capes or headlands, larger than a cove but smaller than a gulf
    H_BAYS, "H.BAYS",
    /// `излучина (реки)`: an open body of water forming a slight recession in a coastline
    H_BGHT, "H.BGHT",
    /// `banker`: возвышенность, которая обычно располагается на мелководье; однако количества воды достаточно для надводного судоходства
    H_BNK, "H.BNK",
    /// `бряг изложен на течение`: a sloping margin of a stream channel which normally confines the stream to its channel on land
    H_BNKR, "H.BNKR",
    /// `част от бряг`: 
    H_BNKX, "H.BNKX",
    /// `kärr, träsk`: a wetland characterized by peat forming sphagnum moss, sedge, and other acid-water plants
    H_BOG, "H.BOG",
    /// `шапка полярного льда`: куполообразная масса ледникового льда, покрывающая гонные вершины; по размеру меньше ледникового щита
    H_CAPG, "H.CAPG",
    /// `channel`: the deepest part of a stream, bay, lagoon, or strait, through which the main current flows
    H_CHN, "H.CHN",
    /// `sjökanal(er)`: наиболее глубокое место в озере; здесь могут проходить суда
    H_CHNL, "H.CHNL",
    /// `skipsled på sjøen`: that part of a body of water deep enough for navigation through an area otherwise not suitable
    H_CHNM, "H.CHNM",
    /// `судоходное русло`: a buoyed channel of sufficient depth for the safe navigation of vessels
    H_CHNN, "H.CHNN",
    /// `место слияние (рек)`: место, где две (или больше) реки соединяются в одну
    H_CNFL, "H.CNFL",
    /// `канал`: искусственная река
    H_CNL, "H.CNL",
    /// `акведук/водопровод`: a conduit used to carry water
    H_CNLA, "H.CNLA",
    /// `canal bend`: a conspicuously curved or bent section of a canal
    H_CNLB, "H.CNLB",
    /// `водоотводный канал`: искусственный судоходный канал, забирает лишнюю воду  либо в заболоченной местности, либо из дренажного канала
    H_CNLD, "H.CNLD",
    /// `irrigation canal`: a canal which serves as a main conduit for irrigation water
    H_CNLI, "H.CNLI",
    /// `navigation canal(s)`: a watercourse constructed for navigation of vessels
    H_CNLN, "H.CNLN",
    /// `abandoned canal`: 
    H_CNLQ, "H.CNLQ",
    /// `подземна напоителна система`: подземный туннель, по которому течет вода, используемая  для орошения; вода берется из водоносного слоя почвы
    H_CNLSB, "H.CNLSB",
    /// `част от канал`: 
    H_CNLX, "H.CNLX",
    /// `заливче`: a small coastal indentation, smaller than a bay
    H_COVE, "H.COVE",
    /// `небольшой временный водоток (связанный с приливами и отливами)`: a meandering channel in a coastal wetland subject to bi-directional tidal currents
    H_CRKT, "H.CRKT",
    /// `strøm`: a horizontal flow of water in a given direction with uniform velocity
    H_CRNT, "H.CRNT",
    /// `скъсяване`: канал, образованный в результате потока воды, идущей  через шейку меандра
    H_CUTF, "H.CUTF",
    /// `док`: a waterway between two piers, or cut into the land for the berthing of ships
    H_DCK, "H.DCK",
    /// `havnebasseng`: a part of a harbor where ships dock
    H_DCKB, "H.DCKB",
    /// `ледниковый купол`: a comparatively elevated area on an icecap
    H_DOMG, "H.DOMG",
    /// `fordypning i iskappen`: небольшое углубление на поверхности ледника
    H_DPRG, "H.DPRG",
    /// `канавка`: небольшой искусственный водоем, используемый для осушения и орошения территории
    H_DTCH, "H.DTCH",
    /// `drainage ditch`: a ditch which serves to drain the land
    H_DTCHD, "H.DTCHD",
    /// `bevattningsdike`: a ditch which serves to distribute irrigation water
    H_DTCHI, "H.DTCHI",
    /// `ditch mouth(s)`: место, где осушительный канал впадает в лагуну, озеро или залив
    H_DTCHM, "H.DTCHM",
    /// `заливно устие`: однорукавное, воронкообразное устье реки, расширяющееся в сторону моря
    H_ESTY, "H.ESTY",
    /// `риболовен район`: место ловли рыбы
    H_FISH, "H.FISH",
    /// `фьорд`: a long, narrow, steep-walled, deep-water arm of the sea at high latitudes, usually along mountainous coasts
    H_FJD, "H.FJD",
    /// `fjorder`: long, narrow, steep-walled, deep-water arms of the sea at high latitudes, usually along mountainous coasts
    H_FJDS, "H.FJDS",
    /// `водопад(и)`: падение воды в реке с уступа, пересекающего речное русло
    H_FLLS, "H.FLLS",
    /// `del av vattenfall`: 
    H_FLLSX, "H.FLLSX",
    /// `mud flat(s)`: a relatively level area of mud either between high and low tide lines, or subject to flooding
    H_FLTM, "H.FLTM",
    /// `tidal flat(s)`: береговая полоса, покрываемая водой при приливе
    H_FLTT, "H.FLTT",
    /// `ледник`: масса льда преимущественно атмосферного происхождения, испытывающая вязкопластическое течение под действием силы тяжести и принявшая форму потока, системы потоков, купола (щита) или плавучей плиты
    H_GLCR, "H.GLCR",
    /// `golf`: вдавшаяся в сушу часть моря
    H_GULF, "H.GULF",
    /// `geysir`: a type of hot spring with intermittent eruptions of jets of hot water and steam
    H_GYSR, "H.GYSR",
    /// `гавань`: естественная или искусственная защищенная от ветра и волн прибрежная часть водного пространства, служащая местом стоянки, причала и ремонта судов
    H_HBR, "H.HBR",
    /// `част от пристанище`: 
    H_HBRX, "H.HBRX",
    /// `innløp`: узкий водный путь, простирающийся до суши; или связывающий бухту или лагуну с большим водным образованием
    H_INLT, "H.INLT",
    /// `засыпанный узкий пролив`: узкий пролив, который ранее был пригоден для судоходства, в настоящее время пролив засыпан
    H_INLTQ, "H.INLTQ",
    /// `озерная котловина`: место, которое осталось после высохшего или специально осушенного озера
    H_LBED, "H.LBED",
    /// `lagun`: a shallow coastal waterbody, completely or partly separated from a larger body of water by a barrier island, coral reef or other depositional feature
    H_LGN, "H.LGN",
    /// `laguner`: мелководные заливы или озеро, отделившиеся от моря вследствие образования песчаной полосы
    H_LGNS, "H.LGNS",
    /// `del av lagune`: 
    H_LGNX, "H.LGNX",
    /// `sjö`: замкнутый естественный водоём в углублении суши, котловине
    H_LK, "H.LK",
    /// `кратерное озеро`: a lake in a crater or caldera
    H_LKC, "H.LKC",
    /// `tilbakevendende innsjø`: 
    H_LKI, "H.LKI",
    /// `соленое озеро`: an inland body of salt water with no outlet
    H_LKN, "H.LKN",
    /// `intermittent salt lake`: 
    H_LKNI, "H.LKNI",
    /// `корекция`: серповидное озеро, прилегающее  к извилистым рекам
    H_LKO, "H.LKO",
    /// `tilbakevendende kroksjø`: 
    H_LKOI, "H.LKOI",
    /// `innsjøer`: large inland bodies of standing water
    H_LKS, "H.LKS",
    /// `подземни езера`: озеро в карстовых пещерах, питающееся подземными водами
    H_LKSB, "H.LKSB",
    /// `kratersjöar`: озера в кратере или кальдере (круглое или овальное углубление в жерле вулкана)
    H_LKSC, "H.LKSC",
    /// `intermittent lakes`: 
    H_LKSI, "H.LKSI",
    /// `saltsjöar`: озера, вода которых сильно минерализована; содержит большое количество солей
    H_LKSN, "H.LKSN",
    /// `исчезающие соленые озера`: 
    H_LKSNI, "H.LKSNI",
    /// `част от езеро`: 
    H_LKX, "H.LKX",
    /// `saltdammar`: diked salt ponds used in the production of solar evaporated salt
    H_MFGN, "H.MFGN",
    /// `мангрови блата`: a tropical tidal mud flat characterized by mangrove vegetation
    H_MGV, "H.MGV",
    /// `mo`: залежи торфа, торфяное болото
    H_MOOR, "H.MOOR",
    /// `marsh(es)`: заболоченная территория, заросшая травой
    H_MRSH, "H.MRSH",
    /// `salt sumpmark`: равнинная местность, подверженная затоплению соленой водой; здесь произрастают преимущественно травянистые солеустойчивые растения
    H_MRSHN, "H.MRSHN",
    /// `narrows`: a navigable narrow part of a bay, strait, river, etc.
    H_NRWS, "H.NRWS",
    /// `hav`: one of the major divisions of the vast expanse of salt water covering part of the earth
    H_OCN, "H.OCN",
    /// `бързеи`: an area of breaking waves caused by the meeting of currents or by waves moving against the current
    H_OVF, "H.OVF",
    /// `пруд`: a small standing waterbody
    H_PND, "H.PND",
    /// `periodisk damm`: 
    H_PNDI, "H.PNDI",
    /// `saltdamm`: небольшой водоем с соленой водой, обычно в болотистой местности, или вдоль морского побережья
    H_PNDN, "H.PNDN",
    /// `временно исчезающий соленый пруд`: 
    H_PNDNI, "H.PNDNI",
    /// `dammar`: небольшие водоемы со стоячей водой
    H_PNDS, "H.PNDS",
    /// `fishponds`: специальный искусственный водоем, в котором разводят рыб
    H_PNDSF, "H.PNDSF",
    /// `временно исчезающие пруды`: 
    H_PNDSI, "H.PNDSI",
    /// `salt ponds`: small standing bodies of salt water often in a marsh or swamp, usually along a seacoast
    H_PNDSN, "H.PNDSN",
    /// `бассейн`: a small and comparatively still, deep part of a larger body of water such as a stream or harbor; or a small body of standing water
    H_POOL, "H.POOL",
    /// `пресъхващи вирове`: 
    H_POOLI, "H.POOLI",
    /// `плёс`: a straight section of a navigable stream or channel between two bends
    H_RCH, "H.RCH",
    /// `заледена част на река между два завоя/на канал между два шлюза`: узкая и длинная возвышенность на ледниковой поверхности
    H_RDGG, "H.RDGG",
    /// `redd`: часть акватории порта, предназначенная для якорной стоянки судов и перегрузки грузов
    H_RDST, "H.RDST",
    /// `rev`: резкое надводное или подводное возвышение морского дна на мелководьях, препятствующее судоходству
    H_RF, "H.RF",
    /// `коралловые рифы`: a surface-navigation hazard composed of coral
    H_RFC, "H.RFC",
    /// `участок риф`: 
    H_RFX, "H.RFX",
    /// `forsar`: a turbulent section of a stream associated with a steep, irregular stream bed
    H_RPDS, "H.RPDS",
    /// `reservoar`: an artificial pond or lake
    H_RSV, "H.RSV",
    /// `пресъхващи водохранилища`: 
    H_RSVI, "H.RSVI",
    /// `воден ресервуар`: наполненный водой бак или резервуар, находящийся на, выше или ниже уровня земли
    H_RSVT, "H.RSVT",
    /// `клисура(и)`: a small, narrow, deep, steep-sided stream channel, smaller than a gorge
    H_RVN, "H.RVN",
    /// `sabkha (oversvømt område)`: замкнутое бессточное понижение рельефа с плоским глинистым днищем, занятым солончаками
    H_SBKH, "H.SBKH",
    /// `sund`: длинный морской рукав, образующий пролив между материком и островом, или островами
    H_SD, "H.SD",
    /// `море`: большое водное пространство, ограниченное с одной или нескольких сторон сушей и отделяемое от самого океана островами или возвышенностями подводного рельефа
    H_SEA, "H.SEA",
    /// `grund, rev`: участок (обычно прибрежный) реки, озера и т.п. с небольшой глубиной
    H_SHOL, "H.SHOL",
    /// `tröskel`: вытянутое горное поднятие на дне океанов и морей. Часто подводные хребты выступают над поверхностью воды в виде островов или целых архипелагов
    H_SILL, "H.SILL",
    /// `извор(и)`: a place where ground water flows naturally out of the ground
    H_SPNG, "H.SPNG",
    /// `серный источник`: a place where sulphur ground water flows naturally out of the ground
    H_SPNS, "H.SPNS",
    /// `varmkälla (or)`: источник термальных вод, температура которых выше 20°C
    H_SPNT, "H.SPNT",
    /// `ручей`: a body of running water moving to a lower level in a channel on land
    H_STM, "H.STM",
    /// `протока`: водоток, отходящий от реки и впадающий в неё ниже по течению;
    H_STMA, "H.STMA",
    /// `elvesving`: a conspicuously curved or bent segment of a stream
    H_STMB, "H.STMB",
    /// `kanaliserat vattendrag`: a stream that has been substantially ditched, diked, or straightened
    H_STMC, "H.STMC",
    /// `biflod(er)`: a branch which flows away from the main stream, as in a delta or irrigation canal
    H_STMD, "H.STMD",
    /// `верховье`: the source and upper part of a stream, including the upper drainage basin
    H_STMH, "H.STMH",
    /// `пресъхващ поток`: 
    H_STMI, "H.STMI",
    /// `del av tilbakevendende bekk`: 
    H_STMIX, "H.STMIX",
    /// `планински поток(и)`: a place where a stream discharges into a lagoon, lake, or the sea
    H_STMM, "H.STMM",
    /// `высохший водоем`: водоем, в котором больше нет воды, но который еще заметный благодаря его топографическим или растительным особенностям
    H_STMQ, "H.STMQ",
    /// `vattendrag`: небольшие мелкие водотоки с постоянным или временным течением
    H_STMS, "H.STMS",
    /// `изгубващи се реки`: река,  полностью высохшая или превратившаяся в подземный канал
    H_STMSB, "H.STMSB",
    /// `част от поток`: 
    H_STMX, "H.STMX",
    /// `sund`: узкое водное пространство, разъединяющее участки суши и соединяющее водоёмы (моря, озёра и т.п.) или их части
    H_STRT, "H.STRT",
    /// `sump`: a wetland dominated by tree vegetation
    H_SWMP, "H.SWMP",
    /// `bevattningssystem`: a network of ditches and one or more of the following elements: water supply, reservoir, canal, pump, well, drain, etc.
    H_SYSI, "H.SYSI",
    /// `судоходный тоннель`: a tunnel through which a canal passes
    H_TNLC, "H.TNLC",
    /// `flod`: a valley or ravine, bounded by relatively steep banks, which in the rainy season becomes a watercourse; found primarily in North Africa and the Middle East
    H_WAD, "H.WAD",
    /// `wadi bend`: a conspicuously curved or bent segment of a wadi
    H_WADB, "H.WADB",
    /// `възел на вади`: a place where two or more wadies join
    H_WADJ, "H.WADJ",
    /// `гърло на вади`: место впадения вади (в море, озеро и т.п.), конечный участок нижнего течения вади
    H_WADM, "H.WADM",
    /// `floder`: valleys or ravines, bounded by relatively steep banks, which in the rainy season become watercourses; found primarily in North Africa and the Middle East
    H_WADS, "H.WADS",
    /// `section of wadi`: 
    H_WADX, "H.WADX",
    /// `водовъртеж`: a turbulent, rotating movement of water in a stream
    H_WHRL, "H.WHRL",
    /// `brønn`: a cylindrical hole, pit, or tunnel drilled or dug down to a depth from which water, oil, or gas can be pumped or brought to the surface
    H_WLL, "H.WLL",
    /// `övergiven källa`: 
    H_WLLQ, "H.WLLQ",
    /// `brønner`: глубокие ямы с отвесными стенками, обычно защищённые от обвалов срубом, служащие для добывания воды из водоносных слоёв земли
    H_WLLS, "H.WLLS",
    /// `заболоченная местность`: an area subject to inundation, usually characterized by bog, marsh, or swamp vegetation
    H_WTLD, "H.WTLD",
    /// `periodisk våtmark`: 
    H_WTLDI, "H.WTLDI",
    /// `watercourse`: стремительно текущая река или ручей
    H_WTRC, "H.WTRC",
    /// `vattenhål`: a natural hole, hollow, or small depression that contains water, used by man and animals, especially in arid areas
    H_WTRH, "H.WTRH",
    /// ``: parks,area, ...
    L, "L",
    /// `agricultural colony`: участок земли, отведенный для сельскохозяйственных поселений
    L_AGRC, "L.AGRC",
    /// `nöjespark`: парк развлечений, то же, что и ярмарка, только с постоянным месторасположением
    L_AMUS, "L.AMUS",
    /// `area`: местность, земля, край; часть какой-либо территории (страны, государства, материка, земной суши и т.п.), выделяемая при районировании по определённому существенному признаку
    L_AREA, "L.AREA",
    /// `dreneringsbasseng`: поверхность, с которой речная система, море или озеро собирают воды
    L_BSND, "L.BSND",
    /// `råoljebassäng`: область с богатой нефтью структурной впадиной
    L_BSNP, "L.BSNP",
    /// `полесражение`: район боевых действий; имеет важное историческое значение
    L_BTL, "L.BTL",
    /// `uthuggning, röjt område`: открытое пространство в парке или в лесу
    L_CLG, "L.CLG",
    /// `обществена собственост`: a park or pasture for community use
    L_CMN, "L.CMN",
    /// `лицензионная территория`: a lease of land by a government for economic development, e.g., mining, forestry
    L_CNS, "L.CNS",
    /// `kolfält`: a region in which coal deposits of possible economic value occur
    L_COLF, "L.COLF",
    /// `continent`: континент: Евразия, Африка, Северная Америка, Южная Америка, Антарктида, Австралия, Океания
    L_CONT, "L.CONT",
    /// `морской берег`: полоса суши, примыкающая к водной поверхности моря или океана и взаимодействующая с ним
    L_CST, "L.CST",
    /// `бизнес центр`: деловая часть города
    L_CTRB, "L.CTRB",
    /// `boligområde`: часть застроенной территории населенного пункта, предполагающая целостность архитектурного замысла
    L_DEVH, "L.DEVH",
    /// `поле(та)`: an open as opposed to wooded area
    L_FLD, "L.FLD",
    /// `overrislet åker`: участки земли, подготовленные для естественной биологической очистки сточных вод и выращивания с.-х. растений
    L_FLDI, "L.FLDI",
    /// `gasfield`: совокупность залежей природного газа на определенной территории
    L_GASF, "L.GASF",
    /// `betesområde`: участок травянистой растительности, используемый и поддерживаемый для выпаса домашних животных
    L_GRAZ, "L.GRAZ",
    /// `gravel area`: покрытая гравием территория
    L_GVL, "L.GVL",
    /// `индустриална зона`: an area characterized by industrial activity
    L_INDS, "L.INDS",
    /// `arktiskt land`: a tract of land in the Arctic
    L_LAND, "L.LAND",
    /// `locality`: a minor area or place of unspecified or mixed character and indefinite boundaries
    L_LCTY, "L.LCTY",
    /// `военная база`: a place used by an army or other armed service for storing arms and supplies, and for accommodating and training troops, a base from which operations can be initiated
    L_MILB, "L.MILB",
    /// `mining area`: an area of mine sites where minerals and ores are extracted
    L_MNA, "L.MNA",
    /// `maneuver area`: a tract of land where military field exercises are carried out
    L_MVA, "L.MVA",
    /// `военно-морская база`: an area used to store supplies, provide barracks for troops and naval personnel, a port for naval vessels, and from which operations are initiated
    L_NVB, "L.NVB",
    /// `oase`: участок пустыни или полупустыни с обильным естественным или искусственным увлажнением, обусловленным близостью рек и грунтовых вод, и с богатой растительностью
    L_OAS, "L.OAS",
    /// `нефтяное месторождение`: an area containing a subterranean store of petroleum of economic value
    L_OILF, "L.OILF",
    /// `торфено находище`: место, где добывают торф
    L_PEAT, "L.PEAT",
    /// `park`: an area, often of forested land, maintained as a place of beauty, or for recreation
    L_PRK, "L.PRK",
    /// `hamn`: a place provided with terminal and transfer facilities for loading and discharging waterborne cargo or passengers, usually located in a harbor
    L_PRT, "L.PRT",
    /// `kvicksand`: an area where loose sand with water moving through it may become unstable when heavy objects are placed at the surface, causing them to sink
    L_QCKS, "L.QCKS",
    /// `republic`: 
    L_REP, "L.REP",
    /// `заповедник`: a tract of public land reserved for future use or restricted as to use
    L_RES, "L.RES",
    /// `område regulert til landbruk`: участок земли, предназначенный для агромелиорации
    L_RESA, "L.RESA",
    /// `горски резерват`: особо охраняемый участок леса, использование которого контролируется
    L_RESF, "L.RESF",
    /// `заповедно-охотничье хозяйство`: участок территории, выделенный для интенсивного воспроизводства дичи и проведения строго регулируемых охот
    L_RESH, "L.RESH",
    /// `naturreservat`: an area reserved for the maintenance of a natural habitat
    L_RESN, "L.RESN",
    /// `palmereservat`: место, где растут пальмы, использование которых контролируется
    L_RESP, "L.RESP",
    /// `reservat`: a tract of land set aside for aboriginal, tribal, or native populations
    L_RESV, "L.RESV",
    /// `wildlife reserve`: заповедник, территория, где природные богатства (животные и растения) находятся под особой охраной правительства
    L_RESW, "L.RESW",
    /// `регион`: an area distinguished by one or more observable physical or cultural characteristics
    L_RGN, "L.RGN",
    /// `økonomisk region`: a region of a country established for economic development or for statistical purposes
    L_RGNE, "L.RGNE",
    /// `historical region`: 
    L_RGNH, "L.RGNH",
    /// `озерный край`: место скопления большого количества озер
    L_RGNL, "L.RGNL",
    /// `артиллерийский полигон`: a tract of land used for artillery firing practice
    L_RNGA, "L.RNGA",
    /// `salt area`: мелководье или равнинная местность, где после периодических наводнений собирается соль
    L_SALT, "L.SALT",
    /// `snødekt område`: место скопления многолетнего снега и льда
    L_SNOW, "L.SNOW",
    /// `stamområde`: участок земли, являющийся в течение некоторого времени местом жительства для кочующих племен
    L_TRB, "L.TRB",
    /// `master source holdings list`: 
    L_ZZZZZ, "L.ZZZZZ",
    /// `Map`: A Web page displaying a map
    Map, "Map",
    /// ``: 
    Marc_Wick, "Marc-Wick",
    /// ``: city, village,...
    P, "P",
    /// `populated place`: a city, town, village, or other agglomeration of buildings where people live and work
    P_PPL, "P.PPL",
    /// `säte för en första ordningens administrativ enhet`: seat of a first-order administrative division (PPLC takes precedence over PPLA)
    P_PPLA, "P.PPLA",
    /// `центр административного деления второго порядка`: 
    P_PPLA2, "P.PPLA2",
    /// `центр административного деления третьего порядка`: 
    P_PPLA3, "P.PPLA3",
    /// `центр административного деления четвертого порядка`: 
    P_PPLA4, "P.PPLA4",
    /// `столица на политическо образувание`: 
    P_PPLC, "P.PPLC",
    /// `деревня`: a populated place where the population is largely engaged in agricultural activities
    P_PPLF, "P.PPLF",
    /// `säte av ledningen för en politisk enhet`: 
    P_PPLG, "P.PPLG",
    /// `befolkad ort`: an area similar to a locality but with a small group of dwellings or other buildings
    P_PPLL, "P.PPLL",
    /// `avfolket sted`: 
    P_PPLQ, "P.PPLQ",
    /// `религиозная община`: a populated place whose population is largely engaged in religious occupations
    P_PPLR, "P.PPLR",
    /// `населенные пункты`: cities, towns, villages, or other agglomerations of buildings where people live and work
    P_PPLS, "P.PPLS",
    /// `разрушенный населенный пункт`: населенный пункт (деревня, город), пострадавший в результате природной катастрофы или войны
    P_PPLW, "P.PPLW",
    /// `del av befolkad plats`: 
    P_PPLX, "P.PPLX",
    /// `еврейские поселения`: 
    P_STLMT, "P.STLMT",
    /// ``: road, railroad, ...
    R, "R",
    /// `veifylling`: a raised roadway across wet ground or shallow water
    R_CSWY, "R.CSWY",
    /// `former causeway`: a causeway no longer used for transportation
    R_CSWYQ, "R.CSWYQ",
    /// `oljepipeline`: a pipeline used for transporting oil
    R_OILP, "R.OILP",
    /// `promenade`: a place for public walking, usually along a beach front
    R_PRMN, "R.PRMN",
    /// `волок`: перевал в верховьях рек различных бассейнов
    R_PTGE, "R.PTGE",
    /// `road`: an open way with improved surface for transportation of animals, people and vehicles
    R_RD, "R.RD",
    /// `ancient road`: развалины, на месте которых была дорога во времена античности
    R_RDA, "R.RDA",
    /// `завой на път`: изогнутый участок дороги
    R_RDB, "R.RDB",
    /// `выемка грунта под дорогу`: an excavation cut through a hill or ridge for a road
    R_RDCUT, "R.RDCUT",
    /// `перекресток`: a place where two or more roads join
    R_RDJCT, "R.RDJCT",
    /// `jernbanekryss`: a place where two or more railroad tracks join
    R_RJCT, "R.RJCT",
    /// `järnväg`: a permanent twin steel-rail track on which freight and passenger cars move long distances
    R_RR, "R.RR",
    /// `заброшенная железная дорога`: 
    R_RRQ, "R.RRQ",
    /// `karavanväg`: маршрут следования караванов
    R_RTE, "R.RTE",
    /// `railroad yard`: техническая железнодорожная станция для расформирования и формирования различных категорий поездов, выполнение техническое обслуживание и коммерческий осмотр составов поездов и устранение выявленных неисправностей вагонов, смена локомотивов и локомотивных бригад
    R_RYD, "R.RYD",
    /// `väg`: пространство между двумя рядами домов в населенных пунктах для проезда и прохода
    R_ST, "R.ST",
    /// `rute for krøtterdrift`: маршрут перегона крупного рогатого домашнего скота
    R_STKR, "R.STKR",
    /// `туннель`: подземное сооружение, служащее для транспортных целей
    R_TNL, "R.TNL",
    /// `naturlig tunnel`: a cave that is open at both ends
    R_TNLN, "R.TNLN",
    /// `road tunnel`: тоннель, через который проходит дорога
    R_TNLRD, "R.TNLRD",
    /// `жп тунел`: a tunnel through which a railroad passes
    R_TNLRR, "R.TNLRR",
    /// `тунели`: подземные  сооружение, служащие для транспортных целей
    R_TNLS, "R.TNLS",
    /// `sti`: узкая пешеходная дорожка, протоптанная людьми или животными в лесу, в поле, по снегу и т. п.
    R_TRL, "R.TRL",
    /// `RDF Data`: A Document containing RDF description of one or several features.
    RDFData, "RDFData",
    /// ``: spot, building, farm, ...
    S, "S",
    /// `administrativ inrättning`: a government building
    S_ADMF, "S.ADMF",
    /// `agricultural facility`: здание или участок земли, используемый для повышения качества сельскохозяйственной продукции
    S_AGRF, "S.AGRF",
    /// `flygbas`: an area used to store supplies, provide barracks for air force personnel, hangars and runways for aircraft, and from which operations are initiated
    S_AIRB, "S.AIRB",
    /// `flygfält`: комплекс сооружений, оборудования и земельный участок с воздушным пространством, предназначенный для взлёта, посадки, размещения и обслуживания самолётов
    S_AIRF, "S.AIRF",
    /// `landningsplats för helikopter`: земельный участок, площадка на здании, судне и т.п., оборудованные для взлёта, посадки и обслуживания вертолётов
    S_AIRH, "S.AIRH",
    /// `airport`: комплекс сооружений (с аэродромом и аэровокзалом) для осуществления регулярных полётов транспортной авиации и обслуживания пассажиров
    S_AIRP, "S.AIRP",
    /// `заброшенный аэродром`: 
    S_AIRQ, "S.AIRQ",
    /// `amfiteater`: an oval or circular structure with rising tiers of seats about a stage or open space
    S_AMTH, "S.AMTH",
    /// `ancient site`: a place where archeological remains, old structures, or cultural artifacts are located
    S_ANS, "S.ANS",
    /// `aquaculture facility`: facility or area for the cultivation of aquatic animals and plants, especially fish, shellfish, and seaweed, in natural or controlled marine or freshwater environments; underwater agriculture
    S_AQC, "S.AQC",
    /// `arch`: a natural or man-made structure in the form of an arch
    S_ARCH, "S.ARCH",
    /// `astronomisk station`: научное учреждение, ведущее астрономические и геофизические наблюдения и исследования; здание, специально оборудованное для таких наблюдений
    S_ASTR, "S.ASTR",
    /// `asyl`: место, где можно укрыться, скрыться от чего-н.
    S_ASYL, "S.ASYL",
    /// `стадион`: комплексное спортивное сооружение со специально оборудованными площадками для состязаний, тренировок, вспомогательными помещениями и трибунами для зрителей
    S_ATHF, "S.ATHF",
    /// `bankomat`: An unattended electronic machine in a public place, connected to a data system and related equipment and activated by a bank customer to obtain cash withdrawals and other banking services.
    S_ATM, "S.ATM",
    /// `bank`: A business establishment in which money is kept for saving or commercial purposes or is invested, supplied for loans, or exchanged.
    S_BANK, "S.BANK",
    /// `маяк`: a fixed artificial navigation mark
    S_BCN, "S.BCN",
    /// `bro`: сооружение для перехода, переезда через реку, овраг, железнодорожный путь и т.п.
    S_BDG, "S.BDG",
    /// `ruined bridge`: a destroyed or decayed bridge which is no longer functional
    S_BDGQ, "S.BDGQ",
    /// `byggnad`: архитектурное сооружение, постройка (обычно больших размеров), например, дом, завод и т.п.
    S_BLDG, "S.BLDG",
    /// `office building`: commercial building where business and/or services are conducted
    S_BLDO, "S.BLDO",
    /// `boundary marker`: ограничительный знак, используемый для обозначения границ
    S_BP, "S.BP",
    /// `brakker`: a building for lodging military personnel
    S_BRKS, "S.BRKS",
    /// `вълнолом`: гидротехническое сооружение для защиты места стоянки судов или берега от разрушительного действия волн
    S_BRKW, "S.BRKW",
    /// `прессовальная станция`: место, где прессуют сено в тюки
    S_BSTN, "S.BSTN",
    /// `лодкостоянка`: расположенное на берегу моря, реки и т.п.  сооружение, которое занимается обслуживанием, ремонтом и изготовлением лодок
    S_BTYD, "S.BTYD",
    /// `burial cave(s)`: a cave used for human burials
    S_BUR, "S.BUR",
    /// `bus station`: a facility comprising ticket office, platforms, etc. for loading and unloading passengers
    S_BUSTN, "S.BUSTN",
    /// `Busshållplats`: пункт, место, установленные для посадки и высадки пассажиров
    S_BUSTP, "S.BUSTP",
    /// `stenkummel`: a heap of stones erected as a landmark or for other purposes
    S_CARN, "S.CARN",
    /// `grotte`: углубление, полое пространство, образовавшееся в земной коре, горных массивах естественным путём или созданные человеком, с выходом наружу
    S_CAVE, "S.CAVE",
    /// `Centre Continuous Learning`: Centres for Continuous Learning
    S_CCL, "S.CCL",
    /// `kirke`: a building for public Christian worship
    S_CH, "S.CH",
    /// `camp(s)`: a site occupied by tents, huts, or other shelters for temporary use
    S_CMP, "S.CMP",
    /// `tømmerkoie`: временное поселение лесозаготовителей
    S_CMPL, "S.CMPL",
    /// `labor camp`: a camp used by migrant or temporary laborers
    S_CMPLA, "S.CMPLA",
    /// `миньорски лагер`: a camp used by miners
    S_CMPMN, "S.CMPMN",
    /// `лагер на нефторъботници`: поселок, в котором живут работники нефтяной промышленности
    S_CMPO, "S.CMPO",
    /// `abandoned camp`: 
    S_CMPQ, "S.CMPQ",
    /// `refugee camp`: лагерь, в котором живут беженцы (люди, покинувшие место своего жительства, спасаясь от бедствия (землетрясения, войны, голода и т.п.)
    S_CMPRF, "S.CMPRF",
    /// `гробище`: a burial place or ground
    S_CMTY, "S.CMTY",
    /// `узел связи`: совокупность технических средств оператора связи, обеспечивающих оказание услуг связи и присоединение к сети общего пользования
    S_COMC, "S.COMC",
    /// `innhegning`: загон для скота
    S_CRRL, "S.CRRL",
    /// `kasino`: a building used for entertainment, especially gambling
    S_CSNO, "S.CSNO",
    /// `slott`: a large fortified building or set of buildings
    S_CSTL, "S.CSTL",
    /// `tullhus`: помещение, где размещается учреждение, ведающее контролем провоза через границу грузов, багажа, почты и собирающее пошлину за такой провоз
    S_CSTM, "S.CSTM",
    /// `courthouse`: здание, в котором проходят заседания суда
    S_CTHSE, "S.CTHSE",
    /// `атомен център`: научный центр, в котором проводятся атомные исследования
    S_CTRA, "S.CTRA",
    /// `community center`: a facility for community recreation and other activities
    S_CTRCM, "S.CTRCM",
    /// `tjänstecentrum`: a place where more than one facility is situated
    S_CTRF, "S.CTRF",
    /// `медицински център`: комплекс лечебных учреждений; включает в себя: больницу, поликлинику, аптеку, медицинский институт
    S_CTRM, "S.CTRM",
    /// `religiøst senter`: комплекс нескольких религиозных учреждений (монашеская обитель, школа для священнослужителей, церковь)
    S_CTRR, "S.CTRR",
    /// `romsenter`: центр, который осуществляет запуск и наблюдение за полетом искусственных спутников земли и космических кораблей
    S_CTRS, "S.CTRS",
    /// `женски манастир`: обитель, общежитие монахинь
    S_CVNT, "S.CVNT",
    /// `damm`: гидротехническое сооружение, искусственная насыпь для предохранения местности от затопления, для ограждения водохранилищ и т.п.
    S_DAM, "S.DAM",
    /// `разрушенная дамба`: дамба, которая больше не функционирует
    S_DAMQ, "S.DAMQ",
    /// `demning gravd ned til grunnfjellet`: дамба, возведенная на  песчаном дне реки
    S_DAMSB, "S.DAMSB",
    /// `mejeri`: предприятие по производству молочных продуктов
    S_DARY, "S.DARY",
    /// `сухой док`: портовое сооружение для ремонта и постройки судов; сухой док сооружается на защищённой от волнения акватории и представляет собой отделяемую от неё с торца затвором водонепроницаемую камеру
    S_DCKD, "S.DCKD",
    /// `dockyard`: предприятие для постройки и ремонта судов
    S_DCKY, "S.DCKY",
    /// `дига`: неглубокий и неширокий ров, обычно служащий для спуска или отвода воды
    S_DIKE, "S.DIKE",
    /// `diplomatic facility`: office, residence, or facility of a foreign government, which may include an embassy, consulate, chancery, office of charge d?affaires, or other diplomatic, economic, military, or cultural mission
    S_DIP, "S.DIP",
    /// `горивно депо`: an area where fuel is stored
    S_DPOF, "S.DPOF",
    /// `egendom(ar)`: крупное земельное владение, обычно с усадьбой
    S_EST, "S.EST",
    /// `banana plantation`: an estate that specializes in the growing of bananas
    S_ESTB, "S.ESTB",
    /// `cotton plantation`: an estate specializing in the cultivation of cotton
    S_ESTC, "S.ESTC",
    /// `oil palm plantation`: большая земельная площадь, на которой выращивают масличные пальмы
    S_ESTO, "S.ESTO",
    /// `каучукова плантация`: большая земельная площадь, на которой выращивают каучуковые деревья
    S_ESTR, "S.ESTR",
    /// `сахарная плантация`: большая земельная площадь, на которой выращивают сахарный тростник
    S_ESTSG, "S.ESTSG",
    /// `sisal plantation`: an estate that specializes in growing sisal
    S_ESTSL, "S.ESTSL",
    /// `teplantage`: большая земельная площадь, на которой выращивают чай
    S_ESTT, "S.ESTT",
    /// `del av egendom`: 
    S_ESTX, "S.ESTX",
    /// `учреждение`: a building or buildings housing a center, institute, foundation, hospital, prison, mission, courthouse, etc.
    S_FCL, "S.FCL",
    /// `foundry`: мастерская, в которой происходит обработка металла путем плавления и заливки в литейные формы
    S_FNDY, "S.FNDY",
    /// `farm`: частное сельскохозяйственное предприятие, принадлежащее фермеру и ведущееся им на собственной или арендованной земле
    S_FRM, "S.FRM",
    /// `abandoned farm`: 
    S_FRMQ, "S.FRMQ",
    /// `фермы`: частные сельскохозяйственные предприятия, принадлежащие фермеру и ведущиеся им на собственной или арендованной земле
    S_FRMS, "S.FRMS",
    /// `farmstead`: the buildings and adjacent service areas of a farm
    S_FRMT, "S.FRMT",
    /// `крепость`: a defensive structure or earthworks
    S_FT, "S.FT",
    /// `ферибот`: a boat or other floating conveyance and terminal facilities regularly used to transport people and vehicles across a waterbody
    S_FY, "S.FY",
    /// `бариера`: конструкция для регулирования входа и выхода
    S_GATE, "S.GATE",
    /// `градина(и)`: an enclosure for displaying selected plant or animal life
    S_GDN, "S.GDN",
    /// `ghat`: a set of steps leading to a river, which are of religious significance, and at their base is usually a platform for bathing
    S_GHAT, "S.GHAT",
    /// `гостиница`: дом с меблированными комнатами («номерами») для временного проживания путешественников
    S_GHSE, "S.GHSE",
    /// `нефтегазовый сепаратор`: аппарат для отделения газа от нефти
    S_GOSP, "S.GOSP",
    /// `local government office`: a facility housing local governmental offices, usually a city, town, or village hall
    S_GOVL, "S.GOVL",
    /// `grav`: место захоронения
    S_GRVE, "S.GRVE",
    /// `eremitage`: жилище монахов-отшельников
    S_HERM, "S.HERM",
    /// `stoppested`: a place where caravans stop for rest
    S_HLT, "S.HLT",
    /// `homestead`: a residence, owner's or manager's, on a sheep or cattle station, woolshed, outcamp, or Aboriginal outstation, specific to Australia and New Zealand
    S_HMSD, "S.HMSD",
    /// `house(s)`: a building used as a human habitation
    S_HSE, "S.HSE",
    /// `landsted`: a large house, mansion, or chateau, on a large estate
    S_HSEC, "S.HSEC",
    /// `hospital`: a building in which sick or injured, especially those confined to bed, are medically treated
    S_HSP, "S.HSP",
    /// `klinikk`: медицинское учреждение, осуществляющее внебольничное лечебно-профилактическое обслуживание населения
    S_HSPC, "S.HSPC",
    /// `apotek`: медицинское учреждение, занимающееся лечением и предупреждением болезней путём систематического врачебного наблюдения
    S_HSPD, "S.HSPD",
    /// `leprosarium`: an asylum or hospital for lepers
    S_HSPL, "S.HSPL",
    /// `историческо място`: a place of historical importance
    S_HSTS, "S.HSTS",
    /// `hotell`: то же, что и гостиница; дом для временного проживания туристов
    S_HTL, "S.HTL",
    /// `hut`: a small primitive house
    S_HUT, "S.HUT",
    /// `hytter`: небольшие примитивные домики
    S_HUTS, "S.HUTS",
    /// `military installation`: a facility for use of and control by armed forces
    S_INSM, "S.INSM",
    /// `research institute`: a facility where research is carried out
    S_ITTR, "S.ITTR",
    /// `molo`: a structure built out into the water at a river mouth or harbor entrance to regulate currents and silting
    S_JTY, "S.JTY",
    /// `стоварище`: a place where boats receive or discharge passengers and freight, but lacking most port facilities
    S_LDNG, "S.LDNG",
    /// `leper colony`: лечебное учреждение для больных проказой; место проживания и лечения таких больных
    S_LEPC, "S.LEPC",
    /// `bibliotek`: A place in which information resources such as books are kept for reading, reference, or lending.
    S_LIBR, "S.LIBR",
    /// `landfill`: a place for trash and garbage disposal in which the waste is buried between layers of earth to build up low-lying land
    S_LNDF, "S.LNDF",
    /// `sluss(ar)`: гидротехническое сооружение между водоёмами с различными уровнями воды, позволяющее переводить суда из одного водоёма в другой
    S_LOCK, "S.LOCK",
    /// `fyrtårn`: сооружение башенного типа, служащее ориентиром для опознавания берегов, определения места судна и предупреждения о навигационной опасности
    S_LTHSE, "S.LTHSE",
    /// `köpcentrum`: A large, often enclosed shopping complex containing various stores, businesses, and restaurants usually accessible by common passageways.
    S_MALL, "S.MALL",
    /// `småbåthavn`: a harbor facility for small boats, yachts, etc.
    S_MAR, "S.MAR",
    /// `фабрика`: промышленное или крупное промысловое предприятие, или комплекс предприятий
    S_MFG, "S.MFG",
    /// `bryggeri`: предприятие по производству пива
    S_MFGB, "S.MFGB",
    /// `консервный завод`: a building where food items are canned
    S_MFGC, "S.MFGC",
    /// `kopparbruk`: a facility for processing copper ore
    S_MFGCU, "S.MFGCU",
    /// `известеобжигательная печь`: a furnace in which limestone is reduced to lime
    S_MFGLM, "S.MFGLM",
    /// `munitions plant`: предприятие по производству оружия
    S_MFGM, "S.MFGM",
    /// `phosphate works`: a facility for producing fertilizer
    S_MFGPH, "S.MFGPH",
    /// `abandoned factory`: 
    S_MFGQ, "S.MFGQ",
    /// `sukkerraffineri`: a facility for converting raw sugar into refined sugar
    S_MFGSG, "S.MFGSG",
    /// `market`: место торговли, на котором встречаются продавцы и покупатели
    S_MKT, "S.MKT",
    /// `фабрика`: промышленное предприятие, обрабатывающее сырье машинным способом
    S_ML, "S.ML",
    /// `завод по переработке руды`: a facility for improving the metal content of ore by concentration
    S_MLM, "S.MLM",
    /// `завод по производству оливкового масла`: предприятие, которое занимается переработкой оливок и получением из них оливкового масла.
    S_MLO, "S.MLO",
    /// `sukkermølle`: предприятие, которое занимается переработкой сахарного тростника в нерафинированный сахар
    S_MLSG, "S.MLSG",
    /// `бивша захарна фабрика`: сахарный завод, прекративший свою работу
    S_MLSGQ, "S.MLSGQ",
    /// `дъскорезница`: изготовление пиломатериалов (брусьев и досок) из брёвен
    S_MLSW, "S.MLSW",
    /// `vindmølle`: аэродинамический механизм, который выполняет механическую работу за счет энергии ветра, улавливаемой крыльями мельницы. Наиболее известным применением ветряных мельниц является их использование для помола муки
    S_MLWND, "S.MLWND",
    /// `water mill`: a mill powered by running water
    S_MLWTR, "S.MLWTR",
    /// `шахта`: горнопромышленное предприятие, ведущее добычу полезных ископаемых подземным способом; место проведения подземных работ
    S_MN, "S.MN",
    /// `gold mine(s)`: предприятие, осуществляющее добычу золота или золотой руды
    S_MNAU, "S.MNAU",
    /// `coal mine(s)`: горнопромышленное предприятие, ведущее добычу угля подземным способом
    S_MNC, "S.MNC",
    /// `шахта по добыче хрома`: a mine where chrome ore is extracted
    S_MNCR, "S.MNCR",
    /// `copper mine(s)`: a mine where copper ore is extracted
    S_MNCU, "S.MNCU",
    /// `diatomite mine(s)`: a place where diatomaceous earth is extracted
    S_MNDT, "S.MNDT",
    /// `iron mine(s)`: a mine where iron ore is extracted
    S_MNFE, "S.MNFE",
    /// `monument`: a commemorative structure or statue
    S_MNMT, "S.MNMT",
    /// `saltgruva(or)`: горнопромышленное предприятие, осуществляющее добычу солей
    S_MNN, "S.MNN",
    /// `nickel mine(s)`: a mine where nickel ore is extracted
    S_MNNI, "S.MNNI",
    /// `lead mine(s)`: a mine where lead ore is extracted
    S_MNPB, "S.MNPB",
    /// `placer mine(s)`: a place where heavy metals are concentrated and running water is used to extract them from unconsolidated sediments
    S_MNPL, "S.MNPL",
    /// `изоставена мина`: 
    S_MNQ, "S.MNQ",
    /// `quarry(-ies)`: a surface mine where building stone or gravel and sand, etc. are extracted
    S_MNQR, "S.MNQR",
    /// `tin mine(s)`: a mine where tin ore is extracted
    S_MNSN, "S.MNSN",
    /// `mole`: гидротехническое сооружение для защиты места стоянки судов или берега от разрушительного действия волн
    S_MOLE, "S.MOLE",
    /// `moské`: у мусульман: молитвенный дом
    S_MSQE, "S.MSQE",
    /// `резиденция миссионеров`: a place characterized by dwellings, school, church, hospital and other facilities operated by a religious group for the purpose of providing charitable services and to propagate religion
    S_MSSN, "S.MSSN",
    /// `nedlagt misjonsstasjon`: 
    S_MSSNQ, "S.MSSNQ",
    /// `monastery`: a building and grounds where a community of monks lives in seclusion
    S_MSTY, "S.MSTY",
    /// `tunnelbanestation`: metro station (Underground, Tube, or Métro)
    S_MTRO, "S.MTRO",
    /// `музей`: a building where objects of permanent interest in one or more of the arts and sciences are preserved and exhibited
    S_MUS, "S.MUS",
    /// `novitiate`: религиозный дом, где учатся послушники, готовящиеся стать монахами
    S_NOV, "S.NOV",
    /// `barnehage`: тёплое с покрытием из прозрачного материала помещение для выращивания ранних овощей, цветов, теплолюбивых растений; оранжерея
    S_NSY, "S.NSY",
    /// `наблюдателна точка`: a wildlife or scenic observation point
    S_OBPT, "S.OBPT",
    /// `observatory`: научное учреждение, ведущее астрономические и геофизические наблюдения и исследования; здание, специально оборудованное для таких наблюдений
    S_OBS, "S.OBS",
    /// `radio observatory`: a facility equipped with an array of antennae for receiving radio waves from space
    S_OBSR, "S.OBSR",
    /// `knutpunkt för oljepipeline`: a section of an oil pipeline where two or more pipes join together
    S_OILJ, "S.OILJ",
    /// `forlatt oljebrønn`: 
    S_OILQ, "S.OILQ",
    /// `oil refinery`: предприятие, которое занимается переработкой нефти в бензин, авиационный керосин, мазут, дизельное топливо, смазочные масла, смазки, битумы, нефтяной кокс сырьё для нефтехимии
    S_OILR, "S.OILR",
    /// `нефтехранилище`: a tract of land occupied by large, cylindrical, metal tanks in which oil or liquid petrochemicals are stored
    S_OILT, "S.OILT",
    /// `oil well`: a well from which oil may be pumped
    S_OILW, "S.OILW",
    /// `operahus`: театр; оперная труппа театра
    S_OPRA, "S.OPRA",
    /// `palats`: a large stately house, often a royal or presidential residence
    S_PAL, "S.PAL",
    /// `pagod`: a tower-like storied structure, usually a Buddhist shrine
    S_PGDA, "S.PGDA",
    /// `pir, kaj, vågbrytare`: a structure built out into navigable water on piles providing berthing for ships and recreation
    S_PIER, "S.PIER",
    /// `място за паркиране`: место для парковки автотранспорта
    S_PKLT, "S.PKLT",
    /// `pumpstation för olja`: станция, предназначенная для перекачки нефти через трубопроводы
    S_PMPO, "S.PMPO",
    /// `water pumping station`: специальное здание для насосов, перекачивающих воду из водоёма к местам её использования
    S_PMPW, "S.PMPW",
    /// `postkontor`: a public building in which mail is received, sorted and distributed
    S_PO, "S.PO",
    /// `police post`: a building in which police are stationed
    S_PP, "S.PP",
    /// `övergiven polispostering`: 
    S_PPQ, "S.PPQ",
    /// `parkingång`: a controlled access to a park
    S_PRKGT, "S.PRKGT",
    /// `hovedkvarter på militæranlegg`: a park administrative facility
    S_PRKHQ, "S.PRKHQ",
    /// `fengsel`: место заключения, здание, где содержатся лица, приговорённые судом к лишению свободы; лица, находящиеся под следствием
    S_PRN, "S.PRN",
    /// `forbedringsanstalt`: исправительно-трудовое учреждение, в котором отбывают наказание лица до 18 лет, осуждённые к лишению свободы
    S_PRNJ, "S.PRNJ",
    /// `nedlagt fengsel`: 
    S_PRNQ, "S.PRNQ",
    /// `электростанция`: a facility for generating electric power
    S_PS, "S.PS",
    /// `vannkraftverk`: a building where electricity is generated from water power
    S_PSH, "S.PSH",
    /// `gränspostering`: a post or station at an international boundary for the regulation of movement of people and goods
    S_PSTB, "S.PSTB",
    /// `customs post`: a building at an international boundary where customs and duties are paid on goods
    S_PSTC, "S.PSTC",
    /// `patrol post`: a post from which patrols are sent out
    S_PSTP, "S.PSTP",
    /// `пирамида`: an ancient massive structure of square ground plan with four triangular faces meeting at a point and used for enclosing tombs
    S_PYR, "S.PYR",
    /// `пирамиды`: В Древнем Египте: массивные сооружения из каменных блоков с четырёхугольным основанием и сходящимися к вершине боковыми гранями, служившие гробницей фараона
    S_PYRS, "S.PYRS",
    /// `причал`: a structure of solid construction along a shore or bank which provides berthing for ships and which generally provides cargo handling facilities
    S_QUAY, "S.QUAY",
    /// `traffic circle`: a road junction formed around a central circle about which traffic moves in one direction only
    S_RDCR, "S.RDCR",
    /// `golfbana`: a recreation field where golf is played
    S_RECG, "S.RECG",
    /// `veddeløpsbane`: a track where races are held
    S_RECR, "S.RECR",
    /// `ресторан`: A place where meals are served to the public
    S_REST, "S.REST",
    /// `store`: a building where goods and/or services are offered for sale
    S_RET, "S.RET",
    /// `дом отдыха`: лечебно-профилактическое учреждение для лечения и отдыха
    S_RHSE, "S.RHSE",
    /// `fuglekoloni`: место, где лежат стадами некоторые морские животные
    S_RKRY, "S.RKRY",
    /// `религиозно място`: место религиозного поклонения
    S_RLG, "S.RLG",
    /// `retreat`: a place of temporary seclusion, especially for religious groups
    S_RLGR, "S.RLGR",
    /// `ranch`: крупная ферма, основной специализацией которой является разведение крупного рогатого скота
    S_RNCH, "S.RNCH",
    /// `запасный железнодорожный путь`: a short track parallel to and joining the main track
    S_RSD, "S.RSD",
    /// `järnvägssignal`: сигнал для передачи приказов и указаний, обеспечивающих безопасность движения поездов, маневровых работ
    S_RSGNL, "S.RSGNL",
    /// `курорт`: местность с целебными природными свойствами, используемая для лечебных целей и для отдыха
    S_RSRT, "S.RSRT",
    /// `railroad station`: здание или комплекс зданий для обслуживания пассажиров и размещения служб на железнодорожных станциях
    S_RSTN, "S.RSTN",
    /// `övergiven järnvägsstation`: 
    S_RSTNQ, "S.RSTNQ",
    /// `railroad stop`: пункт, место, установленные для посадки и высадки пассажиров электропоезда
    S_RSTP, "S.RSTP",
    /// `nedlagt jernbanestoppested`: 
    S_RSTPQ, "S.RSTPQ",
    /// `ruin(er)`: развалины какого-л. строения, сооружения
    S_RUIN, "S.RUIN",
    /// `school`: учебное заведение, которое осуществляет общее образование и воспитание молодого поколения; здание, в котором помещается это заведение
    S_SCH, "S.SCH",
    /// `земеделско училище`: школа, в учебном плане которой сделан акцент на изучение сельскохозяйственных наук
    S_SCHA, "S.SCHA",
    /// `college`: the grounds and buildings of an institution of higher learning
    S_SCHC, "S.SCHC",
    /// `Driving School`: Driving School
    S_SCHD, "S.SCHD",
    /// `language school`: языковые учебные заведения
    S_SCHL, "S.SCHL",
    /// `military school`: a school at which military science forms the core of the curriculum
    S_SCHM, "S.SCHM",
    /// `военно-морское училище`: a school at which maritime sciences form the core of the curriculum
    S_SCHN, "S.SCHN",
    /// `technical school`: школа, предоставляющая профессионально-техническое образование
    S_SCHT, "S.SCHT",
    /// `центр подготовки к государственному экзамену`: state exam preparation centres
    S_SECP, "S.SECP",
    /// `кошара за овце`: хлев, загон для овец
    S_SHPF, "S.SHPF",
    /// `helligdom`: a structure or place memorializing a person or religious concept
    S_SHRN, "S.SHRN",
    /// `хамбар`: a building for storing goods, especially provisions
    S_SHSE, "S.SHSE",
    /// `шлюз`: гидротехническое сооружение между водоёмами с различными уровнями воды, позволяющее переводить суда из одного водоёма в другой
    S_SLCE, "S.SLCE",
    /// `санаторий`: a facility where victims of physical or mental disorders are treated
    S_SNTR, "S.SNTR",
    /// `spa`: курорт с минеральным источником
    S_SPA, "S.SPA",
    /// `spillway`: a passage or outlet through which surplus water flows over, around or through a dam
    S_SPLY, "S.SPLY",
    /// `plass`: a broad, open, public area near the center of a town or city
    S_SQR, "S.SQR",
    /// `stable`: a building for the shelter and feeding of farm animals, especially horses
    S_STBL, "S.STBL",
    /// `stadium`: комплексное спортивное сооружение со специально оборудованными площадками для состязаний, тренировок, вспомогательными помещениями и трибунами для зрителей
    S_STDM, "S.STDM",
    /// `forskningsbase`: помещение, которое используется в качестве базы для проведения различных научных исследований
    S_STNB, "S.STNB",
    /// `coast guard station`: пункт, с которого вооруженные корабли ведут охрану береговой линии
    S_STNC, "S.STNC",
    /// `експериментална станция`: место проведения различного рода испытаний
    S_STNE, "S.STNE",
    /// `skogsstation`: a collection of buildings and facilities for carrying out forest management
    S_STNF, "S.STNF",
    /// `inspeksjonsstasjon`: a station at which vehicles, goods, and people are inspected
    S_STNI, "S.STNI",
    /// `метеорологическая станция`: учреждение, которое проводит регулярные наблюдения за состоянием атмосферы
    S_STNM, "S.STNM",
    /// `radiostasjon`: a facility for producing and transmitting information by radio waves
    S_STNR, "S.STNR",
    /// `сателитна станция`: космический аппарат, предназначенный для долговременного пребывания людей на околоземной орбите с целью проведения научных исследований в условиях космического пространства, разведки, наблюдений за поверхностью и атмосферой планеты, астрономических наблюдений, и т.п.
    S_STNS, "S.STNS",
    /// `hvalfangststasjon`: a facility for butchering whales and processing train oil
    S_STNW, "S.STNW",
    /// `trappa`: stones or slabs placed for ease in ascending or descending a steep slope
    S_STPS, "S.STPS",
    /// `sewage treatment plant`: facility for the processing of sewage and/or wastewater
    S_SWT, "S.SWT",
    /// `teater`: A building, room, or outdoor structure for the presentation of plays, films, or other dramatic performances
    S_THTR, "S.THTR",
    /// `гробница(и)`: a structure for interring bodies
    S_TMB, "S.TMB",
    /// `temple(s)`: здание, предназначенное для совершения богослужений и религиозных обрядов
    S_TMPL, "S.TMPL",
    /// `cattle dipping tank`: искусственный водоем со специальным химическим раствором; в этот водоем опускают крупный рогатый скот для профилактики от опасных заболеваний
    S_TNKD, "S.TNKD",
    /// `torn`: a high conspicuous structure, typically much higher than its diameter
    S_TOWR, "S.TOWR",
    /// `transit terminal`: станция для перегрузки материалов с одного вида транспорта на другой и для пересадки пассажиров
    S_TRANT, "S.TRANT",
    /// `trianguleringsstasjon`: опорный геодезический пункт, созданный методом триангуляции (геодезия)
    S_TRIG, "S.TRIG",
    /// `конечный пункт нефтепровода`: нефтебаза или нефтеналивная станция в конце нефтепровода
    S_TRMO, "S.TRMO",
    /// `temp work office`: Temporary Work Offices
    S_TWO, "S.TWO",
    /// `postgrad & MBA`: Post Universitary Education Institutes (post graduate studies and highly specialised master programs) & MBA
    S_UNIO, "S.UNIO",
    /// `university prep school`: University Preparation Schools & Institutions
    S_UNIP, "S.UNIP",
    /// `university`: An institution for higher learning with teaching and research facilities constituting a graduate school and professional schools that award master's degrees and doctorates and an undergraduate division that awards bachelor's degrees.
    S_UNIV, "S.UNIV",
    /// `amerikansk regeringsbyggnad`: a facility operated by the United States Government in Panama
    S_USGE, "S.USGE",
    /// `veterinary facility`: больница, где лечат животных
    S_VETF, "S.VETF",
    /// `mur`: вертикальная часть здания, служащая для поддержания перекрытий и разделения помещения на части
    S_WALL, "S.WALL",
    /// `fortidsmur`: the remains of a linear defensive stone structure
    S_WALLA, "S.WALLA",
    /// `fiskedam`: гидротехническое сооружение, перегораживающее водоток для подъёма воды в реке, создания водохранилища или электростанции
    S_WEIR, "S.WEIR",
    /// `пристань`: специально оборудованное место, сооружение для причаливания и стоянки судов
    S_WHRF, "S.WHRF",
    /// `vrak`: the site of the remains of a wrecked vessel
    S_WRCK, "S.WRCK",
    /// `водопроводна станция`: a facility for supplying potable water through a water source and a system of pumps and filtration beds
    S_WTRW, "S.WTRW",
    /// `зона свободной торговли`: an area, usually a section of a port, where goods may be received and shipped free of customs duty and of most customs regulations
    S_ZNF, "S.ZNF",
    /// `зоопарк`: a zoological garden or park where wild animals are kept for exhibition
    S_ZOO, "S.ZOO",
    /// ``: mountain, hill, rock, ...
    T, "T",
    /// `асфалтово езеро`: a small basin containing naturally occurring asphalt
    T_ASPH, "T.ASPH",
    /// `атол(и)`: a ring-shaped coral reef which has closely spaced islands on it encircling a lagoon
    T_ATOL, "T.ATOL",
    /// `bar`: наносная мель у морских берегов (обычно в устье реки)
    T_BAR, "T.BAR",
    /// `бряг`: a shore zone of coarse unconsolidated sediment that extends from the low-water line to the highest reach of storm waves
    T_BCH, "T.BCH",
    /// `брегове`: a shore zone of coarse unconsolidated sediment that extends from the low-water line to the highest reach of storm waves
    T_BCHS, "T.BCHS",
    /// `steinørken`: неплодородная эродированная сильнопересечённая местность
    T_BDLD, "T.BDLD",
    /// `stenblocksfält`: равнинная местность, покрытая большими углообразными валунами
    T_BLDR, "T.BLDR",
    /// `blowhole(s)`: a hole in coastal rock through which sea water is forced by a rising tide or waves and spurted through an outlet into the air
    T_BLHL, "T.BLHL",
    /// `пукнатина(и)`: небольшое углубление на песчаной местности, образовавшееся в результате эрозии
    T_BLOW, "T.BLOW",
    /// `bench`: a long, narrow bedrock platform bounded by steeper slopes above and below, usually overlooking a waterbody
    T_BNCH, "T.BNCH",
    /// `høyde med bratte sider`: a small, isolated, usually flat-topped hill with steep sides
    T_BUTE, "T.BUTE",
    /// `мыс`: часть суши, острым углом вдающаяся в водное пространство (море, озеро, реку)
    T_CAPE, "T.CAPE",
    /// `klyfta`: a deep narrow slot, notch, or groove in a coastal cliff
    T_CFT, "T.CFT",
    /// `Вулканично пропадане`: a depression measuring kilometers across formed by the collapse of a volcanic mountain
    T_CLDA, "T.CLDA",
    /// `cliff(s)`: отвесная скала
    T_CLF, "T.CLF",
    /// `kanjon, djup flodbädd`: a deep, narrow valley with steep sides cutting into a plateau or mountainous area
    T_CNYN, "T.CNYN",
    /// `конус(и)`: a conical landform composed of mud or volcanic material
    T_CONE, "T.CONE",
    /// `коридор`: a strip or area of land having significance as an access way
    T_CRDR, "T.CRDR",
    /// `цирк`: a bowl-like hollow partially surrounded by cliffs or steep slopes at the head of a glaciated valley
    T_CRQ, "T.CRQ",
    /// `цирки`: естественные чашеобразные углубления в привершинной части горы (обычно со скалистыми или покрытыми льдом, снегом стенами)
    T_CRQS, "T.CRQS",
    /// `krater`: углубление на вершине вулкана, из которого при извержении выливается лава
    T_CRTR, "T.CRTR",
    /// `cuesta`: an asymmetric ridge formed on tilted strata
    T_CUET, "T.CUET",
    /// `дельта реки`: a flat plain formed by alluvial deposits at the mouth of a stream
    T_DLTA, "T.DLTA",
    /// `depression(s)`: понижение земной поверхности в пределах суши, на дне океанов и морей
    T_DPR, "T.DPR",
    /// `ørken`: обширная засушливая область с небольшим количеством осадков, резкими колебаниями воздуха и почвы и скудной растительностью
    T_DSRT, "T.DSRT",
    /// `дюня(и)`: песчаный холм или их группа, образованные ветром на слабо закреплённых растительностью песчаных массивах
    T_DUNE, "T.DUNE",
    /// `vannskille`: возвышенная местность между бассейнами рек, разделяющая их
    T_DVD, "T.DVD",
    /// `sandørken`: тип пустынь с песчаной поверхностью
    T_ERG, "T.ERG",
    /// `конус выноса`: a fan-shaped wedge of coarse alluvium with apex merging with a mountain stream bed and the fan spreading out at a low angle slope onto an adjacent plain
    T_FAN, "T.FAN",
    /// `брод`: мелкое место во всю ширину реки, озера и т.п., удобное для перехода, переезда
    T_FORD, "T.FORD",
    /// `fissure`: a crack associated with volcanism
    T_FSR, "T.FSR",
    /// `kløft`: a low place in a ridge, not used for transportation
    T_GAP, "T.GAP",
    /// `juv`: a short, narrow, steep-sided section of a stream valley
    T_GRGE, "T.GRGE",
    /// `headland`: часть суши, острым углом вдающаяся в водное пространство (море, озеро, реку)
    T_HDLD, "T.HDLD",
    /// `ås`: a rounded elevation of limited extent rising above the surrounding land with local relief of less than 300m
    T_HLL, "T.HLL",
    /// `kullar`: rounded elevations of limited extent rising above the surrounding land with local relief of less than 300m
    T_HLLS, "T.HLLS",
    /// `hammock (stykker av hevet land)`: ледяная глыба, образовавшаяся при сжатии льдов на северных морях и реках
    T_HMCK, "T.HMCK",
    /// `каменистая пустыня`: тип пустыни, развитой на слабовыветренных коренных породах плато низкогорий и мелкосопочника, покрытых щебнем и галькой (поверхность К. п. почти лишена почвенно-растительного покрова, животный компонент типично пустынный)
    T_HMDA, "T.HMDA",
    /// `interfluve`: a relatively undissected upland between adjacent stream valleys
    T_INTF, "T.INTF",
    /// `øy`: a tract of land, smaller than a continent, surrounded by water at high water
    T_ISL, "T.ISL",
    /// `islet`: small island, bigger than rock, smaller than island.
    T_ISLET, "T.ISLET",
    /// `искусственный остров`: an island created by landfill or diking and filling in a wetland, bay, or lagoon
    T_ISLF, "T.ISLF",
    /// `mangrove island`: a mangrove swamp surrounded by a waterbody
    T_ISLM, "T.ISLM",
    /// `острови`: части суши, со всех сторон окружённые водой
    T_ISLS, "T.ISLS",
    /// `остров свързан със сушата`: a coastal island connected to the mainland by barrier beaches, levees or dikes
    T_ISLT, "T.ISLT",
    /// `section of island`: 
    T_ISLX, "T.ISLX",
    /// `landtunge`: полоса суши, соединяющая два материка или материк с полуостровом и т.п., разделённые водой
    T_ISTH, "T.ISTH",
    /// `karstområde`: ландшафт, образованный деятельностью подземных вод на участках суши, поверхность которых сложена растворимыми горными породами: известняками, гипсом, каменной солью и др.
    T_KRST, "T.KRST",
    /// `зона с лава`: an area of solidified lava
    T_LAVA, "T.LAVA",
    /// `skyddsvall`: гидротехническое сооружение, перегораживающее водоток для подъёма воды в реке, создания водохранилища или электростанции
    T_LEV, "T.LEV",
    /// `плоскогорье`: a flat-topped, isolated elevation with steep slopes on all sides, less extensive than a plateau
    T_MESA, "T.MESA",
    /// `höjd(er)`: a low, isolated, rounded hill
    T_MND, "T.MND",
    /// `морена`: скопление обломков горных пород, образуемое на границе передвижения и таяния ледников
    T_MRN, "T.MRN",
    /// `mountain`: an elevation standing high above the surrounding area with small summit area, steep slopes and local relief of 300m or more
    T_MT, "T.MT",
    /// `планини`: значительные возвышенности, поднимающиеся над окружающей местностью
    T_MTS, "T.MTS",
    /// `meander neck`: a narrow strip of land between the two limbs of a meander loop at its narrowest point
    T_NKM, "T.NKM",
    /// `нунатак`: a rock or mountain peak protruding through glacial ice
    T_NTK, "T.NTK",
    /// `nunataks`: изолированные скалистые пики, горные гребни или холмы, выступающие над ледниковой поверхностью
    T_NTKS, "T.NTKS",
    /// `pan`: a near-level shallow, natural depression or basin, usually containing an intermittent lake, pond, or pool
    T_PAN, "T.PAN",
    /// `sänkor`: a near-level shallow, natural depression or basin, usually containing an intermittent lake, pond, or pool
    T_PANS, "T.PANS",
    /// `Дефиле`: a break in a mountain range or other high obstruction, used for transportation from one side to the other [See also gap]
    T_PASS, "T.PASS",
    /// `полуостров`: часть суши, с трёх сторон омываемая водой, а четвёртой стороной примыкающая к материку или острову
    T_PEN, "T.PEN",
    /// `част от полуостров`: 
    T_PENX, "T.PENX",
    /// `tind`: a pointed elevation atop a mountain, ridge, or other hypsographic feature
    T_PK, "T.PK",
    /// `върхове`: pointed elevations atop a mountain, ridge, or other hypsographic features
    T_PKS, "T.PKS",
    /// `platå`: an elevated plain with steep slopes on one or more sides, and often with incised streams
    T_PLAT, "T.PLAT",
    /// `section of plateau`: 
    T_PLATX, "T.PLATX",
    /// `Преградена с дига зона`: осушенный участок моря или другого водоёма
    T_PLDR, "T.PLDR",
    /// `sletteland`: обширный участок суши без значительных повышений и понижений поверхности
    T_PLN, "T.PLN",
    /// `Част от поле`: 
    T_PLNX, "T.PLNX",
    /// `промонторий`: выступ, мыс
    T_PROM, "T.PROM",
    /// `odde`: длинная узкая отмель, идущая от берега, или низменный узкий мыс
    T_PT, "T.PT",
    /// `odder`: tapering pieces of land projecting into a body of water, less prominent than a cape
    T_PTS, "T.PTS",
    /// `beach ridge`: a ridge of sand just inland and parallel to the beach, usually in series
    T_RDGB, "T.RDGB",
    /// `горный хребет`: ряд гор, тянущихся в одном направлении, горная цепь
    T_RDGE, "T.RDGE",
    /// `stenöken`: каменистая пустыня
    T_REG, "T.REG",
    /// `stein`: a conspicuous, isolated rocky mass
    T_RK, "T.RK",
    /// `Свлачище`: камни, низвергающиеся с гор лавиной
    T_RKFL, "T.RKFL",
    /// `rocks`: conspicuous, isolated rocky masses
    T_RKS, "T.RKS",
    /// `Пясъчна област`: территория, покрытая песками
    T_SAND, "T.SAND",
    /// `dry stream bed`: a channel formerly containing the water of a stream
    T_SBED, "T.SBED",
    /// `escarpment`: a long line of cliffs or steep slopes separating level surfaces above and below
    T_SCRP, "T.SCRP",
    /// `sal`: продолговатая впадина, понижение между вершинами горного хребта или возвышенности
    T_SDL, "T.SDL",
    /// `bredd`: полоса земной поверхности по обе стороны береговой линии моря, озера, водохранилища
    T_SHOR, "T.SHOR",
    /// `sänka`: a small crater-shape depression in a karst area
    T_SINK, "T.SINK",
    /// `Стръмнина`: a mound of earth material, at the base of a slope and the associated scoured area
    T_SLID, "T.SLID",
    /// `наклон(и)`: наклонная поверхность, скат (горы, холма и т.п.)
    T_SLP, "T.SLP",
    /// `Дълъг подводен бряг`: узкий и длинный мыс при слиянии двух рек; песчаная коса, намытая прибоем
    T_SPIT, "T.SPIT",
    /// `spur(s)`: ответвление основной горной цепи
    T_SPUR, "T.SPUR",
    /// `steinur`: a steep concave slope formed by an accumulation of loose rock fragments at the base of a cliff or steep slope
    T_TAL, "T.TAL",
    /// `dal mellom sanddyner`: a long wind-swept trough between parallel longitudinal dunes
    T_TRGD, "T.TRGD",
    /// `terrasse`: a long, narrow alluvial platform bounded by steeper slopes above and below, usually overlooking a waterbody
    T_TRR, "T.TRR",
    /// `плоскогорье`: an extensive interior region of high land with low to moderate surface relief
    T_UPLD, "T.UPLD",
    /// `долина`: an elongated depression usually traversed by a stream
    T_VAL, "T.VAL",
    /// `hanging valley`: боковая долина, днище которой оканчивается выше днища главной долины
    T_VALG, "T.VALG",
    /// `dalar`: elongated depressions usually traversed by a stream
    T_VALS, "T.VALS",
    /// `del av dal`: 
    T_VALX, "T.VALX",
    /// `vulkan`: конусообразная гора с кратером на вершине, через который из недр земли извергаются огонь, лава, пепел, горячие газы, пары воды и обломки горных пород
    T_VLC, "T.VLC",
    /// ``: undersea
    U, "U",
    /// `береговой шельф`: a gentle slope, with a generally smooth surface, particularly found around groups of islands and seamounts
    U_APNU, "U.APNU",
    /// `Арка`: территория, пространство, границы которого имеют форму изогнутой линии (например, дуга на юго-востоке Гавайского хребта)
    U_ARCU, "U.ARCU",
    /// `arrugado`: an area of subdued corrugations off Baja California
    U_ARRU, "U.ARRU",
    /// `borderland`: a region adjacent to a continent, normally occupied by or bordering a shelf, that is highly irregular with depths well in excess of those typical of a shelf
    U_BDLU, "U.BDLU",
    /// `наносы`: elevations, typically located on a shelf, over which the depth of water is relatively shallow but sufficient for safe surface navigation
    U_BKSU, "U.BKSU",
    /// `bench`: a small terrace
    U_BNCU, "U.BNCU",
    /// `нанос`: геологическое образование, обязанное своим происхождением деятельности текучей воды; возвышенность на поверхности шельфа, не препятствующее судоходству
    U_BNKU, "U.BNKU",
    /// `sänka, bassäng`: понижение земной поверхности в пределах суши, на дне океанов и морей
    U_BSNU, "U.BSNU",
    /// `cordillera`: an entire mountain system including the subordinate ranges, interior plateaus, and basins
    U_CDAU, "U.CDAU",
    /// `kanjoner`: глубокие речные долины с очень крутыми или отвесными склонами
    U_CNSU, "U.CNSU",
    /// `canyon`: a relatively narrow, deep depression with steep sides, the bottom of which generally has a continuous slope
    U_CNYU, "U.CNYU",
    /// `материковое подножие`: a gentle slope rising from oceanic depths towards the foot of a continental slope
    U_CRSU, "U.CRSU",
    /// `Бездна`: a localized deep area within the confines of a larger feature, such as a trough, basin or trench
    U_DEPU, "U.DEPU",
    /// `kontinentalsokkelkant`: внешний край континентального шельфа
    U_EDGU, "U.EDGU",
    /// `утес`: отвесная скала
    U_ESCU, "U.ESCU",
    /// `конус выступа`: a relatively smooth feature normally sloping away from the lower termination of a canyon or canyon system
    U_FANU, "U.FANU",
    /// `низменность`: a small level or nearly level area
    U_FLTU, "U.FLTU",
    /// `fork`: a branch of a canyon or valley
    U_FRKU, "U.FRKU",
    /// `forks`: a branch of a canyon or valley
    U_FRSU, "U.FRSU",
    /// `Зона на накъсване`: an extensive linear zone of irregular topography of the sea floor, characterized by steep-sided or asymmetrical ridges, troughs, or escarpments
    U_FRZU, "U.FRZU",
    /// `spår`: длинная узкая впадина на дне океана или моря
    U_FURU, "U.FURU",
    /// `ущелье`: a narrow break in a ridge or rise
    U_GAPU, "U.GAPU",
    /// `gully`: a small valley-like feature
    U_GLYU, "U.GLYU",
    /// `холм`: an elevation rising generally less than 500 meters
    U_HLLU, "U.HLLU",
    /// `åser`: возвышенности, высота которых менее 500 метров
    U_HLSU, "U.HLSU",
    /// `hull`: понижение земной поверхности в пределах суши, на дне океанов и морей
    U_HOLU, "U.HOLU",
    /// `knaus`: an elevation rising generally more than 500 meters and less than 1,000 meters and of limited extent across the summit
    U_KNLU, "U.KNLU",
    /// `knolls`: elevations rising generally more than 500 meters and less than 1,000 meters and of limited extent across the summits
    U_KNSU, "U.KNSU",
    /// `тераса`: надводные или подводные скалистые возвышения морского дна, опасные для судоходства
    U_LDGU, "U.LDGU",
    /// `levee`: an embankment bordering a canyon, valley, or seachannel
    U_LEVU, "U.LEVU",
    /// `median valley`: the axial depression of the mid-oceanic ridge system
    U_MDVU, "U.MDVU",
    /// `mesa`: гора с плоскими вершинами и более или менее крутыми, иногда ступенчатыми склонами
    U_MESU, "U.MESU",
    /// `mound`: низкий холмик округлой формы на месте погребения
    U_MNDU, "U.MNDU",
    /// `Ров`: длинное углубление, вырытое в земле
    U_MOTU, "U.MOTU",
    /// `mountains`: well-delineated subdivisions of a large and complex positive feature
    U_MTSU, "U.MTSU",
    /// `fjell`: a well-delineated subdivision of a large and complex positive feature
    U_MTU, "U.MTU",
    /// `toppar`: prominent elevations, part of a larger feature, either pointed or of very limited extent across the summit
    U_PKSU, "U.PKSU",
    /// `topp`: остроконечная горная вершина или высшая точка горной вершины вообще
    U_PKU, "U.PKU",
    /// `platform`: a flat or gently sloping underwater surface extending seaward from the shore
    U_PLFU, "U.PLFU",
    /// `slätt(land)`: a flat, gently sloping or nearly level region
    U_PLNU, "U.PLNU",
    /// `plateau`: возвышенная равнина, ограниченная чётко выраженными уступами, крутыми склонами
    U_PLTU, "U.PLTU",
    /// `Шпиц`: остроконечная скала
    U_PNLU, "U.PNLU",
    /// `provins`: часть природной географической зоны в составе определенной физико-географической области; региональная единица физико-географического районирования. Физико-географические провинции выделяются по морфоструктурным особенностям рельефа и климату, а в горах - по характеру высотной поясности
    U_PRVU, "U.PRVU",
    /// `ravine`: a small canyon
    U_RAVU, "U.RAVU",
    /// `ås`: a long narrow elevation with steep sides
    U_RDGU, "U.RDGU",
    /// `Гребени`: цепь невысоких гор
    U_RDSU, "U.RDSU",
    /// `rev`: surface-navigation hazards composed of consolidated material
    U_RFSU, "U.RFSU",
    /// `rev`: надводные или подводные скалистые возвышения морского дна, опасные для судоходства
    U_RFU, "U.RFU",
    /// `höjd`: участок морского дна,  возвышающийся над окружающим рельефом
    U_RISU, "U.RISU",
    /// `ramp`: a gentle slope connecting areas of different elevations
    U_RMPU, "U.RMPU",
    /// `range`: a series of associated ridges or seamounts
    U_RNGU, "U.RNGU",
    /// `канал на дне океана`: впадина на дне морского дна, с покатыми краями и защищенная с одной или двух сторон валом
    U_SCNU, "U.SCNU",
    /// `sjøkanaler`: впадины на дне морского дна с пологими склонами, защищенные с одной или двух сторон валом
    U_SCSU, "U.SCSU",
    /// `sal`: a low part, resembling in shape a saddle, in a ridge or between contiguous seamounts
    U_SDLU, "U.SDLU",
    /// `шелф`: прибрежная мелководная зона океана (с глубинами до 200 м); материковая отмель
    U_SHFU, "U.SHFU",
    /// `grund, sandrev`: участок (обычно прибрежный) реки, озера и т.п. с небольшой глубиной
    U_SHLU, "U.SHLU",
    /// `мелководья`: hazards to surface navigation composed of unconsolidated material
    U_SHSU, "U.SHSU",
    /// `shelf valley`: относительно мелководная подводная равнина, прилегающая к берегам материков и генетически составляющая часть материковой платформы
    U_SHVU, "U.SHVU",
    /// `tröskel`: the low part of a gap or saddle separating basins
    U_SILU, "U.SILU",
    /// `sluttning`: покатая поверхность горы, холма и т.п.; склон
    U_SLPU, "U.SLPU",
    /// `undervattensberg`: изолированные поднятия морского дна с относительной высотой более 500 м и четко выраженной конической или куполообразной одной или несколькими вершинами и крутыми склонами
    U_SMSU, "U.SMSU",
    /// `seamount`: an elevation rising generally more than 1,000 meters and of limited extent across the summit
    U_SMU, "U.SMU",
    /// `spur`: a subordinate elevation, ridge, or rise projecting outward from a larger feature
    U_SPRU, "U.SPRU",
    /// `terrasse`: a relatively flat horizontal or gently inclined surface, sometimes long and narrow, which is bounded by a steeper ascending slope on one side and by a steep descending slope on the opposite side
    U_TERU, "U.TERU",
    /// `guyoter`: seamounts having a comparatively smooth, flat top
    U_TMSU, "U.TMSU",
    /// `tablemount (or guyot)`: подводный плосковершинный вулкан
    U_TMTU, "U.TMTU",
    /// `нос`: an elongate (tongue-like) extension of a flat sea floor into an adjacent higher feature
    U_TNGU, "U.TNGU",
    /// `падина`: углубление с пологими склонами  на дне океанов и морей
    U_TRGU, "U.TRGU",
    /// `renne`: a long, narrow, characteristically very deep and asymmetrical depression of the sea floor, with relatively steep sides
    U_TRNU, "U.TRNU",
    /// `valley`: a relatively shallow, wide depression, the bottom of which usually has a continuous gradient
    U_VALU, "U.VALU",
    /// `daler`: a relatively shallow, wide depression, the bottom of which usually has a continuous gradient
    U_VLSU, "U.VLSU",
    /// ``: forest, heath, ...
    V, "V",
    /// `djungel`: a small clump of conspicuous bushes in an otherwise bare area
    V_BUSH, "V.BUSH",
    /// `обработваеми площи`: площадь, предназначенная для возделывания многих с.-х. культур
    V_CULT, "V.CULT",
    /// `гори`: множество дикорастущих деревьев, расположенных на большом пространстве; пространство, обильно поросшее деревьями
    V_FRST, "V.FRST",
    /// `forsteinet skog`: значительное скопление древесных окаменелостей на небольшом участке
    V_FRSTF, "V.FRSTF",
    /// `grässlätt`: an area dominated by grass vegetation
    V_GRSLD, "V.GRSLD",
    /// `kokoslund`: большой участок земли, занятый кокосовыми деревьями
    V_GRVC, "V.GRVC",
    /// `плантация оливковых деревьев`: a planting of olive trees
    V_GRVO, "V.GRVO",
    /// `палмова плантация`: небольшой лесной участок, состоящий из пальмовых деревьев
    V_GRVP, "V.GRVP",
    /// `furulund`: небольшой лесной участок, состоящий из сосен
    V_GRVPN, "V.GRVPN",
    /// `heath`: an upland moor or sandy area dominated by low shrubby vegetation including heather
    V_HTH, "V.HTH",
    /// `meadow`: небольшой луг, луг среди других угодий (лесных, пастбищных
    V_MDW, "V.MDW",
    /// `fruktträdgård(ar)`: участок земли, на котором выращивают фруктовые деревья
    V_OCH, "V.OCH",
    /// `храсти`: an area of low trees, bushes, and shrubs stunted by some environmental limitation
    V_SCRB, "V.SCRB",
    /// `tree(s)`: a conspicuous tree used as a landmark
    V_TREE, "V.TREE",
    /// `тундра`: a marshy, treeless, high latitude plain, dominated by mosses, lichens, and low shrub vegetation under permafrost conditions
    V_TUND, "V.TUND",
    /// `vingård`: a planting of grapevines
    V_VIN, "V.VIN",
    /// `виноградники`: виноградные плантации
    V_VINS, "V.VINS",
    /// `Wikipedia Article`: A Wikipedia article
    WikipediaArticle, "WikipediaArticle",
    /// ``: 
    alternateName, "alternateName",
    /// `children features`: Links to an RDF document containing the descriptions of children features
    childrenFeatures, "childrenFeatures",
    /// `colloquial name`: 
    colloquialName, "colloquialName",
    /// `ISO country code`: A two letters country code in the ISO 3166 list
    countryCode, "countryCode",
    /// `feature class`: The main category of the feature, as defined in geonames taxonomy.
    featureClass, "featureClass",
    /// `feature code`: Type of the feature, as defined in geonames taxonomy.
    featureCode, "featureCode",
    /// `geonames identifier`: 
    geonamesID, "geonamesID",
    /// `historical name`: 
    historicalName, "historicalName",
    /// `located in`: Indicates that the subject resource is located in the object feature
    locatedIn, "locatedIn",
    /// `map`: A geonames map centered on the feature.
    locationMap, "locationMap",
    /// `name`: The main international name of a feature. The value has no xml:lang tag.
    name, "name",
    /// `nearby`: A feature close to the reference feature
    nearby, "nearby",
    /// `nearby features`: Links to an RDF document containing the descriptions of nearby features
    nearbyFeatures, "nearbyFeatures",
    /// `neighbour`: A feature sharing a common boarder with the reference feature
    neighbour, "neighbour",
    /// `neighbouring features`: Links to an RDF document containing the descriptions of neighbouring features. Applies when the feature has definite boarders.
    neighbouringFeatures, "neighbouringFeatures",
    /// `official name`: A name in an official local language
    officialName, "officialName",
    /// `level 1 administrative parent`: 
    parentADM1, "parentADM1",
    /// `level 2 administrative parent`: 
    parentADM2, "parentADM2",
    /// `level 3 administrative parent`: 
    parentADM3, "parentADM3",
    /// `level 4 administrative parent`: 
    parentADM4, "parentADM4",
    /// `parent country`: 
    parentCountry, "parentCountry",
    /// `parent feature`: A feature parent of the current one, in either administrative or physical subdivision.
    parentFeature, "parentFeature",
    /// `population`: 
    population, "population",
    /// `postal code`: 
    postalCode, "postalCode",
    /// `short name`: 
    shortName, "shortName",
    /// `wikipedia article`: A Wikipedia article of which subject is the resource.
    wikipediaArticle, "wikipediaArticle"
);
