# A2 — fronteiras de bibliografia e tipo de hanging-indent

Esta subetapa foi concluída. **A2 e a paridade geral permanecem abertos.**
O corpo e o título da bibliografia deixam de começar na linha da citação
anterior. Valores inválidos de hanging-indent passam a produzir o erro de tipo
correto, inclusive por alias e quando falta o corpo do constructor.

## Correções

- Bibliografia fecha o segmento anterior com seu estilo causal e delega o corpo
  ao protocolo existente de Block, com dimensões automáticas e inset zero.
  O fallback conserva formato, ordem, prefixos e referências anteriores.
  Corpo vazio sem título não acrescenta margem nem quebra.
- set par identifica a função nativa efetiva, incluindo alias. set e constructor
  validam Length e conservam o span do valor e o hint de unidade do inteiro.
  O constructor valida antes de exigir body. Nenhum contrato Rust público,
  modo/flag ou ordem de realização foi alterado.
- Quatro L0s foram atualizados antes do código; três compartilham o novo núcleo
  paragraph-hanging-validation. Oito arquivos existentes mudaram neste lote
  (quatro fontes e seus quatro L0s), mais o núcleo. O patch relativo está em
  [change.patch.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-indent-2026-10-18/change.patch.gz).

## Validação

RED funcional preservado: tipos inválidos aceitos, corpo cacheado/fallback
na linha da citação e título fora da margem. A primeira tentativa GREEN
identificou um bypass por alias; a correção da identidade manteve os critérios.
Depois, **15 testes focados passaram**, oito novos mais sete controles CSL.

A validação completa passou: **8994 testes, zero falhas,
7 ignorados; cinco doctests**. Build workspace, Clippy
all-targets estrito, Rustdoc com warnings negados, quatro índices documentais,
fmt, linhagem V1/V5/V15/V26, reversa com 987 consumers e diff-check passaram.
Os 2093 hashes produtivos/L0 ficaram congelados na validação.
[Comandos/resultados](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-indent-2026-10-18/complete-results.json).

Nas 60 sondas bilaterais repetidas, os 42 casos bibliográficos agora separam
citação/título/corpo e mantêm o texto do oracle; os dez negativos antes aceitos
agora rejeitam com a mesma primeira mensagem. Os oito controles de parágrafo
comum conservam a geometria anterior: isso comprova ausência de regressão
nesses controles, **não** a implementação do recuo. Seis controles adicionais
cobrem aliases/precedência de erro. Os seis casos CSL customizados conferem em
texto; o título padrão, antes intercalado, fica na ordem correta. Os 44 controles
textuais R3 mantêm seus resultados e primeiro erro. A checagem adicional de
cores PDF confirma BIBWORD vermelho e OUTSIDE preto sob show-set bare/where;
o alias-style continua preto no Tekt contra vermelho no oracle, como antes.
[Auditoria](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-indent-2026-10-18/audit.json.gz).

| Tekt | Antes | Depois |
| --- | ---: | ---: |
| V16 (erro) | 105 | 105 |
| V19 (nota) | 1120 | 1120 |
| V20 (nota) | 876 | 876 |

Os detalhes de entradas acrescentadas/removidas estão em
[lint-diff.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-indent-2026-10-18/lint-diff.json.gz). Nenhum erro ou warning novo;
o gate global continua bloqueado pelos V16 ainda existentes.

## Geometria: ganho e diferença residual

Na MLA sem título, a primeira entrada passou de x41.84/y5.168 para
x8/y27.464; o oracle usa x8/y27.463995. O título padrão foi para
x8/y33.3128, como o oracle. Porém o corpo abaixo desse título fica em
y59.9 contra y54.499995: há aproximadamente 5.4 pt de espaço adicional.
As continuações permanecem em x8 contra x26 (default) ou x44 (override
3em) no vanilla. No IEEE, o corpo após o prefixo começa em x25.124
contra x29.924. Esses valores estão em before/after.json.gz, fonte 12 pt,
margem 8 pt, casos mla-none-base, mla-default-base, mla-none-relative
e ieee-none-base. A correção de fronteira não encerra essas diferenças.

## Limites e próxima ordem

1. **A2-Structure:** preservar hanging_indent, prefixo e corpo do CSL para
   compor entradas e colunas; o corpo ainda é um Content pré-formatado único.
2. **A2-HangingTransport / A2-ParIndent:** aplicar recuos válidos e sua prioridade
   por ocorrência, incluindo fonte relativa, zero, negativos e parágrafos
   comuns. O aviso existente em set continua; valores válidos ainda não têm
   implementação física completa. Não há declaração de paridade geométrica.
3. **A2-Multi:** resolver atribuição/cache de múltiplas bibliografias.
4. **Posições de query:** a divergência não é específica da bibliografia.
   Título de controle retorna x0/y0 no Tekt contra x8/y8 no oracle; o controle
   de block nem é encontrado no Tekt. Não inferir posição causal de um elemento
   a partir da caixa dos seus glifos nem declarar a causa já diagnosticada.
5. R3-T1 continua aguardando a aprovação já solicitada. A proposta selada
   permanece intacta; o L0 dos testes ganhou apenas o adendo deste lote.
   [Overlay de integração](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-indent-2026-10-18/r3-t1-overlay.json) determina preservar esse
   adendo ao aplicar a proposta aprovada. Os demais dez L0s-base não mudaram.
   E01–E20 e as frentes públicas do inventário geral permanecem no escopo.

## RAM e proveniência

Pico máximo medido dos jobs: **4364279808 bytes (4.065 GiB)**.
MemoryMax=12 GiB, High=8 GiB, swap=0, Cargo jobs1, debug0/incremental0;
nenhum evento high/max/OOM/OOM-kill. A validação completa executa os comandos
pesados sequencialmente no mesmo cgroup. Esse pico não é consumo total da
máquina nem prova pareada de ganho de memória.

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, branch `Tekt`, working tree não commitado.
Fontes iniciais, UTC, status e lista exata de alterações `git diff HEAD --stat`
estão em [before-sources.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-indent-2026-10-18/before-sources.json.gz); os recibos
registram cada comando/hora. [final-state.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/bibliography-indent-2026-10-18/final-state.json.gz)
registra o estado final, os 2084 arquivos-base
preservados e o pico. before/after.json.gz preservam fontes, binários, argv,
texto e caixas; os PDFs anteriores foram arquivados antes de sobrescrever.
O relatório e manifesto A2-CSL anterior foram verificados sem alterações.

Vanilla pinado bd5ebec285505889e8ff1d4f68804ab147a699f3, árvore
cac836c07e4dec2a89219370428292393a3216b5, binário
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Tekt anterior `97f6fa3078bdd6c40aabb80d6f93f3fd019b405ce70fbf2266686d8fb42271cb`;
Tekt validado `c2b1b2e93cf34f3c597a0a0fbbd51c8f8ac96a3c6bc0a4d9bd72373599acf899`.
Sem commit, push, merge, publicação ou release.
