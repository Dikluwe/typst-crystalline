# Prompt L0 — entities/lang
Hash do Código: 402212db

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/lang.rs`
**ADRs relevantes**: ADR-0052 (materialização), ADR-0033 (paridade vanilla),
ADR-0037 (coesão por domínio)

## Contexto

`Lang` é o identificador de língua natural. Materializado no Passo
131B para obter paridade funcional com o Typst vanilla —
captura `#set text(lang: "pt")`, valida formato ISO 639-1/2/3
(2 ou 3 letras ASCII), emite erro hard com mensagem literal em
valores inválidos.

Réplica estrutural exacta de `typst::text::lang::Lang` vanilla:
newtype compacto (3 bytes ASCII padded + length discriminator),
Copy, zero alocação.

Diagnóstico prévio:
`00_nucleo/diagnosticos/diagnostico-lang-passo-131a.md`.

## Interface pública

```rust
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Lang([u8; 3], u8);

impl Lang {
    pub const ENGLISH: Self;              // único constant inicial
    pub fn as_str(&self) -> &str;         // slice ASCII sem padding
}

impl std::str::FromStr for Lang {
    type Err = &'static str;
    // Aceita 2-3 letras ASCII; normaliza lowercase.
    // Erro: "expected two or three letter language code (ISO 639-1/2/3)"
}
```

## Semântica

- **Forma interna**: 3 bytes ASCII (padded com `b' '` se
  2-letter) + `u8` length (2 ou 3).
- **Representação canónica**: lowercase, sem padding trailing
  no `as_str()`.
- **Validação**:
  - 2 ou 3 caracteres.
  - ASCII only (7-bit).
  - Letras, dígitos, qualquer ASCII printable aceite
    (vanilla relaxed).
  - Hyphen (`"en-GB"`) → rejeita (length 5).
  - Não-ASCII (`"日本"`) → rejeita.

## Constantes

Apenas `Lang::ENGLISH` na materialização inicial. Vanilla tem
~260 constantes — cristalino adiciona on-demand quando
consumer (shaping, hyphenation, translations) exigir.

## Critérios de Verificação

```
Dado Lang::from_str("pt")
Quando chamado as_str()
Então "pt"

Dado Lang::from_str("PT")
Quando chamado as_str()
Então "pt"                      (case normalizado)

Dado Lang::from_str("fil")
Quando chamado as_str()
Então "fil"                     (3-letter, sem padding)

Dado Lang::from_str("")
Quando called
Então Err("expected two or three letter language code (ISO 639-1/2/3)")

Dado Lang::from_str("en-GB")
Quando called
Então Err(...)                  (length 5, rejeita)

Dado Lang::ENGLISH
Quando chamado as_str()
Então "en"
```

## Não incluído (deferido)

- `Lang::dir() -> Dir` — requer tipo `Dir` não materializado.
- Constantes além de `ENGLISH` — on-demand.
- Tipos `Region`, `WritingScript`, `Locale` — passos dedicados.
- Hint "put region in region parameter" para `"en-GB"` —
  requer `Region`.
