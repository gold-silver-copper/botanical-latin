use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::error::Error;

pub type NounMap = HashMap<String, NounRecord>;
pub type AdjectiveMap = HashMap<String, AdjectiveRecord>;
pub type VerbMap = HashMap<String, VerbRecord>;
pub type Verb = String;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Latin {
    pub noun_map: NounMap,
    pub adj_map: AdjectiveMap,
    pub verb_map: VerbMap,
}

pub struct ComplexNoun {
    //   pub case: Case,
    //  pub number: Number,
    pub head_noun: String,
    pub adjective: Vec<String>,
    pub adposition_noun: Vec<String>,
}

impl Default for ComplexNoun {
    fn default() -> Self {
        Self {
            head_noun: "exemplum".into(),
            adposition_noun: Vec::new(),
            adjective: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct NounRecord {
    pub word: String,
    pub nom_sg: String,
    pub gen_sg: String,
    pub dat_sg: String,
    pub acc_sg: String,
    pub abl_sg: String,
    pub voc_sg: String,
    pub loc_sg: String,
    pub nom_pl: String,
    pub gen_pl: String,
    pub dat_pl: String,
    pub acc_pl: String,
    pub abl_pl: String,
    pub voc_pl: String,
    pub loc_pl: String,

    #[serde(deserialize_with = "deserialize_gender")]
    pub gender: Gender,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Mood {
    Indicative,
    Subjunctive,
    Imperative,
    Infinitive,
    Participle,
    VerbalNoun,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Voice {
    Active,
    Passive,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Tense {
    Present,
    Imperfect,
    Future,
    Perfect,
    Pluperfect,
    FuturePerfect,
}

impl Default for Gender {
    fn default() -> Gender {
        Gender::Masculine
    }
}

//word,canonical,present_infinitive,perfect_active,supine,conjugation,irregular
#[derive(Debug, Deserialize, Clone, Default)]
pub struct VerbRecord {
    pub word: String,
    pub canonical: String,
    pub present_infinitive: String,
    pub perfect_active: String,
    pub supine: String,
    pub indicative_active_present_singular_first: String,
    pub indicative_active_present_singular_second: String,
    pub indicative_active_present_singular_third: String,
    pub indicative_active_present_plural_first: String,
    pub indicative_active_present_plural_second: String,
    pub indicative_active_present_plural_third: String,
    pub indicative_active_imperfect_singular_first: String,
    pub indicative_active_imperfect_singular_second: String,
    pub indicative_active_imperfect_singular_third: String,
    pub indicative_active_imperfect_plural_first: String,
    pub indicative_active_imperfect_plural_second: String,
    pub indicative_active_imperfect_plural_third: String,
    pub indicative_active_future_singular_first: String,
    pub indicative_active_future_singular_second: String,
    pub indicative_active_future_singular_third: String,
    pub indicative_active_future_plural_first: String,
    pub indicative_active_future_plural_second: String,
    pub indicative_active_future_plural_third: String,
    pub indicative_active_perfect_singular_first: String,
    pub indicative_active_perfect_singular_second: String,
    pub indicative_active_perfect_singular_third: String,
    pub indicative_active_perfect_plural_first: String,
    pub indicative_active_perfect_plural_second: String,
    pub indicative_active_perfect_plural_third: String,
    pub indicative_active_pluperfect_singular_first: String,
    pub indicative_active_pluperfect_singular_second: String,
    pub indicative_active_pluperfect_singular_third: String,
    pub indicative_active_pluperfect_plural_first: String,
    pub indicative_active_pluperfect_plural_second: String,
    pub indicative_active_pluperfect_plural_third: String,
}

//word,feminine,neuter,comparative,superlative,adverb,declension,adj_stem
#[derive(Debug, Deserialize, Clone, Default)]
pub struct AdjectiveRecord {
    pub word: String,

    pub comparative: String,
    pub superlative: String,
    pub adverb: String,
    pub nom_sg_masc: String,
    pub gen_sg_masc: String,
    pub dat_sg_masc: String,
    pub acc_sg_masc: String,
    pub abl_sg_masc: String,
    pub nom_sg_fem: String,
    pub gen_sg_fem: String,
    pub dat_sg_fem: String,
    pub acc_sg_fem: String,
    pub abl_sg_fem: String,
    pub nom_sg_neut: String,
    pub gen_sg_neut: String,
    pub dat_sg_neut: String,
    pub acc_sg_neut: String,
    pub abl_sg_neut: String,
    pub nom_pl_masc: String,
    pub gen_pl_masc: String,
    pub dat_pl_masc: String,
    pub acc_pl_masc: String,
    pub abl_pl_masc: String,
    pub nom_pl_fem: String,
    pub gen_pl_fem: String,
    pub dat_pl_fem: String,
    pub acc_pl_fem: String,
    pub abl_pl_fem: String,
    pub nom_pl_neut: String,
    pub gen_pl_neut: String,
    pub dat_pl_neut: String,
    pub acc_pl_neut: String,
    pub abl_pl_neut: String,
}

fn deserialize_gender<'de, D>(deserializer: D) -> Result<Gender, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    match s.as_str() {
        "m" => Ok(Gender::Masculine),
        "f" => Ok(Gender::Feminine),
        "n" => Ok(Gender::Neuter),
        _ => Err(serde::de::Error::custom("unknown gender")),
    }
}

fn deserialize_pluralia<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    match s.as_str() {
        "fa" => Ok(false),
        "tr" => Ok(true),

        _ => Err(serde::de::Error::custom("unknown pluralia")),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Case {
    Nom,
    Gen,
    Dat,
    Acc,
    Abl,
    Loc,
    Voc,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaseEndings {
    pub gender: Gender,
    pub nom_sg: &'static str,
    pub acc_sg: &'static str,
    pub gen_sg: &'static str,
    pub dat_sg: &'static str,
    pub abl_sg: &'static str,

    pub nom_pl: &'static str,
    pub acc_pl: &'static str,
    pub gen_pl: &'static str,
    pub dat_pl: &'static str,
    pub abl_pl: &'static str,
}

impl CaseEndings {
    pub fn ending(&self, case: &Case, number: &Number) -> &str {
        match number {
            Number::Singular => match case {
                Case::Nom => self.nom_sg,
                Case::Acc => self.acc_sg,
                Case::Gen => self.gen_sg,
                Case::Dat => self.dat_sg,
                Case::Abl => self.abl_sg,
                Case::Loc => self.abl_sg,
                Case::Voc => self.nom_sg,
            },
            Number::Plural => match case {
                Case::Nom => self.nom_pl,
                Case::Acc => self.acc_pl,
                Case::Gen => self.gen_pl,
                Case::Dat => self.dat_pl,
                Case::Abl => self.abl_pl,
                Case::Loc => self.abl_pl,
                Case::Voc => self.nom_pl,
            },
        }
    }
}

pub const ONE_LETTER_ENDINGS: [CaseEndings; 4] = [
    A_DECLENSION_ENDINGS,
    E_DECLENSION_ENDINGS,
    O_DECLENSION_ENDINGS,
    U_DECLENSION_ENDINGS,
];
pub const TWO_LETTER_ENDINGS: [CaseEndings; 23] = [
    AL_DECLENSION_ENDINGS,
    AR_DECLENSION_ENDINGS,
    AR_DECLENSION_ENDINGS,
    AS_DECLENSION_ENDINGS,
    AX_DECLENSION_ENDINGS,
    EN_DECLENSION_ENDINGS,
    ER_DECLENSION_ENDINGS,
    ES_DECLENSION_ENDINGS,
    EX_DECLENSION_ENDINGS,
    ON_DECLENSION_ENDINGS,
    OR_DECLENSION_ENDINGS,
    OS_DECLENSION_ENDINGS,
    UM_DECLENSION_ENDINGS,
    US_DECLENSION_ENDINGS,
    UT_DECLENSION_ENDINGS,
    UX_DECLENSION_ENDINGS,
    MA_DECLENSION_ENDINGS,
    YX_DECLENSION_ENDINGS,
    YS_DECLENSION_ENDINGS,
    NX_DECLENSION_ENDINGS,
    MA_DECLENSION_ENDINGS,
    IS_DECLENSION_ENDINGS,
    IX_DECLENSION_ENDINGS,
];

pub const TEST_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Neuter,
    nom_sg: "nom_sg",
    acc_sg: "acc_sg",
    gen_sg: "gen_sg",
    dat_sg: "dat_sg",
    abl_sg: "abl_sg",

    nom_pl: "nom_pl",
    acc_pl: "acc_pl",
    gen_pl: "gen_pl",
    dat_pl: "dat_pl",
    abl_pl: "abl_pl",
};

pub const A_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Feminine,
    nom_sg: "a",
    acc_sg: "am",
    gen_sg: "ae",
    dat_sg: "ae",
    abl_sg: "a",

    nom_pl: "ae",
    acc_pl: "as",
    gen_pl: "arum",
    dat_pl: "is",
    abl_pl: "is",
};

/*pub const RA_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Feminine,
    nom_sg: "ra",
    acc_sg: "ram",
    gen_sg: "rae",
    dat_sg: "rae",
    abl_sg: "ra",

    nom_pl: "rae",
    acc_pl: "ras",
    gen_pl: "rarum",
    dat_pl: "ris",
    abl_pl: "ris",
};
 */
pub const US_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "us",
    acc_sg: "um",
    gen_sg: "i",
    dat_sg: "o",
    abl_sg: "o",

    nom_pl: "i",
    acc_pl: "os",
    gen_pl: "orum",
    dat_pl: "is",
    abl_pl: "is",
};

pub const O_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "o",
    acc_sg: "onem",
    gen_sg: "onis",
    dat_sg: "oni",
    abl_sg: "one",

    nom_pl: "ones",
    acc_pl: "ones",
    gen_pl: "onum",
    dat_pl: "onibus",
    abl_pl: "onibus",
};

pub const ON_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "on",
    acc_sg: "ontem",
    gen_sg: "ontis",
    dat_sg: "onti",
    abl_sg: "onte",

    nom_pl: "ontes",
    acc_pl: "ontes",
    gen_pl: "ontum",
    dat_pl: "ontibus",
    abl_pl: "ontibus",
};

pub const UT_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Neuter,
    nom_sg: "ut",
    acc_sg: "ut",
    gen_sg: "itis",
    dat_sg: "iti",
    abl_sg: "ite",

    nom_pl: "ita",
    acc_pl: "ita",
    gen_pl: "itum",
    dat_pl: "itibus",
    abl_pl: "itibus",
};

pub const OR_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "or",
    acc_sg: "orem",
    gen_sg: "oris",
    dat_sg: "ori",
    abl_sg: "ore",

    nom_pl: "ores",
    acc_pl: "ores",
    gen_pl: "orum",
    dat_pl: "oribus",
    abl_pl: "oribus",
};

pub const OR_ADJ_NEUTER_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "or",
    acc_sg: "or",
    gen_sg: "oris",
    dat_sg: "ori",
    abl_sg: "ori",

    nom_pl: "oria",
    acc_pl: "oria",
    gen_pl: "orium",
    dat_pl: "oribus",
    abl_pl: "oribus",
};

pub const OS_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "os",
    acc_sg: "orem",
    gen_sg: "oris",
    dat_sg: "ori",
    abl_sg: "ore",

    nom_pl: "ores",
    acc_pl: "ores",
    gen_pl: "orum",
    dat_pl: "oribus",
    abl_pl: "oribus",
};
pub const S_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "s",
    acc_sg: "tem",
    gen_sg: "tis",
    dat_sg: "ti",
    abl_sg: "te",

    nom_pl: "tes",
    acc_pl: "tes",
    gen_pl: "tium",
    dat_pl: "tibus",
    abl_pl: "tibus",
};

pub const S_ADJ_NEUTER_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "s",
    acc_sg: "s",
    gen_sg: "tis",
    dat_sg: "ti",
    abl_sg: "te",

    nom_pl: "tia",
    acc_pl: "tia",
    gen_pl: "tium",
    dat_pl: "tibus",
    abl_pl: "tibus",
};

pub const UM_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Neuter,
    nom_sg: "um",
    acc_sg: "um",
    gen_sg: "i",
    dat_sg: "o",
    abl_sg: "o",

    nom_pl: "a",
    acc_pl: "a",
    gen_pl: "orum",
    dat_pl: "is",
    abl_pl: "is",
};

/*pub const RUM_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Neuter,
    nom_sg: "rum",
    acc_sg: "rum",
    gen_sg: "ri",
    dat_sg: "ro",
    abl_sg: "ro",

    nom_pl: "ra",
    acc_pl: "ra",
    gen_pl: "rorum",
    dat_pl: "ris",
    abl_pl: "ris",
}; */

pub const U_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Neuter,
    nom_sg: "u",
    acc_sg: "u",
    gen_sg: "us",
    dat_sg: "ui",
    abl_sg: "u",

    nom_pl: "ua",
    acc_pl: "ua",
    gen_pl: "uum",
    dat_pl: "uibus",
    abl_pl: "uibus",
};

pub const ER_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "er",
    acc_sg: "er",
    gen_sg: "eris",
    dat_sg: "eri",
    abl_sg: "ere",

    nom_pl: "era",
    acc_pl: "era",
    gen_pl: "erum",
    dat_pl: "eribus",
    abl_pl: "eribus",
};

pub const ER_ADJECTIVE_MASC_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "",
    acc_sg: "um",
    gen_sg: "i",
    dat_sg: "o",
    abl_sg: "o",

    nom_pl: "i",
    acc_pl: "os",
    gen_pl: "orum",
    dat_pl: "is",
    abl_pl: "is",
};

pub const AL_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Neuter,
    nom_sg: "al",
    acc_sg: "al",
    gen_sg: "alis",
    dat_sg: "ali",
    abl_sg: "ali",

    nom_pl: "alia",
    acc_pl: "alia",
    gen_pl: "alium",
    dat_pl: "alibus",
    abl_pl: "alibus",
};

pub const AR_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Neuter,
    nom_sg: "ar",
    acc_sg: "ar",
    gen_sg: "aris",
    dat_sg: "ari",
    abl_sg: "ari",

    nom_pl: "aria",
    acc_pl: "aria",
    gen_pl: "arium",
    dat_pl: "aribus",
    abl_pl: "aribus",
};

pub const AS_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Feminine,
    nom_sg: "as",
    acc_sg: "atem",
    gen_sg: "atis",
    dat_sg: "ati",
    abl_sg: "ate",

    nom_pl: "ates",
    acc_pl: "ates",
    gen_pl: "atum",
    dat_pl: "atibus",
    abl_pl: "atibus",
};

pub const AX_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Feminine,
    nom_sg: "ax",
    acc_sg: "acem",
    gen_sg: "acis",
    dat_sg: "aci",
    abl_sg: "ace",

    nom_pl: "aces",
    acc_pl: "aces",
    gen_pl: "acum",
    dat_pl: "acibus",
    abl_pl: "acibus",
};

pub const IX_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Feminine,
    nom_sg: "ix",
    acc_sg: "icem",
    gen_sg: "icis",
    dat_sg: "ici",
    abl_sg: "ice",

    nom_pl: "ices",
    acc_pl: "ices",
    gen_pl: "icum",
    dat_pl: "icibus",
    abl_pl: "icibus",
};
pub const YX_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Feminine,
    nom_sg: "yx",
    acc_sg: "ycem",
    gen_sg: "ycis",
    dat_sg: "yci",
    abl_sg: "yce",

    nom_pl: "yces",
    acc_pl: "yces",
    gen_pl: "ycum",
    dat_pl: "ycibus",
    abl_pl: "ycibus",
};

pub const YS_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "ys",
    acc_sg: "ydem",
    gen_sg: "ydis",
    dat_sg: "ydi",
    abl_sg: "yde",

    nom_pl: "ydes",
    acc_pl: "ydes",
    gen_pl: "ydum",
    dat_pl: "ydibus",
    abl_pl: "ydibus",
};

pub const UX_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Feminine,
    nom_sg: "ux",
    acc_sg: "ucem",
    gen_sg: "ucis",
    dat_sg: "uci",
    abl_sg: "uce",

    nom_pl: "uces",
    acc_pl: "uces",
    gen_pl: "ucum",
    dat_pl: "ucibus",
    abl_pl: "ucibus",
};

pub const NX_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Feminine,
    nom_sg: "nx",
    acc_sg: "ngem",
    gen_sg: "ngis",
    dat_sg: "ngi",
    abl_sg: "nge",

    nom_pl: "nges",
    acc_pl: "nges",
    gen_pl: "ngium",
    dat_pl: "ngibus",
    abl_pl: "ngibus",
};

pub const IS_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "is",
    acc_sg: "em",
    gen_sg: "is",
    dat_sg: "i",
    abl_sg: "e",

    nom_pl: "es",
    acc_pl: "es",
    gen_pl: "ium",
    dat_pl: "ibus",
    abl_pl: "ibus",
};

pub const IS_ADJECTIVE_MASC_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "is",
    acc_sg: "em",
    gen_sg: "is",
    dat_sg: "i",
    abl_sg: "i",

    nom_pl: "es",
    acc_pl: "es",
    gen_pl: "ium",
    dat_pl: "ibus",
    abl_pl: "ibus",
};

pub const EX_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "ex",
    acc_sg: "icem",
    gen_sg: "icis",
    dat_sg: "ici",
    abl_sg: "ice",

    nom_pl: "ices",
    acc_pl: "ices",
    gen_pl: "icum",
    dat_pl: "icibus",
    abl_pl: "icibus",
};

pub const EX_ADJECTIVE_MASC_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "ex",
    acc_sg: "icem",
    gen_sg: "icis",
    dat_sg: "ici",
    abl_sg: "ici",

    nom_pl: "ices",
    acc_pl: "ices",
    gen_pl: "icium",
    dat_pl: "icibus",
    abl_pl: "icibus",
};

pub const EX_ADJECTIVE_NEUT_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Masculine,
    nom_sg: "ex",
    acc_sg: "ex",
    gen_sg: "icis",
    dat_sg: "ici",
    abl_sg: "ici",

    nom_pl: "icia",
    acc_pl: "icia",
    gen_pl: "icium",
    dat_pl: "icibus",
    abl_pl: "icibus",
};

pub const E_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Neuter,
    nom_sg: "e",
    acc_sg: "e",
    gen_sg: "is",
    dat_sg: "i",
    abl_sg: "i",

    nom_pl: "ia",
    acc_pl: "ia",
    gen_pl: "ium",
    dat_pl: "ibus",
    abl_pl: "ibus",
};
pub const EN_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Neuter,
    nom_sg: "en",
    acc_sg: "en",
    gen_sg: "inis",
    dat_sg: "ini",
    abl_sg: "ine",

    nom_pl: "ina",
    acc_pl: "ina",
    gen_pl: "inum",
    dat_pl: "inibus",
    abl_pl: "inibus",
};
pub const ES_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Feminine,
    nom_sg: "es",
    acc_sg: "em",
    gen_sg: "ei",
    dat_sg: "ei",
    abl_sg: "e",

    nom_pl: "es",
    acc_pl: "es",
    gen_pl: "erum",
    dat_pl: "ebus",
    abl_pl: "ebus",
};

pub const ES_ADJ_MASC_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Feminine,
    nom_sg: "es",
    acc_sg: "em",
    gen_sg: "is",
    dat_sg: "i",
    abl_sg: "e",

    nom_pl: "es",
    acc_pl: "es",
    gen_pl: "um",
    dat_pl: "ibus",
    abl_pl: "ibus",
};
pub const ES_ADJ_NEUT_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Feminine,
    nom_sg: "es",
    acc_sg: "em",
    gen_sg: "is",
    dat_sg: "i",
    abl_sg: "e",

    nom_pl: "es",
    acc_pl: "es",
    gen_pl: "um",
    dat_pl: "ibus",
    abl_pl: "ibus",
};



pub const MA_DECLENSION_ENDINGS: CaseEndings = CaseEndings {
    gender: Gender::Neuter,
    nom_sg: "ma",
    acc_sg: "ma",
    gen_sg: "matis",
    dat_sg: "mati",
    abl_sg: "mate",

    nom_pl: "mata",
    acc_pl: "mata",
    gen_pl: "matum",
    dat_pl: "matibus",
    abl_pl: "matibus",
};

// have a possesive func, but reflexive person?
#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Singular,
    Plural,
}

pub type Noun = (String, Gender);
pub type Adjective = String;

#[derive(Debug, PartialEq, Clone)]
pub enum Person {
    First,
    Second,
    Third,
}

impl Latin {
    pub fn guess_noun( word: &str, case: &Case, number: &Number) -> Noun {
        let mut appended = Vec::from(TWO_LETTER_ENDINGS);
        appended.append(&mut Vec::from(ONE_LETTER_ENDINGS));

        for ce in &appended {
            let nom_end = ce.nom_sg;

            if word.ends_with(nom_end) {
                let word_gender = &ce.gender;

                let n = nom_end.len();

                let mut word_stem = word.to_string();

                if n <= word.len() {
                    word_stem.truncate(word.len() - n);
                    let ending = ce.ending(case, number);
                    let merged = format!("{}{}", word_stem, ending);
                    return (merged, word_gender.clone());
                }
            }
        }

        (word.to_string(), Gender::Masculine)
    }

    pub fn guess_adjective(word: &str, case: &Case, number: &Number, gender: &Gender) -> Adjective {

        let mut word_stem = word.to_string();
        word_stem.truncate(word_stem.len() - 2);

        let mut masc_ends = &US_DECLENSION_ENDINGS; 
        let mut fem_ends = &A_DECLENSION_ENDINGS;
        let mut neut_ends = &UM_DECLENSION_ENDINGS;
    

   if word.ends_with("us") {

    word_stem = word.to_string();
    word_stem.truncate(word.len() - 2);

  masc_ends = &US_DECLENSION_ENDINGS; 
    fem_ends = &A_DECLENSION_ENDINGS;
    neut_ends = &UM_DECLENSION_ENDINGS;

   }
  

    else if word.ends_with("er") {
        word_stem = word.to_string();
       
        masc_ends = &ER_ADJECTIVE_MASC_ENDINGS;

    }
   else  if word.ends_with("is") {
         word_stem = word.to_string();
         word_stem.truncate(word.len() - 2);
       
        masc_ends = &IS_ADJECTIVE_MASC_ENDINGS;
        fem_ends = &IS_ADJECTIVE_MASC_ENDINGS;
       neut_ends = &E_DECLENSION_ENDINGS;

    }

    else if word.ends_with("ex") {
        word_stem = word.to_string();
        word_stem.truncate(word.len() - 2);
      
       masc_ends = &EX_ADJECTIVE_MASC_ENDINGS;
       fem_ends = &EX_ADJECTIVE_MASC_ENDINGS;
      neut_ends = &EX_ADJECTIVE_NEUT_ENDINGS;

   }

  else  if word.ends_with("or") {
    word_stem = word.to_string();
    word_stem.truncate(word.len() - 2);
  
   masc_ends = &OR_DECLENSION_ENDINGS;
   fem_ends = &OR_DECLENSION_ENDINGS;
  neut_ends = &OR_ADJ_NEUTER_ENDINGS;

}


else if word.ends_with("des") {
    word_stem = word.to_string();
    word_stem.truncate(word.len() - 2);
  
   masc_ends = &ES_ADJ_MASC_ENDINGS;
   fem_ends = &ES_ADJ_MASC_ENDINGS;
  neut_ends = &ES_ADJ_NEUT_ENDINGS;

}
else if word.ends_with("s") {
    word_stem = word.to_string();
    word_stem.truncate(word.len() - 1);
  
   masc_ends = &S_DECLENSION_ENDINGS;
   fem_ends = &S_DECLENSION_ENDINGS;
  neut_ends = &S_ADJ_NEUTER_ENDINGS;

}



let ending = match gender {
    Gender::Feminine => fem_ends.ending(case, number),
    Gender::Masculine => masc_ends.ending(case, number),
    Gender::Neuter => neut_ends.ending(case, number)
};

format!("{word_stem}{ending}")


    }

    pub fn complex_noun(
        &self,
        complex_nomen: &ComplexNoun,
        case: &Case,
        number: &Number,
    ) -> String {
        let noun = self.noun(&complex_nomen.head_noun, case, number);

        let mut response = noun.0;

        for adpos in &complex_nomen.adposition_noun {
            let adposik = self.noun(adpos, case, number);
            if adposik.0 != "" {
                response = format!("{} {}", response, adposik.0);
            }
        }

        for adj in &complex_nomen.adjective {
            let adjik = self.adjective(adj, case, number, &noun.1);
            if adjik != "" {
                response = format!("{} {}", response, adjik);
            }
        }

        response
    }

    pub fn noun(&self, word: &str, case: &Case, number: &Number) -> Noun {
        let defik = NounRecord::default();

        let recordik = self.noun_map.get(word);

        match recordik {
            Some(record) => { let mut response = match number {
                Number::Singular => match case {
                    Case::Nom => (record.nom_sg.clone(), record.gender.clone()),
                    Case::Gen => (record.gen_sg.clone(), record.gender.clone()),
                    Case::Dat => (record.dat_sg.clone(), record.gender.clone()),
                    Case::Acc => (record.acc_sg.clone(), record.gender.clone()),
                    Case::Abl => (record.abl_sg.clone(), record.gender.clone()),
                    Case::Voc => (record.voc_sg.clone(), record.gender.clone()),
                    Case::Loc => (record.loc_sg.clone(), record.gender.clone()),
                },
                Number::Plural => match case {
                    Case::Nom => (record.nom_pl.clone(), record.gender.clone()),
                    Case::Gen => (record.gen_pl.clone(), record.gender.clone()),
                    Case::Dat => (record.dat_pl.clone(), record.gender.clone()),
                    Case::Acc => (record.acc_pl.clone(), record.gender.clone()),
                    Case::Abl => (record.abl_pl.clone(), record.gender.clone()),
                    Case::Voc => (record.voc_pl.clone(), record.gender.clone()),
                    Case::Loc => (record.loc_pl.clone(), record.gender.clone()),
                },
            };
    
            if case == &Case::Loc && (response.0 == "" || response.0 == "-") {
                response.0 = format!("{}", record.abl_sg.clone());
            }
    
            if (response.0 == "" || response.0 == "-") {
                response = Latin::guess_noun(word, case, number);
            }
    
            response},
            None => Latin::guess_noun(word, case, number)
        }

       
    }

    pub fn new(noun_path: String, adjective_path: String, verb_path: String) -> Self {
        Latin {
            noun_map: Latin::load_nouns_from_csv(noun_path),
            adj_map: Latin::load_adjectives_from_csv(adjective_path),
            verb_map: Latin::load_verbs_from_csv(verb_path),
        }
    }

    //"nouns.csv"
    pub fn load_nouns_from_csv(path: String) -> NounMap {
        let mut nounmap = HashMap::new();
        let file_path: &str = path.as_str();

        let mut rdr = csv::Reader::from_path(file_path).unwrap();
        for result in rdr.deserialize() {
            let record: NounRecord = result.unwrap();

            nounmap.insert(record.word.clone(), record.clone());

            //    //println!("{:?}", record);
        }
        nounmap
    }
    pub fn load_adjectives_from_csv(path: String) -> AdjectiveMap {
        let file_path: &str = path.as_str();
        let mut adjmap = HashMap::new();
        let mut rdr = csv::Reader::from_path(file_path).unwrap();
        for result in rdr.deserialize() {
            //   //println!("{:?}", result);
            let record: AdjectiveRecord = result.unwrap();
            adjmap.insert(record.word.clone(), record.clone());
            //  //println!("{:?}", record);
        }
        adjmap
    }

    pub fn load_verbs_from_csv(path: String) -> VerbMap {
        let file_path: &str = path.as_str();
        let mut verbmap = HashMap::new();
        let mut rdr = csv::Reader::from_path(file_path).unwrap();
        for result in rdr.deserialize() {
            //println!("{:?}", result);
            let record: VerbRecord = result.unwrap();
            verbmap.insert(record.word.clone(), record.clone());
            //println!("{:?}", record);
        }
        verbmap
    }

    pub fn adjective(
        &self,
        word: &str,
        case: &Case,
        number: &Number,
        gender: &Gender,
    ) -> Adjective {
        let defik = AdjectiveRecord::default();

        let recordik = self.adj_map.get(word);

        match recordik {
            Some(record) => {    match gender {
                Gender::Masculine => match number {
                    Number::Singular => match case {
                        Case::Nom => record.nom_sg_masc.clone(),
                        Case::Gen => record.gen_sg_masc.clone(),
                        Case::Dat => record.dat_sg_masc.clone(),
                        Case::Acc => record.acc_sg_masc.clone(),
                        Case::Abl => record.abl_sg_masc.clone(),
                        _ => record.abl_sg_masc.clone(),
                    },
                    Number::Plural => match case {
                        Case::Nom => record.nom_pl_masc.clone(),
                        Case::Gen => record.gen_pl_masc.clone(),
                        Case::Dat => record.dat_pl_masc.clone(),
                        Case::Acc => record.acc_pl_masc.clone(),
                        Case::Abl => record.abl_pl_masc.clone(),
                        _ => record.abl_pl_masc.clone(),
                    },
                },
                Gender::Feminine => match number {
                    Number::Singular => match case {
                        Case::Nom => record.nom_sg_fem.clone(),
                        Case::Gen => record.gen_sg_fem.clone(),
                        Case::Dat => record.dat_sg_fem.clone(),
                        Case::Acc => record.acc_sg_fem.clone(),
                        Case::Abl => record.abl_sg_fem.clone(),
                        _ => record.abl_sg_fem.clone(),
                    },
                    Number::Plural => match case {
                        Case::Nom => record.nom_pl_fem.clone(),
                        Case::Gen => record.gen_pl_fem.clone(),
                        Case::Dat => record.dat_pl_fem.clone(),
                        Case::Acc => record.acc_pl_fem.clone(),
                        Case::Abl => record.abl_pl_fem.clone(),
                        _ => record.abl_pl_fem.clone(),
                    },
                },
                Gender::Neuter => match number {
                    Number::Singular => match case {
                        Case::Nom => record.nom_sg_neut.clone(),
                        Case::Gen => record.gen_sg_neut.clone(),
                        Case::Dat => record.dat_sg_neut.clone(),
                        Case::Acc => record.acc_sg_neut.clone(),
                        Case::Abl => record.abl_sg_neut.clone(),
                        _ => record.abl_sg_neut.clone(),
                    },
                    Number::Plural => match case {
                        Case::Nom => record.nom_pl_neut.clone(),
                        Case::Gen => record.gen_pl_neut.clone(),
                        Case::Dat => record.dat_pl_neut.clone(),
                        Case::Acc => record.acc_pl_neut.clone(),
                        Case::Abl => record.abl_pl_neut.clone(),
                        _ => record.abl_pl_neut.clone(),
                    },
                },
            }
        
        
        },
        None => Latin::guess_adjective(word, case, number, gender)

        }

    
    }

    pub fn verb(
        &self,
        word: &str,
        mood: &Mood,
        voice: &Voice,
        tense: &Tense,
        number: &Number,
        person: &Person,
    ) -> Verb {
        let defik = VerbRecord::default();

        let record = self.verb_map.get(word).unwrap_or(&defik);

        match mood {
            Mood::Indicative => match voice {
                Voice::Active => match tense {
                    Tense::Present => match number {
                        Number::Singular => match person {
                            Person::First => {
                                record.indicative_active_present_singular_first.clone()
                            }
                            Person::Second => {
                                record.indicative_active_present_singular_second.clone()
                            }
                            Person::Third => {
                                record.indicative_active_present_singular_third.clone()
                            }
                        },
                        Number::Plural => match person {
                            Person::First => record.indicative_active_present_plural_first.clone(),
                            Person::Second => {
                                record.indicative_active_present_plural_second.clone()
                            }
                            Person::Third => record.indicative_active_present_plural_third.clone(),
                        },
                    },
                    Tense::Imperfect => match number {
                        Number::Singular => match person {
                            Person::First => {
                                record.indicative_active_imperfect_singular_first.clone()
                            }
                            Person::Second => {
                                record.indicative_active_imperfect_singular_second.clone()
                            }
                            Person::Third => {
                                record.indicative_active_imperfect_singular_third.clone()
                            }
                        },
                        Number::Plural => match person {
                            Person::First => {
                                record.indicative_active_imperfect_plural_first.clone()
                            }
                            Person::Second => {
                                record.indicative_active_imperfect_plural_second.clone()
                            }
                            Person::Third => {
                                record.indicative_active_imperfect_plural_third.clone()
                            }
                        },
                    },
                    Tense::Future => match number {
                        Number::Singular => match person {
                            Person::First => record.indicative_active_future_singular_first.clone(),
                            Person::Second => {
                                record.indicative_active_future_singular_second.clone()
                            }
                            Person::Third => record.indicative_active_future_singular_third.clone(),
                        },
                        Number::Plural => match person {
                            Person::First => record.indicative_active_future_plural_first.clone(),
                            Person::Second => record.indicative_active_future_plural_second.clone(),
                            Person::Third => record.indicative_active_future_plural_third.clone(),
                        },
                    },
                    Tense::Perfect => match number {
                        Number::Singular => match person {
                            Person::First => {
                                record.indicative_active_perfect_singular_first.clone()
                            }
                            Person::Second => {
                                record.indicative_active_perfect_singular_second.clone()
                            }
                            Person::Third => {
                                record.indicative_active_perfect_singular_third.clone()
                            }
                        },
                        Number::Plural => match person {
                            Person::First => record.indicative_active_perfect_plural_first.clone(),
                            Person::Second => {
                                record.indicative_active_perfect_plural_second.clone()
                            }
                            Person::Third => record.indicative_active_perfect_plural_third.clone(),
                        },
                    },
                    Tense::Pluperfect => match number {
                        Number::Singular => match person {
                            Person::First => {
                                record.indicative_active_pluperfect_singular_first.clone()
                            }
                            Person::Second => {
                                record.indicative_active_pluperfect_singular_second.clone()
                            }
                            Person::Third => {
                                record.indicative_active_pluperfect_singular_third.clone()
                            }
                        },
                        Number::Plural => match person {
                            Person::First => {
                                record.indicative_active_pluperfect_plural_first.clone()
                            }
                            Person::Second => {
                                record.indicative_active_pluperfect_plural_second.clone()
                            }
                            Person::Third => {
                                record.indicative_active_pluperfect_plural_third.clone()
                            }
                        },
                    },
                    Tense::FuturePerfect => {
                        todo!("IMPLEMENT FUTURE PERFECT")
                    }
                },
                _ => todo!("IMPLEMENT PASSIVE VOICE"),
            },
            _ => todo!("IMPLEMENT OTHER MOODS"),
        }
    }

    pub fn last_n_chars(word: &str, n: usize) -> String {
        let split_pos = word.char_indices().nth_back(n - 1).unwrap_or((0, 'a')).0;
        word[split_pos..].into()
    }
}

/*

fn main() {
    //println!("Hello, world!");

    let boop = Latin::last_n_chars("be", 3);
    //println!("boopik : {:#?}", boop);
    let conji = Latin::new();

    let testik = conji.noun_map.clone();
    let testik2 = conji.adj_map.clone();
    let testik3 = conji.verb_map.clone();

    for wot in testik {
        //println!("new_noun : {:#?}", wot);
        let new_noun = conji.noun(&wot.0, &Case::Acc, &Number::Singular);
        //println!("new_noun : {:#?}", new_noun);
    }
    for wot in testik2 {
        //println!("adj : {:#?}", wot);
        let new_noun = conji.adjective(&wot.0, &Case::Acc, &Number::Singular, &Gender::Feminine);
        //println!("adj : {:#?}", new_noun);
    }
    for wot in testik3 {
        //println!("verb : {:#?}", wot);
        let new_noun = conji.verb(&wot.0, &Mood::Indicative, &Voice::Active, &Tense::Perfect, &Number::Plural, &Person::Second);
        //println!("verb : {:#?}", new_noun);
    }
}


*/
