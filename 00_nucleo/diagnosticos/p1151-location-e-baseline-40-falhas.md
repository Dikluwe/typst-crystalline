# P1151 — diagnóstico de `content.location()` e das 40 falhas integrais

**Data:** 2026-08-24
**HEAD medido:** `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`
**Estado:** working tree não commitada; lote P1149/P1150 preservado
**Vanilla:** `a51e02804`

## 1. `location`

### Medição

Sonda executada nos dois binários ratificados com duas headings de corpo
`Same` e `query(heading)` dentro de `context`. Ambos devolveram:

```text
len                         2
type dos dois               content
location dos dois           location(..)
locations iguais            false
conteúdos iguais            true
repr/fields                 iguais e sem Location exposta
```

No cristalino, `native_query` conserva `loc` até
`compiler/stdlib/foundations/query.rs:55-64`, mas o descarta ao construir
`Value::Content(c.clone())`. `eval_content_method` em
`compiler/eval/bindings/field_access.rs:662-665` já não tem informação para
devolver. Procurar por igualdade é inválido: a sonda prova que dois conteúdos
iguais podem ter Locations distintas.

### Gate

A correção exige uma representação pública de `Value` que transporte o par
sem alterar o tipo de linguagem `content`. Isso muda contrato público Rust;
L0s foram atualizados e a implementação para no gate ADR-0127.

## 2. O que são as 40 falhas

A suite integral medida no fim de P1150 teve 5.182 aprovados e 40 falhas. Após
P1151, teve 5.183 aprovados e as mesmas 40 falhas. Elas
não foram causadas por P1150 e dividem-se em dois blocos: 30 testes de I/O com
fixtures/world incompatíveis com o caminho de leitura atual e 10 divergências
semânticas antigas.

| Grupo | Qt. | Testes | Causa observada |
|---|---:|---|---|
| imagem em eval | 1 | `eval_image_le_ficheiro_para_content` | leitura de fixture falha no world de teste |
| bibliografia em eval | 6 | P420 ×4, P421, P450 | paths CSL/BIB não são resolvidos pelo world de teste atual |
| CSV em eval | 3 | P787 ×3 | fixture/path chega ao mesmo bloqueio de leitura |
| loading/read | 9 | JSON; P823 CBOR; P824 ×4; `read_*` ×3 | testes unitários esperam bytes/texto, mas o world usado não resolve o ficheiro |
| plugin | 1 | `plugin_caminho_inexistente_erro_de_leitura` | diagnóstico esperado diverge após o bloqueio de acesso |
| bibliografia stdlib | 4 | `native_bibliography_*` ×4 | fixtures BIB/YAML/CSL não chegam ao parser |
| imagem stdlib | 6 | `native_image_*` ×5 e P835 | fixtures inseridas no `NullWorld` não coincidem com a resolução atual de `FileId`; erro observado: `cannot access file system from here` |
| `int(float)` | 1 | `native_int_float_retorna_err` | teste espera erro, implementação atual aceita/converte `3.7` |
| radial focal | 3 | P269 ×3 | named args usam `focal_center`/`focal_radius`; parser vigente rejeita-os apesar de os listar, indicando deriva underscore ↔ nome público |
| gradient relative | 6 | P273 ×6 | testes fornecem apenas um stop; validação atual exige pelo menos dois antes de verificar `relative` |
| **Total** | **40** |  | **30 I/O + 10 semânticas** |

## 3. Classificação operacional

As 40 falhas são baseline nominal, mas não devem ser tratadas como sucesso:

- as 30 de I/O parecem principalmente dívida do harness/fixtures após a
  fronteira World/FileId, embora cada grupo ainda precise de RED isolado para
  separar fixture inválida de bug real de resolução;
- as 3 P269 são forte evidência de bug real de normalização de named args;
- as 6 P273 misturam uma expectativa antiga de constructor com a validação
  atual de quantidade mínima de stops; devem ser re-medidas no vanilla antes
  de decidir se muda teste ou implementação;
- `native_int_float_retorna_err` também exige sonda vanilla: o nome do teste
  não é prova do contrato vigente.

Nenhuma dessas falhas é pré-requisito para transportar Location, mas uma suite
integral verde exige passos próprios: primeiro harness I/O (30), depois P269,
P273 e `int(float)` com sondas ratificadas.

## 4. Atualização P1152

Em `2026-08-25T07:03:34-03:00`, no mesmo HEAD
`1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc` e com a working tree não
commitada preservada, P1152 confirmou que as 30 falhas de I/O eram dívida dos
mocks: eles implementavam `read_bytes`, mas não `resolve_path` + `read_path`.

Após migrar cinco worlds test-only para a fronteira P1141, a suite passou de
5.183 aprovados/40 falhas para **5.213 aprovados/10 falhas**. Não houve mudança
de código de produção ou contrato público. As 10 falhas restantes coincidem
nominalmente com o bloco semântico inventariado acima: 1 `int(float)`, 3 P269
e 6 P273. Elas constituem o handoff fechado para P1153.
