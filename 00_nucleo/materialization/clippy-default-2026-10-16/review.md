# Proposta: Default para três construtores públicos

Medição: os três tipos oferecem new() sem argumentos e ainda não implementam Default. A inspeção das fontes mostra que as duas sessões nascem vazias sem I/O, e EvalContext::new define valores não equivalentes a derivar Default campo a campo (orçamento e apply_show_rules). Fontes/L0 vigentes e proveniência em base.json.gz; os quatro L0 propostos estão em proposed/ e o delta em proposal.diff. Os owners ativos e o código não foram alterados por esta proposta.

Adicionar somente `Default` a `EvalContext`, `FontCatalogSession` e `SourceSession`, sempre delegando a `Self::new()`. Nenhuma remoção de new(), mudança de campo, valor, API existente, default do produto ou fase. Não é uma nova política de criação automática de recursos; a sessão Default continua vazia e lazy. Chamadas atuais continuam válidas; novos consumidores podem usar `<T as Default>::default()` ou bounds `T: Default`. Remover estes impls depois quebraria esses novos consumidores.

## Aceitação concreta

- D1: teste de uso público por Default deve falhar antes da implementação e passar depois para os três tipos.
- D2: EvalContext Default preserva configurações/limites, e mutar uma instância não contamina outra; testes pertencem ao owner de eval/tests.
- D3: FontCatalogSession Default inicia sem catálogo e mantém aquisição lazy/isolamento; controles de seleção/gerações permanecem.
- D4: SourceSession Default inicia vazia e conserva parsing/retenção/limites e independência entre sessões/Worlds.
- D5: Clippy selecionado, suíte workspace, build, Rustdoc, formato e linhagem passam; sem supressão de lint e sem alterar os esperados existentes. RAM máxima 12 GiB, swap zero e um job.

A [ADR-0127 §2](/repos/Antigravity/typst-crystalline/00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md) exige confirmação para contratos públicos e para mudanças cuja reversão quebra consumidores. Citação de §2.4: “qualquer mudança que, se revertida depois, quebraria código já escrito contra ela (binário/API)”. Aplicamos esse critério aos novos impls públicos. A confirmação abrange somente os três impls, quatro L0 e D1–D5; não inclui release ou outras alterações de traits/formatadores/entidades.
