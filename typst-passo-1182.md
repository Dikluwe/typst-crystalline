# P1182 — auditar e classificar o corpus legado contra a bijeção L0

**Data:** 2026-08-25  
**Estado:** `EXECUTADO — CLASSIFICAÇÃO CONCLUÍDA; PARADO NO GATE`  
**Dependências:** ADR-0129 em vigor; P1181 fases 0–1 concluídas  
**Escopo:** auditoria documental, sem divisão de prompts, criação de Núcleos,
alteração de headers ou reparo de hashes  
**Classe ADR-0127:** medição/classificação; parar antes de qualquer mudança L0

## Objetivo

Construir o inventário semântico definitivo do corpus legado que viola:

```text
1 Prompt L0 materializável ↔ 1 consumer produtivo
```

Classificar individualmente as 24 colisões V15 e seus 114 consumers, reconciliar
os dois órfãos V7 e propor, sem materializar:

- o Prompt L0 proprietário de cada consumer;
- quais cláusulas permanecem específicas;
- quais claims são genuinamente compartilhadas;
- quais Núcleos Tekt seriam justificados;
- a ordem segura dos futuros lotes de saneamento.

Este passo produz diagnóstico e decisão. Não altera o corpus L0.

## Baseline medido antes da escrita

Medição em 2026-08-25T21:20:01-03:00:

```text
typst-crystalline HEAD: 00f402e875956304aa435f749a251f359287e2ba
linter source HEAD: b2a2826e540a556081476918f98cb85c5dfe21be
linter branch: codex/p0111-mutation-pilot
linter binary SHA-256:
eb7494979040e70feb6ac3b738c86979488b8c26927480126746aae2ff707c9d
```

A fonte do linter estava limpa no momento da proveniência. A árvore do produto
continha somente a adoção não commitada de P1181: `CLAUDE.md`,
`crystalline.toml`, ADR-0129, diagnóstico P1181 e o próprio passo P1181.
Índice vazio.

### Lint integral

`crystalline-lint .` terminou com exit 1, stdout de 6.042 linhas e stderr
vazio. SHA-256 do stdout:
`46f90eedf6215c8d3361b5ab73d27e19e6ec96cf0371f6e01c3910166aeb4cd1`.

| Regra | Achados |
|---|---:|
| V5 | 421 |
| V7 | 2 |
| V15 | 24 |
| V16 | 210 |
| V17 | 36 |
| V18 | 2 |
| V19 | 349 |
| V20 | 600 |
| V21 | 24 |
| V26 | 0 |

V16–V21 são lentes independentes e não entram na decisão de ownership deste
passo. V5 não pode ser reparada enquanto V15 estiver aberta.

### Lente focal

`crystalline-lint --checks v15,v26 --fail-on warning .` terminou com exit 1,
72 linhas, stderr vazio e SHA-256
`f1feeb0620caa94210e3e6b3aade45f429d5d04729d5570dacde28e98134b70a`.

Resultado: 24 prompts em colisão, 114 ocorrências de consumers e zero V26.
O corpus ainda não contém Núcleos Tekt.

## Restrições

- não acessar/listar `00_nucleo/context/` ou `00_nucleo/materialization/`;
- não editar `00_nucleo/prompts/`, L1–L4, headers ou `crystalline.toml`;
- não criar `00_nucleo/prompts/_nuclei/` neste passo;
- não executar `--fix-hashes` sem `--dry-run`;
- não decidir por nome de arquivo ou proximidade de diretório;
- não converter automaticamente um prompt compartilhado em Núcleo;
- não escolher um consumer representativo;
- não copiar integralmente o mesmo prompt para vários owners;
- não misturar correção funcional ou paridade Typst;
- preservar a working tree P1181 e manter o índice vazio.

## 1. Selar novamente a proveniência

Antes da auditoria, repetir:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
git diff --cached --stat
sha256sum /home/dikluwe/.cargo/bin/crystalline-lint
git -C /repos/Antigravity/tekt-linter rev-parse HEAD
git -C /repos/Antigravity/tekt-linter status --short
```

Se o SHA-256 do binário ou HEAD da fonte divergir do baseline acima, rodar os
dois lints novamente e usar nova proveniência. Não combinar contagens de
binários diferentes.

Executar o lint focal duas vezes e exigir output byte-idêntico. Guardar logs
em `/tmp`, nunca em paths tracked.

## 2. Gerar o manifesto mecânico

Criar:

```text
00_nucleo/diagnosticos/typst-p1182-inventario-bijecao-l0.tsv
```

Uma linha por consumer V15, com colunas:

```text
prompt_atual
consumer
consumer_count
layer
prompt_hash
consumer_hash
metadata_count
prompt_role
consumer_role
shared_claim_candidates
class
proposed_owner_prompt
proposed_nucleus
decision
evidence
```

Regras de fechamento:

- exatamente 114 linhas de consumers, salvo nova medição justificada;
- 24 prompts distintos, salvo nova medição justificada;
- nenhum consumer duplicado dentro da mesma relação;
- todo path existe e é arquivo regular não symlink;
- toda decisão cita `file:line` do prompt e do consumer;
- contagens fecham com o lint focal da mesma proveniência.

## 3. Auditar os 24 grupos

Ordem inicial, do menor risco para o maior:

| Consumers | Prompt atual |
|---:|---|
| 2 | `compiler/layout_references.md` |
| 2 | `compiler/math/layout/_comum.md` |
| 2 | `compiler/stdlib/context.md` |
| 2 | `compiler/stdlib/foundations.md` |
| 2 | `compiler/stdlib/state.md` |
| 2 | `compiler/stdlib_audit_methodology.md` |
| 2 | `entities/page_canvas.md` |
| 2 | `entities/page_running.md` |
| 2 | `infra.md` |
| 2 | `infra/export/builder.md` |
| 2 | `infra/package_downloader.md` |
| 2 | `infra/shaper.md` |
| 2 | `shell/cli.md` |
| 2 | `shell/info.md` |
| 2 | `testing/math_oracle.md` |
| 3 | `wiring.md` |
| 4 | `compiler/lexer/mod.md` |
| 4 | `compiler/stdlib/primitives-constructors.md` |
| 5 | `compiler/lang.md` |
| 5 | `entities/f_fronteira_e1.md` |
| 7 | `compiler/parse.md` |
| 10 | `compiler/eval.md` |
| 17 | `compiler/layout.md` |
| 29 | `compiler/atomizacao_elementos.md` |

Para cada grupo:

1. ler integralmente o prompt vigente;
2. confirmar seu hash e metadata canônica;
3. ler todos os headers e a responsabilidade observável de cada consumer;
4. identificar cláusulas específicas com `file:line`;
5. identificar claims potencialmente comuns com `file:line`;
6. procurar L0 proprietário já existente antes de propor arquivo novo;
7. medir imports e papel arquitetural apenas quando necessários à decisão;
8. escrever medição antes da classificação;
9. declarar inferências e o que as refutaria;
10. registrar todos os destinos no TSV.

Não ler código para inferir intenção quando o L0 já decide a questão. Código
pode demonstrar responsabilidade observável, mas não transforma comportamento
acidental em contrato.

## 4. Classes permitidas

Cada prompt compartilhado recebe exatamente uma classe principal.

### A — contratos específicos misturados

O Markdown contém contratos pertencentes a consumers diferentes.

Destino proposto: um Prompt L0 por consumer, distribuindo apenas suas cláusulas
específicas. Núcleo somente se restarem claims comuns independentes de owner.

### B — owner real com claims compartilhadas

Um consumer é owner legítimo do prompt atual, mas outros dependem de um
subconjunto normativo comum.

Destino proposto: preservar o owner, criar prompts próprios para os outros e
extrair apenas o subconjunto comum para Núcleo Tekt.

### C — documento transversal, sem owner produtivo

O documento inteiro governa vários prompts e não materializa diretamente um
consumer.

Destino proposto: Núcleo Tekt para as claims normativas e prompts proprietários
individuais para todos os consumers. O Markdown antigo só pode ser removido
depois de zero referências e prova de preservação.

### D — associação documental incorreta

O consumer aponta para metodologia, agregador, teste ou contrato de outra
unidade.

Destino proposto: localizar/criar o Prompt L0 correto. O documento antigo vai
para sua categoria verdadeira; não vira Núcleo automaticamente.

### E — bloqueio semântico

O material disponível não permite separar ownership de obrigação comum sem
decisão nova.

Destino: registrar alternativas e parar o grupo. Não preencher o manifesto com
uma escolha conveniente.

## 5. Auditar os dois V7

Classificar separadamente:

```text
00_nucleo/prompts/_convencoes.md
00_nucleo/prompts/shell/custom-ca-cert.md
```

Para cada um decidir, com evidência:

- Prompt L0 materializável com consumer ausente;
- exceção legítima não materializável;
- documento transversal candidato a Núcleo;
- documento em categoria/diretório incorreto;
- obsoleto, sem autoridade vigente.

Não remover, excepcionar ou converter neste passo. V7 pertence ao mesmo
fechamento da bijeção porque representa o lado Prompt sem consumer.

## 6. Desenhar Núcleos candidatos sem criar arquivos

Para cada candidato, registrar no diagnóstico:

```text
id proposto
path lógico proposto
prompts consumidores propostos
claims atômicas propostas
modalidade de cada claim
evidência de compartilhamento
dependências propostas
risco de ciclo
refutador
```

Um Núcleo candidato precisa de pelo menos dois prompts consumidores propostos.
Excluir ownership, algoritmos específicos, inventário de consumers, métricas,
histórico, decisões ADR e conteúdo meramente explicativo.

## 7. Produzir diagnóstico e plano de lotes

Criar:

```text
00_nucleo/diagnosticos/typst-p1182-auditoria-bijecao-l0.md
```

O diagnóstico deve conter:

- proveniência completa;
- fechamento 24/114 e reconciliação dos dois V7;
- tabela dos grupos e classes A–E;
- mapa consumer→prompt proprietário proposto;
- catálogo de Núcleos candidatos;
- prompts existentes reutilizáveis;
- casos bloqueados e decisões humanas necessárias;
- comparação explícita com P1179 (22/107 histórico);
- ordem dos futuros lotes, custo e dependências;
- confirmação de zero mutação fora dos dois entregáveis diagnósticos e do
  estado do passo.

Critérios para ordenar lotes futuros:

1. zero Núcleo e dois consumers com responsabilidades claramente distintas;
2. separação entidade/compilador ou contrato/infra;
3. build scripts e testes com owners próprios;
4. famílias pequenas com um Núcleo candidato;
5. parse/eval;
6. layout;
7. atomização por último.

## Gate final obrigatório

Depois de completar inventário e diagnóstico:

1. executar novamente V15/V26 apenas para provar que a auditoria não mutou o
   corpus — espera-se ainda 24 V15 e zero V26;
2. comparar logs com o baseline da mesma proveniência;
3. executar `git diff --check`;
4. provar índice vazio;
5. **PARAR** e apresentar a classificação ao humano.

Não criar Prompt L0, Núcleo, header ou hash neste passo. A classificação aprovada
legitimará passos posteriores por lote.

## Critérios de aceitação

- binário e fonte pinados no momento da auditoria;
- dois lints focais byte-idênticos;
- 24/24 colisões classificadas ou explicitamente bloqueadas;
- 114/114 consumers com destino proposto;
- 2/2 V7 classificados;
- toda decisão posterior à medição `file:line`;
- nenhum Núcleo artificial ou órfão proposto;
- todo Núcleo candidato possui ao menos dois prompts consumidores;
- nenhuma alteração em prompts, código, headers, configuração ou hashes;
- V15/V26 finais idênticos ao baseline;
- `git diff --check` limpo e índice vazio;
- gate humano respeitado antes da primeira materialização.

## Próximo passo condicionado

Após aprovação do diagnóstico P1182, escrever P1183 para materializar apenas o
primeiro lote pequeno e sem bloqueio semântico. Não iniciar por `eval.md`,
`layout.md` ou `atomizacao_elementos.md`.

## Resultado da execução

Executado em 2026-08-25 sobre HEAD
`00f402e875956304aa435f749a251f359287e2ba`, com binário SHA-256
`eb7494979040e70feb6ac3b738c86979488b8c26927480126746aae2ff707c9d`.
Duas passagens focais foram byte-idênticas: 24 V15, zero V26, 114 consumers.

O manifesto fechou 114/114 relações, 114 owners distintos propostos, 22 paths
de owner preserváveis e 92 owners novos. Classificação por grupo: A=22, B=1,
D=1, E=0. Os dois V7 foram classificados como documentos transversais em
categoria incorreta, candidatos a Núcleo após estabilização dos prompts
consumidores.

Entregáveis:

```text
00_nucleo/diagnosticos/typst-p1182-inventario-bijecao-l0.tsv
00_nucleo/diagnosticos/typst-p1182-auditoria-bijecao-l0.md
```

Nenhum Prompt L0, Núcleo, header, hash, configuração ou código produtivo foi
alterado. Execução parada para aprovação humana antes do primeiro lote.
