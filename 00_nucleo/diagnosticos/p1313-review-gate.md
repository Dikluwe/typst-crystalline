# P1313 — parecer prévio do gate CSV/DataSource

## Proveniência e papel

Revisor `/root/p1313_review`, em 2026-09-08T11:26:19Z. Regime A/B de
préflight, executado sem atestação de isolamento técnico; nenhum selo de
refinamento ou veredito de implementação. Escrita autorizada somente em
`00_nucleo/diagnosticos/p1313-review*`. Não foram editados L0, código ou testes.
Nenhum candidato P1313 existia segundo a atribuição recebida. O contexto
herdado contém a proposta do coordenador, não um patch P1313.

HEAD `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, working tree não commitado.
Estado registrado antes deste parecer por `git diff HEAD --stat`:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  62 ++++-
 00_nucleo/prompts/compiler/stdlib/loading.md       | 125 ++++++++-
 01_core/src/compiler/eval/bindings/field_access.rs | 175 +++++++++++-
 01_core/src/compiler/stdlib/loading.rs             | 306 ++++++++++++++++++++-
 4 files changed, 658 insertions(+), 10 deletions(-)
```

Entradas identificadas por SHA-256 integral:

- `00_nucleo/prompts/compiler/stdlib/loading.md`:
  `824bccfbc996e73e9015814a7bab084e647b2a9e643012d6090efa42e0f5fd4d`.
- `00_nucleo/diagnosticos/p1312-verification.json`:
  `f2c9b6e18e6a4bd036ec3e740e096616a6b1a033829b99a0425891254dadd638`.
- `00_nucleo/diagnosticos/p1312-selection.json`:
  `59497dd309d5e6e36411d8efe215570f23c4255fd79dd0800a9470a1d3ebfbef`.

Lidos integralmente: CLAUDE.md, ADR-0127, loading.md e a skill
`/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md`, com
`references/papeis-e-capacidades.md` e `references/artefatos-e-gates.md`.
A busca textual restrita a `00_nucleo/adr/` não encontrou ADR de
materialização segregada. As pastas materialization/context não foram
abertas nem varridas.

## Evidência anterior à classificação

1. Na fonte vanilla ratificada pelo repositório em `a51e02804`,
   `lab/typst-original/crates/typst-library/src/loading/csv.rs:27–30`
   declara `source: Spanned<DataSource>`; `read.rs:24–27` declara
   `path: Spanned<PathOrStr>`. A documentação de CSV na linha 29 explicita
   caminho ou bytes crus. É evidência de intenção declarada. O funcionamento
   de `csv(bytes("a,b"))` consta da medição citada no L0 P1312; este parecer
   não o apresenta como nova execução independente.
2. `loading.md:229–237` determina Path/Str para read/csv e Path/Str/Bytes
   para os cinco outros decoders. P1141 termina com: “As assinaturas/casts
   públicos mudam; implementar somente após o gate P1141.” Esse precedente
   não permite presumir que toda mudança de cast seja interna apenas porque
   a assinatura Rust permaneça igual.
3. `loading.md`, seção P1312, manda preservar a rejeição CSV de Bytes/Symbol
   e exige revisão própria futura do contrato CSV/DataSource. O veredito
   P1312 é `PASS_SCOPED` de read. A seleção P1312 justifica seu fluxo contínuo
   expressamente também pela ausência de mudança dos tipos aceitos.
4. ADR-0127:27–40 exige paragem para assinaturas públicas e compatibilidade
   cuja reversão quebraria código consumidor. O item de comportamento por
   defeito exclui expressamente correções de paridade; portanto seria incorreto
   usar qualquer alteração visível, sozinha, como prova de paragem.
5. ADR-0127:48–53 permite correções de paridade em geral e mudanças cujo
   desfazer não quebra nada fora do módulo. ADR-0127:92–94 resolve a dúvida
   de classificação: “em caso de dúvida sobre a classe da mudança, a regra
   de bolso é **parar**”.

## Classificação e conflito

A proposta tem intenção de convergência para a linguagem vanilla, não de
invenção de uma nova modalidade do produto. O texto de fluxo contínuo
ADR-0127 §2.3 fornece argumento real para continuar; ele não deve ser omitido.
Nenhum campo, trait, assinatura Rust ou fase do pipeline mudaria segundo o
escopo apresentado. O diagnóstico de tipo inválido com origem causal, tomado
isoladamente sem ampliar entradas, é comparável ao lote interno P1312.

Entretanto aceitar `csv(Bytes)` amplia o domínio público da função Typst.
Depois da mudança, documentos podem depender dessa chamada; desfazê-la
reintroduziria a rejeição. Isso alcança uma leitura plausível do critério de
compatibilidade, e o próprio owner já classificou casts públicos como gate
em P1141. A ADR não explicita que a exceção de paridade sobrepõe também essa
situação. Tampouco há base para afirmar que P1141 prova, sozinho, que qualquer
cast futuro exige confirmação: aquele lote também envolveu outras mudanças.
Existe, portanto, dúvida normativa concreta entre exceção geral de paridade,
compatibilidade ao reverter e precedente do owner. A ausência de alteração
Rust não elimina essa dúvida.

**Veredito: `GATE_REQUIRED_BEFORE_CODE`, pela regra explícita de dúvida da
ADR-0127, reforçada pelo precedente de casts públicos no L0.** Não classificar
silenciosamente a ampliação Path/Str → Path/Str/Bytes como mera correção de
mensagem. O parecer não afirma que a ADR exclui toda ampliação de inputs do
fluxo contínuo; afirma que a autorização normativa deste caso não está
inequívoca nas entradas examinadas.

## Ação autorizada antes da confirmação

Concluir as medições e redigir a emenda concreta do owner L0 e seu passo,
com hashes e fronteiras verificáveis. Depois apresentar ao dono a mudança
CSV/DataSource e o gate; não iniciar código produtivo que aceite Bytes antes
da confirmação. O pedido informado “Escreva o próximo e implemente” autoriza
preparar essa proposta e manifesta intenção de implementação, mas foi feito
antes de o L0 P1313 existir e, nas entradas disponíveis, não contém dispensa
expressa do gate nem aprovação informada do conflito acima. Uma aprovação
anterior específica fornecida pelo dono refutaria essa avaliação de autorização.

A proposta deve preservar read/P1310, explicitar a substituição estreita da
preservação CSV de P1312, tratar Symbol como dívida separada, conservar as
opções e a ordem normativa de validação, e distinguir erro de cast de erro de
decode. Não alegar paridade geral CSV apenas por aceitar bytes em RAM. Nenhuma
mudança deste parecer substitui o gate humano nem valida tecnicamente a futura
implementação.
