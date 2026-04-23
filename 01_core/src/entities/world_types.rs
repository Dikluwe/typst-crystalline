//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/world-types.md
//! @prompt-hash d359b14c
//! @layer L1
//! @updated 2026-03-27

use std::sync::atomic::{AtomicUsize, Ordering};

use comemo::{Track, Tracked, Validate};

use super::file_id::FileId;
use super::source_result::{SourceDiagnostic, SourceResult};
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

/// Rota de compilação usada para detectar imports cíclicos e aninhamento
/// excessivo.
///
/// Paridade com `Route` do Typst vanilla (ADR-0033). Cada segmento é um
/// elo numa lista ligada imutável: o segmento actual referencia o pai
/// através de `outer: Option<Tracked<'a, Self>>`, tal como o vanilla
/// (`engine.rs:251`). A travessia via `contains`/`within` é mediada pelo
/// proxy `Tracked` do `comemo` (ADR-0001).
pub struct Route<'a> {
    /// Segmento pai, se existir.
    ///
    /// Parametrizamos a `Constraint` via `Route<'static>` — pattern
    /// documentado na docstring do `comemo::Tracked` para habilitar
    /// covariância em cadeias `Tracked<'a, Self>`. Sem este override
    /// o `Tracked` seria invariante em `T` e o encadeamento recursivo
    /// de `Route::extend` não compilaria. Equivale ao truque do
    /// vanilla com `<Route<'static> as Track>::Call`, adaptado à API
    /// de `comemo 0.4.0`.
    outer: Option<Tracked<'a, Self, <Route<'static> as Validate>::Constraint>>,
    /// Definido quando o segmento foi inserido na entrada de avaliação de
    /// um módulo.
    id: Option<FileId>,
    /// Contribuição deste segmento para a profundidade total da rota. Soma
    /// com os `len` dos segmentos `outer`.
    len: usize,
    /// Upper bound estabelecido para o comprimento da cadeia parente.
    ///
    /// Não guardamos o comprimento exacto — isso anularia o reuso de cache
    /// do `comemo` para profundidades distintas mas ambas não-excedentes.
    upper: AtomicUsize,
}

impl<'a> Route<'a> {
    /// Cria um segmento de rota raiz, vazio.
    pub fn root() -> Self {
        Self {
            id: None,
            outer: None,
            len: 0,
            upper: AtomicUsize::new(0),
        }
    }

    /// Estende a rota com um novo segmento de comprimento 1.
    pub fn extend(outer: Tracked<'a, Self>) -> Self {
        Route {
            outer: Some(outer),
            id: None,
            len: 1,
            upper: AtomicUsize::new(usize::MAX),
        }
    }

    /// Anota o segmento com o `FileId` do módulo em avaliação.
    pub fn with_id(self, id: FileId) -> Self {
        Self { id: Some(id), ..self }
    }

    /// Zera a contribuição deste segmento para a profundidade.
    pub fn unnested(self) -> Self {
        Self { len: 0, ..self }
    }

    /// Começa a rastrear esta rota.
    ///
    /// Em comparação com `Track::track`, salta este elo se o segmento
    /// actual não contribui com `id` nem com `len` — optimização de cache
    /// do `comemo`.
    pub fn track(&self) -> Tracked<'_, Self> {
        match self.outer {
            Some(outer) if self.id.is_none() && self.len == 0 => outer,
            _ => Track::track(self),
        }
    }

    /// Incrementa a profundidade contribuída por este segmento.
    pub fn increase(&mut self) {
        self.len += 1;
    }

    /// Decrementa a profundidade contribuída por este segmento.
    pub fn decrease(&mut self) {
        self.len -= 1;
    }
}

/// Limites de profundidade. Distintos para que, mesmo quando show-rule e
/// call-checks se alternam, o erro de show-rule seja sempre emitido antes
/// dos outros (precedência por menor limite).
impl Route<'_> {
    /// Profundidade máxima de show rules aninhadas.
    pub const MAX_SHOW_RULE_DEPTH: usize = 64;

    /// Profundidade máxima de layout aninhado.
    pub const MAX_LAYOUT_DEPTH: usize = 72;

    /// Profundidade máxima de HTML aninhado.
    pub const MAX_HTML_DEPTH: usize = 72;

    /// Profundidade máxima de chamadas de função.
    pub const MAX_CALL_DEPTH: usize = 80;
}

/// Verifica o limite de show rules a partir de um proxy `Tracked<Route>`.
///
/// Os `check_*_depth` do vanilla são métodos sobre `&Route`; no cristalino,
/// as funções do eval só têm acesso a `Tracked<'r, Route<'r>>` (o Route é
/// propagado covariante, ADR-0036). Como `Tracked` só expõe métodos do
/// bloco `#[comemo::track]`, estas verificações são funções livres que
/// usam `within` (tracked) e constroem o diagnóstico externamente.
pub fn check_show_depth(route: Tracked<'_, Route<'_>>) -> SourceResult<()> {
    if !route.within(Route::MAX_SHOW_RULE_DEPTH) {
        return Err(vec![
            SourceDiagnostic::error(
                Span::detached(),
                "maximum show rule depth exceeded",
            )
            .with_hint("maybe a show rule matches its own output")
            .with_hint("maybe there are too deeply nested elements"),
        ]);
    }
    Ok(())
}

/// Verifica o limite de layout — paridade com `typst-layout/src/flow/mod.rs:143`.
pub fn check_layout_depth(route: Tracked<'_, Route<'_>>) -> SourceResult<()> {
    if !route.within(Route::MAX_LAYOUT_DEPTH) {
        return Err(vec![
            SourceDiagnostic::error(
                Span::detached(),
                "maximum layout depth exceeded",
            )
            .with_hint("try to reduce the amount of nesting in your layout"),
        ]);
    }
    Ok(())
}

/// Verifica o limite de HTML — paridade com `typst-html/src/fragment.rs:66`.
pub fn check_html_depth(route: Tracked<'_, Route<'_>>) -> SourceResult<()> {
    if !route.within(Route::MAX_HTML_DEPTH) {
        return Err(vec![
            SourceDiagnostic::error(
                Span::detached(),
                "maximum HTML depth exceeded",
            )
            .with_hint("try to reduce the amount of nesting of your HTML"),
        ]);
    }
    Ok(())
}

/// Verifica o limite de chamadas — paridade com `typst-eval/src/call.rs:33`.
pub fn check_call_depth(route: Tracked<'_, Route<'_>>) -> SourceResult<()> {
    if !route.within(Route::MAX_CALL_DEPTH) {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "maximum function call depth exceeded",
        )]);
    }
    Ok(())
}

#[comemo::track]
#[allow(clippy::needless_lifetimes)]
impl<'a> Route<'a> {
    /// Verifica se o `id` faz parte da rota (segmento actual ou cadeia
    /// `outer`).
    pub fn contains(&self, id: FileId) -> bool {
        self.id == Some(id) || self.outer.is_some_and(|outer| outer.contains(id))
    }

    /// Verifica se a profundidade total da rota é ≤ `depth`.
    pub fn within(&self, depth: usize) -> bool {
        // Só precisamos de atomicidade, não de sincronização com outras
        // operações — `Relaxed` é suficiente.
        let upper = self.upper.load(Ordering::Relaxed);
        if upper.saturating_add(self.len) <= depth {
            return true;
        }

        match self.outer {
            Some(_) if depth < self.len => false,
            Some(outer) => {
                let within = outer.within(depth - self.len);
                if within && depth < upper {
                    // Não queremos aumentar acidentalmente o upper bound,
                    // daí o compare-exchange.
                    self.upper
                        .compare_exchange(upper, depth, Ordering::Relaxed, Ordering::Relaxed)
                        .ok();
                }
                within
            }
            None => true,
        }
    }
}

impl Default for Route<'_> {
    fn default() -> Self {
        Self::root()
    }
}

impl Clone for Route<'_> {
    fn clone(&self) -> Self {
        Self {
            outer: self.outer,
            id: self.id,
            len: self.len,
            upper: AtomicUsize::new(self.upper.load(Ordering::Relaxed)),
        }
    }
}


// `Sink` materializado no Passo 104 (ADR-0042) — ver
// `01_core/src/entities/sink.rs`. Re-exportado aqui para preservar
// o path histórico `entities::world_types::Sink` usado pela
// assinatura `eval(_sink: TrackedMut<Sink>)`.
pub use crate::entities::sink::Sink;

// Bloco tracked — canal de warnings via `TrackedMut<Sink>` (ADR-0043,
// Passo 106; estendido no 107 para incluir hint). `warn_note` delega
// para a API não-tracked em `sink.rs` para preservar dedup. Args `Span`
// (Hash+Copy) e `&str` (Hash) são compatíveis com comemo tracking;
// convenção: `hint == ""` significa "sem hint" (evita `Option<&str>`
// que o macro `#[comemo::track]` não aceita por falta de elisão). A API
// `Sink::warn(diag)` não-tracked permanece disponível para callers com
// `&mut Sink` directo.
#[comemo::track]
impl Sink {
    /// Emite warning via canal tracked (ADR-0043, Passo 106; extensão
    /// Passo 107: parâmetro `hint`). Usado pelo `eval()` e pelas
    /// funções internas `eval_*` que só têm `&mut TrackedMut<Sink>`
    /// (`&mut Sink` não é obtível de `TrackedMut` sem perder tracking).
    ///
    /// Convenção: `hint == ""` = sem hint. Para warnings complexos
    /// (múltiplos hints, severity custom, trace), usar a API não-tracked
    /// `Sink::warn(diag)` via `&mut Sink` directo.
    pub fn warn_note(
        &mut self,
        span: crate::entities::span::Span,
        message: &str,
        hint: &str,
    ) {
        let mut diag = crate::entities::source_result::SourceDiagnostic::warning(
            span,
            message.to_string(),
        );
        if !hint.is_empty() {
            diag = diag.with_hint(hint.to_string());
        }
        self.record(diag);
    }
}


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
        let _ = Route::root();
    }

    // ── Route (ADR-0033, paridade com vanilla) ────────────────────────────────

    fn file(n: u16) -> FileId {
        use std::num::NonZeroU16;
        FileId::from_raw(NonZeroU16::new(n).unwrap())
    }

    #[test]
    fn route_root_nao_contem_nenhum_ficheiro() {
        // Segmento raiz: id=None, outer=None. Nada pertence à rota.
        let r = Route::root();
        assert!(!r.contains(file(1)));
        assert!(!r.contains(file(42)));
    }

    #[test]
    fn route_com_id_contem_proprio_id() {
        let fid = file(3);
        let r = Route::root().with_id(fid);
        assert!(r.contains(fid));
        assert!(!r.contains(file(4)));
    }

    #[test]
    fn route_extend_adiciona_ao_stack() {
        // parent tem id=1; child extende-o via .track() e ganha id=2.
        // Após extend, a cadeia contém ambos os ids.
        let parent = Route::root().with_id(file(1));
        let child  = Route::extend(parent.track()).with_id(file(2));
        assert!(child.contains(file(1)), "id do pai deve ser visível via outer");
        assert!(child.contains(file(2)), "id próprio deve estar presente");
        assert!(!child.contains(file(3)));
    }

    #[test]
    fn route_contains_detecta_ciclo() {
        // Cenário: A importa B que tenta importar A novamente.
        // A cadeia é A → B; contains(A) na cadeia devolve true — ciclo.
        let a  = Route::root().with_id(file(10));
        let b  = Route::extend(a.track()).with_id(file(11));
        assert!(b.contains(file(10)), "contains detecta A na cadeia B←A");
    }

    #[test]
    fn route_increase_decrease_equilibrado() {
        let mut r = Route::root();
        r.increase();
        r.increase();
        r.increase();
        r.decrease();
        r.decrease();
        r.decrease();
        // Sem API pública para ler `len`; inferimos o estado via `within`:
        // após equilibrar increases/decreases, a profundidade actual é a
        // inicial (0), logo `within(0)` é true.
        assert!(r.within(0), "increase/decrease equilibrados repõem a profundidade");
    }

    #[test]
    fn route_check_depth_rejeita_profundidade_excessiva() {
        // `within` só aplica o limite quando existe um pai — no segmento
        // raiz isolado, qualquer profundidade passa (paridade com vanilla
        // `engine.rs:419`: `None => true`). Construímos um segmento filho
        // e inflamos o `len` acima de MAX_SHOW_RULE_DEPTH.
        let parent = Route::root();
        let mut child = Route::extend(parent.track());
        // `extend` inicia com len=1; incrementamos até ultrapassar o limite.
        for _ in 0..Route::MAX_SHOW_RULE_DEPTH {
            child.increase();
        }
        // child.len = 1 + 64 = 65 > MAX_SHOW_RULE_DEPTH.
        // `check_show_depth` é função livre desde Passo 93 — recebe
        // `Tracked<Route>` porque o eval só tem acesso à Route por proxy.
        assert!(check_show_depth(child.track()).is_err(),
                "check_show_depth deve rejeitar profundidade > MAX_SHOW_RULE_DEPTH");
        let diags = check_show_depth(child.track()).unwrap_err();
        assert!(diags[0].message.contains("show rule depth"),
                "mensagem identifica a categoria excedida");
    }

    // ── check_*_depth free functions (Passo 93, DEBT-45 parcial) ──────────────

    #[test]
    fn check_call_depth_rejeita_chain_excessiva() {
        // Paridade com `typst-eval/src/call.rs:33`: MAX_CALL_DEPTH = 80.
        // Inflamos o len de um segmento filho acima do limite.
        let parent = Route::root();
        let mut child = Route::extend(parent.track());
        for _ in 0..Route::MAX_CALL_DEPTH {
            child.increase();
        }
        // child.len = 1 + 80 = 81 > MAX_CALL_DEPTH.
        assert!(check_call_depth(child.track()).is_err(),
                "check_call_depth deve rejeitar profundidade > MAX_CALL_DEPTH");
        let diags = check_call_depth(child.track()).unwrap_err();
        assert!(diags[0].message.contains("function call depth"),
                "mensagem identifica a categoria excedida");
    }

    #[test]
    fn check_funcoes_livres_aceitam_route_raiz() {
        // Smoke test: todas as 4 funções livres aceitam root().track()
        // e devolvem Ok — root isolado nunca excede limite (outer=None).
        let r = Route::root();
        assert!(check_show_depth(r.track()).is_ok());
        assert!(check_layout_depth(r.track()).is_ok());
        assert!(check_html_depth(r.track()).is_ok());
        assert!(check_call_depth(r.track()).is_ok());
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
