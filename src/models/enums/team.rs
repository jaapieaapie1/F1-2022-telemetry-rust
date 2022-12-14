use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, Copy, Clone, Debug, Serialize)]
pub enum Team {
    Mercedes = 0,
    Ferrari = 1,
    RedBullRacing = 2,
    Williams = 3,
    AstonMartin = 4,
    Alpine = 5,
    AlphaTauri = 6,
    Haas = 7,
    McLaren = 8,
    AlfaRomeo = 9,
    Mercedes2020 = 85,
    Ferrari2020 = 86,
    RedBullRacing2020 = 87,
    Williams2020 = 88,
    RacingPoint2020 = 89,
    Renault2020 = 90,
    AlphaTauri2020 = 91,
    Haas2020 = 92,
    McLaren2020 = 93,
    AlfaRomeo2020 = 94,
    AstonMartinDB11V12 = 95,
    AstonMartinVantageF1 = 96,
    AstonMartinVantageSafety = 97,
    FerrariF8Tributo = 98,
    FerrariRoma = 99,
    McLaren720S = 100,
    McLarenArtura = 101,
    MercedesAMGGTSafety = 102,
    MercedesAMGGTR = 103,
    F1CustomTeam = 104,
    Prema21 = 106,
    UniVirtuosi21 = 107,
    Carlin21 = 108,
    Hitech21 = 109,
    ArtGP21 = 110,
    MPMotorsport21 = 111,
    Charouz21 = 112,
    Dams21 = 113,
    Campos21 = 114,
    BWT21 = 115,
    Trident21 = 116,
    MercedesAMGGT = 117,
    Unknown = 255,
}