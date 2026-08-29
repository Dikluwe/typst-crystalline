# P1245 — gap público de tiling

**Veredito:** `L0_HASHES_CONFIRMED_AWAITING_SEGREGATED_PRESEAL`.

## Medição antes da decisão

No cristalino, `Tiling` contém `body`, `size`, `relative` e `spacing`
(`01_core/src/entities/tiling.rs:22-31`). `TilingBody` é fechado em
Image/Gradient/Color (`:61-68`), e o default físico é `relative=Itself`
(`:33-41,71-77`). O construtor aceita Color, Image e path, rejeita Gradient e
analisa apenas `size`, `spacing` e `relative`
(`01_core/src/compiler/stdlib/visualize.rs:27-113,116-177`).

No vanilla ratificado `a51e02804`, o valor materializado guarda `Frame`, size,
spacing, offset, angle e relative
(`lab/typst-original/crates/typst-library/src/visualize/tiling.rs:59-76`). O
construtor recebe conteúdo arbitrário, resolve `size:auto` a partir do frame,
aceita offset relativo e angle, valida finitude e aplica translação antes da
rotação (`:99-185,250-315,335-376`). Isso é semântica/morfologia da linguagem,
não estrutura Rust obrigatória (ADR-0107).

O inventário encontrou 20 correspondências textuais: 17 consumers produtivos
selecionados e três ocorrências de declaração/plumbing/teste explicitamente
excluídas. Os consumers que
materialmente mudam são a entidade, o construtor/dispatcher, os adaptadores de
Paint/Value, a fase de layout que produz o frame e os exporters que o consomem.
A lista auditável está em `p1245-tiling-consumers.tsv`.

## Opções

1. **Entidade declarativa + layout existente (proposta):** `Tiling` aceita
   Content, offset, angle e relative auto/self/parent, mas permanece não
   resolvido. A fase de layout existente materializa uma representação privada;
   não nasce um segundo contrato público `ResolvedTiling`.
2. **Contrato público resolvido:** expor frame/tiling resolvido como novo tipo
   público. Rejeitada porque amplia a API e copia mecânica vanilla desnecessária.
3. **Resolver em cada exporter:** menor localmente, mas duplica layout, mistura
   fases e não resolve `size:auto` com autoridade única. Rejeitada.
4. **Rasterizar no construtor/exporter:** reduz a superfície imediata, mas perde
   morfologia, acessibilidade e independência de target. Rejeitada por ADR-0107
   e ADR-0128.

A alteração pública mínima semanticamente completa proposta é a opção 1.
O dono confirmou explicitamente os dois hashes atuais. Adicionar só
offset/angle não fecha conteúdo arbitrário nem `size:auto`; adicionar só Content
não fecha placement. O rascunho de deltas L0 está em
`p1245-tiling-l0-draft.md` e não é um Prompt L0 aprovado.

## Gate

A opção proposta adiciona/altera campos públicos, muda a superfície do
construtor e explicita a fronteira eval→layout existente. A confirmação humana
ADR-0127 foi registrada em `p1245-owner-confirmation.tsv`; os L0 proprietários
foram propostos primeiro. O fluxo para agora no preseal segregado. ADR-0129 exige
owners 1:1; claims realmente comuns entre entidade, stdlib, layout e exporters
podem virar Núcleo Tekt pinado, mas nenhum código pode apontar para esse Núcleo.

P1243 está `PRESEAL_INVALIDATED_NEGATIVE_BOUNDARY_RETAINED` e P1244 foi fechado
sem código. Mesmo após eventual aprovação arquitetural, a cadeia deverá selar
contrato/oráculos próprios antes de produção.

## Proveniência

- HEAD `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`.
- Working tree não commitada: 59 ficheiros tracked, 2247 inserções, 188 remoções.
- Medido em `2026-08-28T07:21:40-03:00`.
- Nenhum código, whitelist ou header produtivo foi alterado. Foram atualizados
  apenas os L0 `entities/tiling` e `stdlib/tiling`.

## Selo documental pendente

- `00_nucleo/prompts/entities/tiling.md`:
  SHA-256 `fef0967145b237b4b5e8df7999e4c5664fa9ea2d468c4263a767772232642696`.
- `00_nucleo/prompts/compiler/stdlib/tiling-stdlib.md`:
  SHA-256 `080487e1bc80b7a27c67c650c9d6c55f25669835c3ff9de0aea7380fe43df690`.
- Estado: `L0_HASHES_CONFIRMED_AWAITING_SEGREGATED_PRESEAL`; implementação
  P1254 proibida até selo segregado válido de contrato/oráculos.
