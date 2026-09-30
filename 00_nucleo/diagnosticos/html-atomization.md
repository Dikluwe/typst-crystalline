# Atomização do exportador HTML

Registro: 2026-09-26T09:39:39-03:00; HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado. [Manifesto de proveniência](html-atomization-manifest.json): stat global, untracked, SHA-256 dos consumers/owners e logs de validação.

## Resultado

`03_infra/src/export/html.rs` passou de 1355 para 79 linhas. Mantém HtmlSerializationMode, HtmlFrameResolver, entrypoints e adapter público do modo. Nove descendentes privados recebem a implementação e as duas suítes de testes, cada um com L0 proprietário.

As alterações locais preexistentes de Styled e SpaceFromNewline foram preservadas. Não houve mudança de API, default, algoritmo, mensagens de erro, fase, comportamento ou política de serialização. HTML continua semântico; SVG de frame vem do resolvedor da pipeline, sem layout no exporter.

| Módulo | Responsabilidade | Linhas |
|---|---|---:|
| html/document.rs | Envelope/raiz explícita e agrupamento de blocos | 175 |
| html/inline.rs | Content inline, HtmlElem e frames | 108 |
| html/table.rs | Tabelas semânticas e spans | 85 |
| html/whitespace.rs | Bordas, espaços protegidos e ruby | 176 |
| html/encoding.rs | Escapes contextuais, raw e title | 80 |
| html/pretty.rs | Tokenização e indentação estrutural | 174 |
| html/tags.rs | Classificações estáticas de tags | 65 |
| html/tests.rs | Regressões funcionais e novas caracterizações | 516 |
| html/p1360_ab_tests.rs | Suíte A/B pretty e modos | 59 |

## Validação

- Baseline: 18 testes existentes. Três caracterizações adicionadas antes da extração: Styled/raw/title, frame não resolvido/resolvido sem escape e tabela com header/spans/escapes. Os 21 testes passaram ainda no monólito usando harness isolado com dependências do lockfile do projeto.
- Após extração: 21 testes na crate real passaram, assim como quatro testes integrados P1424 e o build offline. Fixtures `pipeline/html` executadas em modo smoke; não são medição de desempenho.
- AST: 66 itens sem diferenças sob a normalização declarada no manifesto; inclui funções, constantes, structs/enums e alias público. Gate estrutural RED antes (descendentes ausentes), GREEN depois.
- Resselo oficial restrito aos dez pares consumer/owner, em cópia com preflight V15/V26 e check V5/V15/V26. Última passagem não teve nada a corrigir. Não foi aplicado reparo global.
- Validações globais e seus logs estão no manifesto; warnings/infos de checks não-bloqueantes não são tratados como zero violations global estrito de todos os checks.

## Limites e próximo refinamento

Preservaram-se também limitações do algoritmo atual: chunking de tabelas com spans, tokenizer pretty e subconjunto de Content suportado. Não houve ampliação de suporte ou correção incidental. A comparação AST não afirma paridade nova com vanilla. Qualquer otimização requer medição da mesma fixture/configuração; esta extração não demonstra speedup.
