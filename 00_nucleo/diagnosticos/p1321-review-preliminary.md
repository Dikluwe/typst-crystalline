# P1321 — revisão independente anterior ao L0 candidato

Revisor `/root/p1321_review`, 2026-09-08T20:13:59Z. Regime: A/B executado
sem atestação técnica de isolamento, não protocolo completo nem selo. Li
integralmente a skill tekt-materializacao-segregada e suas duas referências,
ADRs 0107/0108/0127/0129, `01_core/CLAUDE.md` e o L0 vigente inteiro.
A busca por `segrega` em `00_nucleo/adr` não encontrou ADR local específica.
Minhas escritas estão limitadas a `00_nucleo/diagnosticos/p1321-review-*`;
produto, baseline e oráculos são somente leitura. Não recebi oráculos privados.

## Proveniência

HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
Os únicos arquivos rastreados modificados são `loading.md` e `loading.rs`;
`git diff HEAD --stat -- <esses caminhos>` registra respectivamente
92 linhas de delta e 247, total 306 inserções/33 deleções. Esses deltas
precedem P1321 e constituem P1319, não código candidato desta revisão.

- L0 `00_nucleo/prompts/compiler/stdlib/loading.md` SHA-256
  `1bd9633f0cdfb5a1ef7ca0265b55ac7ea8ad5b28c41035a2aaaa3572edff0dd1`.
- Owner `01_core/src/compiler/stdlib/loading.rs` SHA-256
  `15078b5441b514a200c54039589521cb4755b99a99a68b5a37983c247f759800`.
- Baseline `/tmp/p1319-target.VqXtmj/release/typst` SHA-256
  `37a8a23d6e2b5daf355d90d510bca7efb399547730e1d807e8ae871be75748bd`.

## Achado e classe

No owner, `native_csv` rejeita unknown named pelo mapa antes de cast/opções,
não rejeita positional excedente antes do parser e `arg_csv_source` emite
missing detached com mensagem antiga. O L0 P1313–P1319 preserva expressamente
essa ordem: editar somente código violaria a obrigação vigente.

Na referência ratificada `a51e02804`, `loading/csv.rs:27–46` declara source,
delimiter e row-type nessa ordem; `foundations/args.rs:150–175` determina
missing e conselho para named source; `args.rs:218–235` valida todas as
ocorrências da opção antes de reter a última; `args.rs:259–266` rejeita
primeiro remanescente. `typst-macros/src/func.rs:391` emite finish antes do
corpo. Essa é evidência da intenção de validação, além do comportamento.

É viável corrigir localmente em loading sem nova entidade/trait/API pública
Rust: já existe `Args::occurrence_sequence()` e consumo equivalente privado
nos encoders textuais (`loading.rs:298–359`). Isso é inferência de suficiência,
refutável por perda de origem/ordem no carrier existente. Não autoriza alterar
dispatch/Args para encobrir eventual perda. Diagnóstico integral é observável
da linguagem na exceção ADR-0108. A classe é correção de paridade contínua
ADR-0127; não há novo modo padrão, ampliação do cast, fase ou API proposta.

## Condições substantivas do próximo gate

O L0 deve substituir explicitamente as preservações históricas de ordem,
missing/unknown/excesso e somente essas fronteiras. Deve distinguir named
source sem positional (conselho positional, span do named completo) de named
source junto a positional (remanescente), e missing de erro nas opções.
Cast precede delimiter, todas delimiter precedem todas row-type, e ambas
precedem primeiro remanescente causal. Este último pode ser unknown ou
segundo positional, mesmo se unknown aparece antes da fonte consumida.
Remanescente deve vencer I/O e parsing, incluindo Bytes malformados.

Testes/freeze precisam incluir origens distintas, detached explícito,
Args sintético, With/spread e erro inválido pré-ligado sobreposto por válido.
Symbol deve conservar rejeição normativa; não pode virar paridade alegada.
Sem erro de argumento, parsing e resolução devem continuar os de P1319.

Sem GO para patch neste instante: faltam o L0 congelado, delta explícito de
expectativas históricas, RED genuíno e freeze independente A/B. O fechamento
exigirá também hashes direto A (com Núcleo) e reverso B recalculados fora do
linter, por falso negativo B já documentado em P1319. Esta primeira inspeção
produziu fronteiras concretas; ainda não houve revisão focal repetida.

## Incidente exploratório comunicado após a inspeção

O coordenador informou que `/root/p1321_ab` leu incidentalmente `before.diff`
P1319 ao abrir saídas JSON P1320. Nenhum candidato P1321 existia, mas essa
entrada estava fora da allowlist do testador. O papel foi encerrado sem
freeze. Seus oráculos não serão usados como aceitação. Foi iniciado
`/root/p1321_ab_clean` com contexto vazio, proibição de todos os diagnósticos
anteriores e escritas `p1321-ab2-*`; somente seus artefatos congelados serão
considerados nesta revisão. Não li saídas privadas do papel descartado.
O reinício limpa as entradas declaradas, mas não atesta tecnicamente
isolamento num filesystem compartilhado.
