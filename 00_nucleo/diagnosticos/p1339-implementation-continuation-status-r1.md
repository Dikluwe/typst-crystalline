# P1339 — integração em andamento e pré-requisito fora do contrato

Autor: `/root`, implementador/integrador; não é veredito independente.
Regime: **executado sem atestação de isolamento**. Não houve commit.

## Evidência antes da decisão

As execuções abaixo foram realizadas sobre HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4` com working tree não commitada.
Cada recibo conserva UTC, comando, canais integrais, estado, lista exata de
alterações (`git diff HEAD --stat`) e hashes dos arquivos no momento da execução.
Não representam a revisão final, pois a integração continua incompleta.

- `p1339-implementation-integrated-local-r1.json`, SHA-256
  `c1761190509b2fbaeecf33f39ce3a02bc34c4b984874004e7e290690a72efdc9`:
  16 testes passaram; o teste local de bytes falhou porque seu invólucro
  `array(...)` chega a `type array does not have a constructor`.
- `p1339-implementation-numerics-local-test-correction-r2.md`, SHA-256
  `cf0c05bec668700d5a25cc88f43599b72ecd7e7a8e1962120aa805eed5233993`:
  documenta a remoção dessa dependência apenas do teste local, preservando
  exigência de `Value::Bytes`, bytes exatos, endian e sinal. Seus recibos
  registram 11 testes float e 24 version passando; as repetições não são
  contadas como cobertura nova. Nenhum oráculo externo foi alterado.
- `p1339-implementation-pipeline-local-red-r1.json`, SHA-256
  `2bad9f7365677def5718f400c1d91ac11d67b454afe7eff5c1e8a648099d2ed9`:
  o controle de erro independente passou; o teste de leitura anterior a
  update contextual posterior falhou com `(0,)` em vez de `(3,)`.
  Os testes estão em `03_infra/src/pipeline.rs`; a implementação produtiva
  do ciclo seletivo ainda não foi adicionada.
- `p1339-implementation-pause-check-r1.json`, SHA-256
  `77d03045f3c4926e64444b1d3715a1da250e3f04cf782a0445caee9ad9fe24b5`:
  `cargo check -p typst-core -p typst-infra --locked` terminou com exit 0,
  sem alteração dos sources durante a execução. Compilação não é prova
  funcional nem substitui os gates finais.

## Impedimento substantivo

A análise independente identificou a mesma dependência em `array(...)`
nos casos públicos congelados. O dispatcher de construtores não possui a
ramificação `Type::Array`. A seção P1339 do L0 de call_dispatch proíbe
alterar construtores: acrescentar esse comportamento não é um adaptador
mecânico autorizado pelo selo atual.

O RED de descoberta podia falhar antes dessa chamada, pela ausência de
`float.to-bytes`, e portanto não demonstrava que todos os pré-requisitos
do programa composto estariam disponíveis depois da implementação.
O GREEN do teste local isolado não resolve essa lacuna dos casos públicos.

O parecer `p1339-verifier-array-prerequisite-gap-r1.json`, SHA-256
`1f8c66dd50aa592d0e1924580b5569458bde971d13ee8b6395301e788a34957e`,
registra 70 casos positivos afetados, correspondentes a 840 células finais.
Recomenda autorização estreita de conversão Array a partir de Bytes, sem
incluir Array a partir de Version ou um construtor geral. A primeira
premissa afetada é a viabilidade B/C; a falha foi observada na integração
de F. Requer sucessão normativa e budget prospectivo, sem apagar custos
ou resultados anteriores.

Não foi implementado Array, não foram trocadas as entradas públicas e não
foi reinterpretado Unknown como sucesso. O parecer independente delimita
o recorte afetado e a sucessão necessária. A conclusão do passo requer
decisão explícita sobre essa ampliação ou uma revisão legítima dos casos,
com revalidação da parte afetada; não apenas um novo hash.

## Integração preservada

Foram integrados os owners de float/version, descoberta e aplicação de
métodos, selectors/where, matching, igualdade e representação; o registro
de leituras cobre os caminhos implementados de contadores, estado, query
e localização. A captura por requisição inclui estilos, arquivo e regras/
guards. `apply_func` restaura essa captura também quando um constructor
de elemento retorna erro, sem retorno antecipado pelo operador `?`.

O adaptador de estilos recebeu apenas a sucessão mecânica aceita em
`p1339-verifier-seal-F-style-supersession-r1.json`, SHA-256
`b52144ed17ca14a154bf61141da5e98501901a3d6485e68f3460b552a68c29d8`.
Originais e expectativas permanecem intactos. A ligação real dos testes
instrumentados ainda precisa do seu gate independente.

A compilação instrumentada delimitada seguinte falhou antes de runtime:
`p1339-implementation-observer-instrumented-compile-r2.json`, SHA-256
`3c4b1e39a5f9ee3328fc06b04f44fd1c419eb8b6b3e85297086df2d334dc44e0`.
O adaptador sucessor usa `TrackedMut::reborrow_mut` como método em vez de
função associada, produzindo cinco E0599. O recibo conserva os canais e
a identidade inalterada dos sources. Precisa de correção mecânica por sua
autoridade e nova revisão; não houve execução interna F nem crédito de
sucesso. O implementador não alterou o harness protegido para contornar
esses erros.

O handoff `p1339-implementation-observer-handoff-r1.md`, SHA-256
`571d9129add88450225550faf4feb91e2ff35bc9f5f67d241fb0ec8261937cf2`,
explicita uma limitação adicional de proveniência dessa compilação: uma
newline extra foi removida da cópia do thin wrapper durante o build.
O recorder acompanhava os sources produtivos, não essa cópia diagnóstica;
portanto a execução não prova imutabilidade de todos os inputs. Uma futura
compilação deve pinar também o wrapper efetivamente incluído.

## Trabalho restante, sem alegação de fechamento

- Resolver o pré-requisito de Array e sua fronteira normativa antes de
  alterar contrato, L0 ou oráculos protegidos.
- Implementar o ciclo seletivo paginado: descoberta real, seed vazio,
  retenção por geração e recursos, descendentes, sinks transacionais,
  snapshots completos, orçamento compartilhado e diagnóstico final.
- Terminar auditoria da relação observacional, inclusive folhas opacas
  transitivas em capturas de Module/Closure, e a instrumentação passiva
  do ciclo real. Identidade de Arc isolada não substitui essa auditoria.
- Executar os gates públicos e internos de F, mutações e controles,
  rebaseline global, build/testes, lint estrito e linhagem, seguidos do
  veredito independente. Não reutilizar evidência do baseline como GREEN
  desta implementação.

O selo histórico, o RED independente e os recibos de tentativas anteriores
continuam preservados. Este documento não modifica seus resultados nem
autoriza ultrapassar os budgets ou mudar entradas protegidas.
