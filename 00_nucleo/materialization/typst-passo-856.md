# Prompt — typst-passo-856: completar a validação de `ref(<label>)` — casos de erro (resto do achado #63 / P853)

**Origem**: P853 implementou aceitação de `Value::Label` em `ref()`, mas só validou o caminho feliz (label existente e numerado). O prompt original pedia explicitamente testar label existente-mas-não-referenciável e label inexistente, comparando mensagens com o vanilla — isso não foi feito.
**Estado**: aguardando execução — código já existe, isto é fechar a validação que ficou faltando, não reimplementar.

---

## O que já está feito (não refazer)

`native_ref` aceita `Value::Label`, convertendo para o nome usado por `Content::reference_with_supplement`. `#ref(<label_existente_e_numerado>)` já funciona e está testado.

## O que falta

Dois casos de erro que o prompt original pedia e não foram cobertos:

---

## Passo 1 — Sonda dos casos de erro

1. **Label inexistente**: `#ref(<naoexiste>)` (label que não aparece em nenhum lugar do documento) — comparar a mensagem de erro exata do vanilla com o comportamento atual do cristalino.
2. **Label existente mas não-referenciável**: `#ref(<lbl>)` onde `<lbl>` está anexado a um elemento que não tem numeração/não é o tipo de coisa que `ref()` normalmente aponta (ex.: um parágrafo comum, ou um heading sem `numbering:` ativo — confirmar no vanilla qual é exatamente a condição de "não referenciável"). Comparar a mensagem de erro.
3. Confirmar em qual camada essas duas validações acontecem no cristalino hoje — o relatório de P853 já observa que "a validação pós-resolução... continua a ser responsabilidade do layout/introspector", então é provável que os dois casos já sejam tratados em algum lugar (não necessariamente com a mensagem certa) — não assumir que está totalmente ausente sem checar.

## Passo 2 — Corrigir se houver divergência

Se as mensagens não baterem com o vanilla, corrigir o texto (e o ponto do pipeline, se a validação estiver acontecendo no lugar errado — por exemplo, se antes P853 rejeitava tudo cedo demais e agora deveria deixar passar para a validação de layout decidir).

## Passo 3 — Validação final

Os dois casos de erro batendo com o vanilla (mensagem e comportamento). Confirmar que o caminho feliz (já testado por P853) continua sem regressão. Suíte completa — **usando `cargo test --workspace` de verdade, com as contagens discriminadas por crate** (`typst-core`, `typst-infra`, `typst-shell`, os demais), não um número único.

## Relatório

`00_nucleo/diagnosticos/typst-passo-856-relatorio.md` com os dois casos de erro medidos e corrigidos (se necessário), e a validação final com contagens de teste discriminadas por crate.
