# Prompt L0 — export HTML: encoding
Hash do Código: c64e7859

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/html/encoding.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

No snapshot anterior `03_infra/src/export/html.rs:612-629 possui raw_text; :706-735 title_text; :900-918 escaping contextual.`
A localização é mecânica L3 (ADR-0109); preservar a morfologia HTML e
mensagens de erro observáveis (ADR-0107), sem inferir intenção do produto.

## Responsabilidade e invariantes vigentes

Possui raw_text/title_text e escape_text/escape_attribute. Crystalline continua conservador, Vanilla contextual; preservar ordem das substituições. Raw aceita somente Empty/Text/Space/SpaceFromNewline/Linebreak/Sequence/Styled, sem escapar script/style. Title escapa somente & e <; linebreak vira newline e absorve um Space/SpaceFromNewline seguinte. Diagnóstico para filho não textual permanece idêntico. Nenhuma normalização de DOM, bytes ou whitespace adicional.

Módulo privado descendente de export::html; helpers usados entre irmãos
empregam pub(super), sem pub(crate), API nova ou mudança de fase. Mantém as
assinaturas, algoritmos e diagnósticos do snapshot medido, incluindo
adaptação local existente a Content::Styled(styled). A extração é interna
e segue fluxo contínuo ADR-0127; cada consumer possui owner 1:1 ADR-0129.

## Aceitação

Testes funcionais existentes e novas caracterizações passam antes/depois;
comparação de assinatura/corpo detecta deriva na movimentação. Não afirmar
ganho de tempo nem paridade nova a partir desta reorganização.

## Proveniência histórica das regras transferidas

As medições abaixo pertencem aos pins e passos históricos citados; não
constituem nova medição contra upstream atual. Estados de proposta e scope-out
antigos são superados pelos invariantes vigentes acima e cortes posteriores.

## Modos de serialização e escaping contextual

Este owner define:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtmlSerializationMode { Crystalline, Vanilla }

pub fn export_html_with_serialization(
    content: &Content,
    mode: HtmlSerializationMode,
) -> Result<String, SourceDiagnostic>;
```

`export_html(content)` equivale ao modo `Crystalline`. Esse modo preserva o
escape conservador vigente. `Vanilla` seleciona o escape contextual: atributo
normal escapa `&` e `"`; texto normal escapa `&` e `<`.

DOM ordenado, Presence, void, nesting, whitespace estrutural, documento
explícito, `title` escapable-raw, diagnósticos e subset suportado são comuns
aos dois modos. O modo não altera `Content`, `HtmlElem`, feature, target ou
fase e não é inferido por tag, texto, path ou estado global.

Testes distinguem os dois modos e provam DOM equivalente, além de preservar
void, whitespace e `title`.
