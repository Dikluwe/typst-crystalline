# Auditoria — argumentos de text e tamanho relativo

Auditoria concluída; **nenhum código, L0 ou núcleo vigente foi alterado**. A implementação TS1 depende de aprovação pública. O objetivo geral continua aberto, com 125 V16 e os demais defeitos registrados. Não houve commit/push, merge ou release.

## Medição e classificação

Foram medidos **64 pares / 128 invocações**: 48 casos principais e 16 suplementares, incluindo controles e repetições. **45 pares divergem** nos observáveis definidos; 19 coincidem nesses observáveis, o que não certifica paridade universal. Critérios: exit/mensagens de erro e hints, páginas e palavras exatos; dimensões de página/xMin/yMin/yMax com tolerância de 0.002 pt. Cor/pintura não foi certificada pelos bboxes. Todos os inputs, argv, stderr e PDFs/XML estão arquivados.

- Constructor não reconhece tamanho posicional: as seis permutações Length/Color/body, body antes do tamanho e alias são rejeitados indevidamente.
- Tamanho relativo perde em: 2em vira 0pt contra 24pt;2em+3pt vira 3pt contra 27pt; nested18pt→2em vira 0pt contra 36pt. Set posicional é ignorado; múltiplos sets relativos e show-heading também divergem.
- Nomeado deve impedir a busca posicional daquele tipo; o posicional restante vira body ou argumento excedente, conforme a ordem observada. Body None é válido e está sendo rejeitado; mensagens de body ausente, tipo inválido e sobras divergem. Duplicatas sintáticas já são rejeitadas corretamente pelo parser.
- fill:none é aceito indevidamente; Gradient é rejeitado apesar de aceito pelo pin; Gradient relativo self exige erro/hint próprio. Paint textual é frente separada: não substituir gradiente por cor sólida.
- show text é rejeitado como seletor; isso impede controles relativos nesse caminho. Esse suporte requer seu próprio owner/contrato, além do tamanho.
- A segunda medição **refutou a hipótese de identidade pelos valores declarados**: vanilla considera text(size:2em)[A] igual a text(size:24pt)[A], enquanto Tekt não. content/mod.rs:779–782 do pin compara apenas child entre Styleds. Styled versus conteúdo cru continua distinto nos controles; isso não autoriza apagar todos os wrappers. Igualdade da linguagem não é PartialEq/ObservationEq Rust e precisa de correção específica.

## Proposta pública concreta TS1

[Revisão](../../materialization/text-size-length-2026-10-17/review.md), [API](../../materialization/text-size-length-2026-10-17/api.md), [escopo](../../materialization/text-size-length-2026-10-17/scope.json) e [S1–S16](../../materialization/text-size-length-2026-10-17/test-plan.md).

Dois contratos: **Style::Size(Pt) → Size(Length)** e **StyleDelta.size: Option<f64> → Option<Length>**. Preservar abs+em no backing único e dobrar funções afins antes de resolver Pt. TextStyle.size e StyleChain::size mantêm superfícies resolvidas. Migração externa exige Length::pt para valores absolutos. Isso aciona ADR-0127 §2.1/§2.4; não foi implementado antes da confirmação.

O pacote tem 19 owners/candidatos e um núcleo TOML proposto com três claims, pinado em quatro L0 candidatos. Nenhum código aponta para núcleo; candidato não é L0 vigente. Corpus não é usado como prova de implementação inexistente. Plano S1–S16 cobre RED, API, transporte, captura, Raw/math, restauração, diagnósticos, memória e gates completos, sem release.

Memória atual medida pelo probe existente: Length 16bytes; Option<Length> 24; Option<f64> 16; Style 32; StyleDelta/Styles 328. Aumento de payload do campo 8bytes; tamanho futuro 336bytes é **estimativa**, não medição materializada. Pico e alocações precisam ser medidos após aprovação/implementação.

Há quatro caminhos em comum com a proposta RG1 (apply e layout/tests, código+L0). RG1 e seu manifesto continuam intactos. TS1 não a aprova; a retomada deve rebasear os adapters preservando ambos os contratos e revalidando testes. Publicar proposta não equivale a publicar código/release.

Manifesto TS1: **771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba**.

## Proveniência e recursos

HEAD **06848cdcb5450f2c555626bf116bc7acd82bd82b**, branch Tekt, working tree não commitado. before.json.gz registra status/diffstat e hashes dos 2119 arquivos de fonte/L0/configuração do checkpoint; as medições têm UTC, argv e SHA de binários/inputs/fontes. audit-state.json.gz revalida que todos permanecem iguais. Oráculo 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. Probe sizes.json guarda rustc, rlib, fonte e executável medidos. As medições descrevem o working tree congelado, não somente o HEAD.

Execuções pesadas sequenciais com MemoryMax 12 GiB, MemoryHigh 10 GiB, swap zero e Cargo jobs=1. Pico desta auditoria **481550336bytes (0.448GiB)**, sem OOM/OOM-kill. Este pico não é uma medida da implementação proposta e não deve ser comparado ao da suíte anterior como melhoria de RAM.

A suíte 8792/0 falhas/7 ignorados, Clippy, Rustdoc e linhagem da etapa Par/Box continuam evidência da mesma árvore produtiva, conferida por hashes. Não foram apresentados novos testes GREEN de uma API ainda não implementada. Enquanto TS1 aguarda aprovação, correções independentes de V16 podem continuar; igualdade Styled, Paint, show text, Box fracionário, SVG multipágina e RG1 permanecem abertos.
