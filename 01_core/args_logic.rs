//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Args
//! Responsabilidade: Funções atômicas, puras e desvinculadas do I/O, focadas inteiramente em domínio técnico.

use std::num::NonZeroUsize;
use std::ops::RangeInclusive;
use std::str::FromStr;
use chrono::{DateTime, Utc};

/// Análise funcional pura de um par chave-valor para inputs do sistema (`sys.inputs`).
///
/// Falha se não contiver "=" ou se a chave for vazia.
pub fn parse_sys_input_pair(raw: &str) -> Result<(String, String), String> {
    let (key, val) = raw
        .split_once('=')
        .ok_or("input must be a key and a value separated by an equal sign")?;
    let key = key.trim().to_owned();
    if key.is_empty() {
        return Err("the key was missing or empty".to_owned());
    }
    let val = val.trim().to_owned();
    Ok((key, val))
}

/// Parsing atômico de timestamp Unix para formato DateTime Padrão.
/// Regra pura focada no <https://reproducible-builds.org/specs/source-date-epoch/>.
pub fn parse_source_date_epoch(raw: &str) -> Result<DateTime<Utc>, String> {
    let timestamp: i64 = raw
        .parse()
        .map_err(|err| format!("timestamp must be decimal integer ({err})"))?;
    DateTime::from_timestamp(timestamp, 0)
        .ok_or_else(|| "timestamp out of range".to_string())
}

/// Valida e realiza conversão segura do número da página.
pub fn parse_page_number(value: &str) -> Result<NonZeroUsize, &'static str> {
    if value == "0" {
        Err("page numbers start at one")
    } else {
        NonZeroUsize::from_str(value).map_err(|_| "not a valid page number")
    }
}

/// Ranges lógicos de páginas empaginadas.
#[derive(Debug, Clone)]
pub struct Pages(pub RangeInclusive<Option<NonZeroUsize>>);

impl FromStr for Pages {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.split('-').map(str::trim).collect::<Vec<_>>().as_slice() {
            [] | [""] => Err("page export range must not be empty"),
            [single_page] => {
                let page_number = parse_page_number(single_page)?;
                Ok(Pages(Some(page_number)..=Some(page_number)))
            }
            ["", ""] => Err("page export range must have start or end"),
            [start, ""] => Ok(Pages(Some(parse_page_number(start)?)..=None)),
            ["", end] => Ok(Pages(None..=Some(parse_page_number(end)?))),
            [start, end] => {
                let start = parse_page_number(start)?;
                let end = parse_page_number(end)?;
                if start > end {
                    Err("page export range must end at a page after the start")
                } else {
                    Ok(Pages(Some(start)..=Some(end)))
                }
            }
            [_, _, _, ..] => Err("page export range must have a single hyphen"),
        }
    }
}

/// Domínio da formatação final do Target / Arquivo.
pub enum Target {
    Paged,
    Html,
}

pub enum OutputFormat {
    Pdf,
    Png,
    Svg,
    Html,
}

impl OutputFormat {
    /// Domínio puro de averiguação se o arquivo deve resultar na representação empaginada.
    pub fn is_paged(&self) -> bool {
        matches!(self, Self::Pdf | Self::Png | Self::Svg)
    }
}
