//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/world-types.md
//! @prompt-hash d359b14c
//! @layer L1
//! @updated 2026-03-27

use super::file_id::FileId;
use super::span::Span;

/// Conteúdo binário de um ficheiro carregado.
/// Interior provisório — pode mudar de `Vec<u8>` para o tipo real no Passo 5.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn new(data: Vec<u8>) -> Self { Self(data) }
    pub fn as_slice(&self) -> &[u8] { &self.0 }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

/// Fonte tipográfica carregada.
/// Opaca até Font ser migrado no Passo 5.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Font(Vec<u8>);

impl Font {
    pub fn from_data(data: Vec<u8>) -> Self { Self(data) }
    pub fn as_slice(&self) -> &[u8] { &self.0 }
}

/// Biblioteca de funções e valores do Typst.
/// Opaca até Library ser migrada no Passo 4.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Library(());

impl Default for Library {
    fn default() -> Self {
        Self::new()
    }
}

impl Library {
    pub fn new() -> Self { Self(()) }
}

/// Data e hora para o método `today()` de `World`.
/// Wrapper sobre `time::Date` + `Option<time::Time>` — ADR-0021.
/// `time::OffsetDateTime::now_utc()` não entra em L1 — fica em L3.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Datetime {
    date: time::Date,
    time: Option<time::Time>,
}

impl Datetime {
    /// Cria Datetime a partir de componentes de data.
    /// Retorna None se a data não for válida no calendário gregoriano.
    pub fn new_date(year: i32, month: u8, day: u8) -> Option<Self> {
        let month = time::Month::try_from(month).ok()?;
        let date  = time::Date::from_calendar_date(year, month, day).ok()?;
        Some(Self { date, time: None })
    }

    /// Cria Datetime com componentes de hora.
    /// Retorna None se a data ou hora não forem válidas.
    pub fn new_datetime(
        year: i32, month: u8, day: u8,
        hour: u8, minute: u8, second: u8,
    ) -> Option<Self> {
        let month = time::Month::try_from(month).ok()?;
        let date  = time::Date::from_calendar_date(year, month, day).ok()?;
        let time  = time::Time::from_hms(hour, minute, second).ok()?;
        Some(Self { date, time: Some(time) })
    }

    pub fn year(&self)   -> i32        { self.date.year() }
    pub fn month(&self)  -> u8         { self.date.month() as u8 }
    pub fn day(&self)    -> u8         { self.date.day() }
    pub fn hour(&self)   -> Option<u8> { self.time.map(|t| t.hour()) }
    pub fn minute(&self) -> Option<u8> { self.time.map(|t| t.minute()) }
    pub fn second(&self) -> Option<u8> { self.time.map(|t| t.second()) }

    /// Dia da semana: 1=Segunda … 7=Domingo (ISO 8601).
    pub fn weekday(&self) -> u8 {
        self.date.weekday().number_from_monday()
    }
}

/// Erro de acesso a ficheiro.
#[derive(Clone, Debug, PartialEq, Eq, Hash, thiserror::Error)]
pub enum FileError {
    #[error("file not found")]
    NotFound,
    #[error("access denied")]
    AccessDenied,
    #[error("{0}")]
    Other(String),
}

/// Resultado de uma operação de ficheiro.
pub type FileResult<T> = Result<T, FileError>;

/// Vtable de execução do compilador Typst.
///
/// Stub — migração após Content e Func estarem em L1. ADR-0017.
/// No original: macro-generated struct com fn pointers para eval_string,
/// eval_closure, realize, layout_frame, etc.
pub struct Routines(());

impl Routines {
    pub fn new() -> Self { Self(()) }
}

impl Default for Routines {
    fn default() -> Self { Self::new() }
}

/// Rastreia o span sob inspecção para diagnósticos ricos.
///
/// Paridade com `Traced` do Typst vanilla (ADR-0033). `Default` corresponde
/// a "não rastrear nada"; `new(span)` embrulha um span a rastrear.
#[derive(Default)]
pub struct Traced(Option<Span>);

impl Traced {
    /// Embrulha um `Span` a rastrear.
    ///
    /// Para não rastrear nada, usar `Traced::default()`.
    pub fn new(traced: Span) -> Self {
        Self(Some(traced))
    }
}

#[comemo::track]
impl Traced {
    /// Devolve o span rastreado _se_ pertence ao ficheiro dado, `None`
    /// caso contrário.
    ///
    /// Filtrar por ficheiro garante que só resultados da fonte com o span
    /// rastreado são invalidados pelo `comemo`.
    pub fn get(&self, id: FileId) -> Option<Span> {
        if self.0.and_then(Span::id) == Some(id) { self.0 } else { None }
    }
}


/// Sistema de propriedades encadeadas do Typst.
///
/// Stub — NÃO migrar neste passo. No original: `EcoVec<LazyHash<Style>>`
/// com vtable dinâmica para show rules e set rules. Dependências:
/// ecow, typst_utils::LazyHash, Style (vtable dinâmica), Content.
pub struct Styles(());

impl Styles {
    pub fn new() -> Self { Self(()) }
}

impl Default for Styles {
    fn default() -> Self { Self::new() }
}

/// Rota de compilação para detecção de ciclos de importação.
///
/// Stub — o original usa lista ligada com lifetime + AtomicUsize +
/// `#[comemo::track]`, o que complica o esqueleto neste passo.
/// ADR-0017: implementação real quando eval() for incrementalmente migrado.
#[derive(Hash)]
pub struct Route(());

impl Route {
    pub fn new() -> Self { Self(()) }
}

impl Default for Route {
    fn default() -> Self { Self::new() }
}

#[comemo::track]
impl Route {}


/// Colector de diagnósticos durante eval().
///
/// Stub — o original usa EcoVec (ecow), Introspection, Value, Styles.
/// ADR-0017: implementação real quando esses tipos migrarem.
#[derive(Hash)]
pub struct Sink(());

impl Sink {
    pub fn new() -> Self { Self(()) }
}

impl Default for Sink {
    fn default() -> Self { Self::new() }
}

#[comemo::track]
impl Sink {}


/// Contexto central de compilação.
///
/// Stub — no original: Routines + TrackedWorld + Introspector + Traced + Sink + Route.
/// ADR-0017: tipo real quando os campos migrarem.
pub struct Engine(());

impl Engine {
    pub fn new() -> Self { Self(()) }
}

impl Default for Engine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_new_and_slice() {
        let b = Bytes::new(vec![1, 2, 3]);
        assert_eq!(b.as_slice(), &[1u8, 2, 3]);
        assert_eq!(b.len(), 3);
        assert!(!b.is_empty());
    }

    #[test]
    fn bytes_empty() {
        let b = Bytes::new(vec![]);
        assert!(b.is_empty());
        assert_eq!(b.len(), 0);
    }

    #[test]
    fn file_error_not_found_display() {
        assert_eq!(FileError::NotFound.to_string(), "file not found");
    }

    #[test]
    fn file_error_access_denied_display() {
        assert_eq!(FileError::AccessDenied.to_string(), "access denied");
    }

    #[test]
    fn file_error_other_display() {
        assert_eq!(FileError::Other("custom".to_string()).to_string(), "custom");
    }

    // ── Datetime ──────────────────────────────────────────────────────────────

    #[test]
    fn datetime_date_valida() {
        let d = Datetime::new_date(2026, 3, 27).unwrap();
        assert_eq!(d.year(), 2026);
        assert_eq!(d.month(), 3);
        assert_eq!(d.day(), 27);
        assert!(d.hour().is_none());
    }

    #[test]
    fn datetime_month_invalido() {
        assert!(Datetime::new_date(2026, 0, 1).is_none());
        assert!(Datetime::new_date(2026, 13, 1).is_none());
    }

    #[test]
    fn datetime_day_invalido_para_mes() {
        // Fevereiro 2026 tem 28 dias (2026 não é bissexto)
        assert!(Datetime::new_date(2026, 2, 29).is_none());
        // 2024 foi bissexto
        assert!(Datetime::new_date(2024, 2, 29).is_some());
    }

    #[test]
    fn datetime_com_hora() {
        let d = Datetime::new_datetime(2026, 3, 27, 14, 30, 0).unwrap();
        assert_eq!(d.hour(), Some(14));
        assert_eq!(d.minute(), Some(30));
        assert_eq!(d.second(), Some(0));
    }

    #[test]
    fn datetime_weekday_segunda() {
        // 2026-03-23 foi segunda-feira
        let d = Datetime::new_date(2026, 3, 23).unwrap();
        assert_eq!(d.weekday(), 1); // ISO 8601: Segunda = 1
    }

    #[test]
    fn datetime_roundtrip() {
        let d = Datetime::new_date(2026, 12, 31).unwrap();
        assert_eq!(d.year(), 2026);
        assert_eq!(d.month(), 12);
        assert_eq!(d.day(), 31);
    }

    // ── Engine type stubs ──────────────────────────────────────────────────────

    #[test]
    fn routines_stub_exists() {
        // Contrato correcto — stub opaco compila e existe como tipo
        let _ = Routines::new();
    }

    #[test]
    fn traced_stub_exists() {
        let _ = Traced::default();
    }

    // ── Traced (ADR-0033, paridade com vanilla) ───────────────────────────────

    #[test]
    fn traced_default_retorna_none() {
        // Default = "não rastrear nada": get devolve None para qualquer FileId.
        use std::num::NonZeroU16;
        let id = FileId::from_raw(NonZeroU16::new(7).unwrap());
        assert_eq!(Traced::default().get(id), None);
    }

    #[test]
    fn traced_com_span_preserva_valor() {
        use std::num::NonZeroU16;
        let file  = FileId::from_raw(NonZeroU16::new(3).unwrap());
        let other = FileId::from_raw(NonZeroU16::new(4).unwrap());
        let span  = Span::from_number(file, 42).unwrap();

        let t = Traced::new(span);
        // get(mesmo ficheiro) devolve o span rastreado.
        assert_eq!(t.get(file),  Some(span));
        // get(ficheiro diferente) devolve None — filtro por FileId.
        assert_eq!(t.get(other), None);
    }

    #[test]
    fn traced_integra_com_comemo_track() {
        // `#[comemo::track]` gera `.track()` que devolve `Tracked<'_, Traced>`.
        // O teste é compile-time: a chamada abaixo só compila se o trait existe.
        use comemo::Track;
        let t = Traced::default();
        let _tracked: comemo::Tracked<'_, Traced> = t.track();
    }

    #[test]
    fn styles_stub_exists() {
        let _ = Styles::new();
    }

    #[test]
    fn route_stub_exists() {
        let _ = Route::new();
    }

    #[test]
    fn sink_stub_exists() {
        let _ = Sink::new();
    }

    #[test]
    fn engine_stub_exists() {
        let _ = Engine::new();
    }
}
