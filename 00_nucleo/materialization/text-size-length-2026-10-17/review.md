# Revisão TS1 — tamanho textual Length

**PROPOSTO, aguardando aprovação; nenhum código/L0/núcleo vigente foi alterado.**

## Medição anterior à proposta

O diagnóstico [text-argumentos-2026-10-17](../../diagnosticos/text-argumentos-2026-10-17/relatorio.md), audit.json e seus dois corpora registram **64 pares**, HEAD`06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC/status/diffstat e fonte/binário exatos. Pin vanilla9f2b6e8715237cb086899a42873660fe744622e8, SHA7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

Size18pt posicional é rejeitado; named2em vira0pt em vez24;2em+3pt vira3 em vez27; nested18×2 vira0 em vez36. Constructor armazena somente abs; StyleChain também corta em e collapse faz top-wins. Pin usa busca tipada em qualquer posição, nomeado com preferência, e fold afim. Igualdade Styled também diverge, mas a hipótese de que o pin compara valores declarados foi refutada e não sustenta esta decisão.

## Proposta

[api.md](api.md) muda exatamente **dois contratos públicos**: `Style::Size(Pt)` para `Style::Size(Length)` e `StyleDelta.size: Option<f64>` para `Option<Length>`. TextStyle.size continua Pt resolvido; Length já existe. Constructor/set/capture/chain preservam a contribuição completa até o ponto de resolução já existente. Nenhuma fase é movida.

Preferida: um tamanho declarado tipado no backing único. A alternativa de transportar a parte relativa por custom enquanto o typed size permanece escalar exige resolver duas representações e preservar separadamente a presença semântica do constructor. Ela não foi selecionada como atalho compatível. O canal custom de set continua vigente, mas é dobrado pela mesma política; não se cria campo paralelo ou valor escalar fictício.

Literais/chamadas Rust externos precisam migrar conforme api.md. A ADR-0127 §2.1 exige parada para “alterar campo público” e §2.4 para compatibilidade. A confirmação cobre implementação e [S1–S16](test-plan.md), sob12GiB/swap0, sem publicação/merge/release.

Memória: payload Option<Length>24 versus Option<f64>16bytes, aumento8bytes por campo; StyleDelta atual328bytes.336 é estimativa, não medição da API nova. Medir após materialização.

## Artefatos e escopo

scope.json contém 19 consumers: 5 semânticos, 4 adapters produtivos e 10 exclusivamente fixtures. Cada um tem seu owner exclusivo, copiado como candidato. O novo núcleo `00_nucleo/prompts/_nuclei/text-size-affine.toml` contém apenas invariantes compartilhados e é pinado em quatro candidatos; nenhum código aponta para núcleo. Hash do Código antigo nos candidatos não legitima implementação futura.

Interseção com RG1: 4 caminhos, listados em scope.json. TS1 não aprova RG1. Seus contratos/candidatos e manifesto permanecem intactos; quando ambos forem aprovados, rebase deve preservar ambas as alterações e revalidar os controles, sem sobrescrever o working tree nem ressellar fontes não autorizadas.

Após aprovação: atualizar L0s vigentes/pins primeiro, RED funcional, implementação/migração, resselo só dos owners, validação integral e relatório. Mudança pública adicional exige nova revisão. Se a compilação revelar outro consumer da mesma migração, auditar seu L0 antes de adaptar, registrar a extensão e preservar a mesma superfície pública aprovada.

Paint textual, seletor show text e igualdade da linguagem sobre Styled não são certificados por esta proposta. Permanecem na lista global de correções, com medições próprias.
